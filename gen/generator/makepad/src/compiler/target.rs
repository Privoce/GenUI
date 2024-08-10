use std::{
    collections::HashSet,
    io::Write,
    path::{Path, PathBuf},
};

use gen_converter::model::{file_data, Model};
use gen_parser::ParseTarget;
use gen_utils::common::{
    fs::{self, create_file},
    token_tree_ident, Source,
};
use quote::quote;

use crate::{
    model::{ModelNode, ModelTree, RsFile},
    widget::{
        model::{
            app_main::AppMain, auto_builtin_widgets::AutoBuiltinCompile, widget::Widget,
            ToLiveDesign,
        },
        utils::imports_to_live_registers,
    },
    ToToken,
};

use super::AUTO_BUILTIN_WIDGETS;

/// # Makepad Core
/// Makepad is a core struct to handle makepad project
#[derive(Debug)]
pub struct Makepad {
    /// project app main entry file not the main.rs
    pub app_main: AppMain,
    /// project widget tree
    pub tree: Option<ModelTree>,
    /// main.rs file
    pub main_rs: RsFile,
}

impl Makepad {
    /// get node from tree
    pub fn get(&self, key: &Source) -> Option<ModelNode> {
        match self.tree.as_ref() {
            Some(tree) => tree.get(key),
            None => None,
        }
    }
    pub fn create_widget_tree<P>(path: P, root: Option<&PathBuf>) -> ModelTree
    where
        P: AsRef<Path>,
    {
        // match root {
        //     Some(root) => {
        //         let gen_model: Widget =
        //             gen_converter::model::Model::new(root, &path.as_ref().to_path_buf(), false)
        //                 .unwrap()
        //                 .into();
        //         ModelTree::new(gen_model.into())
        //     }
        //     None => ModelTree::default_root(),
        // }

        let mut widget = Widget::default_ui_root();

        widget.source.replace((root.unwrap(), path.as_ref()).into());
        ModelTree {
            node: widget.into(),
            children: None,
        }
    }
    pub fn create_app_main<P>(entry: &str, path: P, widget_tree: &ModelTree) -> AppMain
    where
        P: AsRef<Path>,
    {
        let (ui_root, root_widget) = widget_tree.super_ui_root();
        // let live_register = widget_tree.to_live_register();
        let imports = widget_tree.to_imports();
        let app_path = path.as_ref().join(format!("{}.gen", entry).as_str());
        let source = Source::from((app_path.as_path(), path.as_ref()));

        let mut app = AppMain::new(&source);
        // other will be handle after widget tree add method
        // app.set_root_ref(ui_root)
        //     .set_root_ref_ptr(&root_widget)
        //     .set_live_register(live_register);
        app.set_root_ref(ui_root)
            .set_root_ref_ptr(&root_widget)
            .set_imports(imports);
        // dbg!(&app);
        app
    }
    /// makepad main rs is easy, which just need to use app_main fn to run app
    pub fn create_main_rs<P>(entry: &str, path: P) -> RsFile
    where
        P: AsRef<Path>,
    {
        let main_path = path.as_ref().join("src").join("main.rs");
        let entry = token_tree_ident(entry);
        let project_name = quote! {src_gen};
        // let mut main_file = create_file(main_path.as_path());
        let content = quote! {
            fn main(){
                #project_name::#entry::app_main()
            }
        };
        RsFile::new((main_path, path).into(), content)
        // main_file
        //     .write_all(main_content.to_string().as_bytes())
        //     .unwrap();
    }
    pub fn compile_app_main(
        &mut self,
        gen_files: Option<&Vec<&PathBuf>>,
        other_registers: Option<Vec<String>>,
    ) -> () {
        // get imports from gen_files(widget tree just to handle compiled file, if file is in cache, it will not be compiled)
        // get file path and use ParseTarget to compile and get script part
        if let Some(files) = gen_files {
            let mut live_registers = HashSet::new();
            for file in files {
                if let Ok(content) = file_data(file.as_path()) {
                    let target = ParseTarget::try_from(content.as_str()).unwrap();
                    let target_imports = target.has_script_then_imports();
                    if let Some(target_imports) = imports_to_live_registers(target_imports) {
                        live_registers.extend(target_imports.into_iter());
                    }
                }
            }

            if other_registers.is_some() {
                live_registers.extend(other_registers.unwrap().into_iter());
            }

            // add root gen as live register
            live_registers.insert(self.tree.as_ref().unwrap().root_live_register());
            // in widget imports are imports
            // but in here, imports are app main live register, so called set_live_register
            let content = self
                .app_main
                .set_live_registers(live_registers)
                .to_live_design()
                .to_token_stream()
                .to_string();
            let mut file = create_file(self.app_main.source.compiled_file.as_path()).unwrap();
            file.write_all(content.as_bytes()).unwrap();
        }
    }
    pub fn compile_lib_rs(&self, auto: bool) -> () {
        let lib_mods = self.tree.as_ref().unwrap().to_lib();
        let auto_mod = if auto {
            Some(quote! {
                pub mod auto;
            })
        } else {
            None
        };
        let content = quote! {
            pub use makepad_widgets;
            pub use makepad_widgets::makepad_draw;
            pub mod app;
            #lib_mods
            #auto_mod
        }
        .to_string();

        let mut lib_path = self.main_rs.source.compiled_file.clone();
        lib_path.pop();
        lib_path.push("lib.rs");
        let mut file = create_file(lib_path.as_path()).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }
    /// insert item to model tree, if item exists, replace it
    pub fn insert(&mut self, item: Model) -> () {
        let _ = self.tree.as_mut().unwrap().insert(item.into());
        // let live_register = self.tree.as_ref().unwrap().to_live_register();
        // dbg!(&live_register);
        // self.app_main.set_live_register(live_register);
        // dbg!(&self.app_main);
    }
    /// Makepad Compile
    /// - compile main.rs
    /// - compile app.rs
    /// - compile lib.rs
    /// - compile other widget.rs (which is in ModelTree, use ModelTree compile method to compile)
    pub fn compile(&mut self, gen_files: Option<&Vec<&PathBuf>>) {
        // compile main.rs
        self.main_rs.compile();
        // compile other widget.rs
        self.tree.as_ref().unwrap().compile();
        // compile auto widgets
        let auto_widgets = AUTO_BUILTIN_WIDGETS.lock().unwrap();
        let mut auto_flag = false;
        let auto_live_registers = if !auto_widgets.is_empty() {
            auto_flag = true;
            // before compile auto widgets, create auto dir
            let auto_path = self
                .main_rs
                .source
                .compiled_dir
                .as_path()
                .join("src")
                .join("auto")
                .join("mod.rs");
            let _ = auto_widgets.before_compile(self.main_rs.source.compiled_dir.as_path()).unwrap();
            let _ = fs::create_file(auto_path.as_path())
                .expect("create auto dir or auto mod.rs failed");
            auto_widgets.compile(auto_path.as_path())
        } else {
            None
        };
        // create app main and compile app.rs
        // get auto widgets live register
        self.compile_app_main(gen_files, auto_live_registers);
        // compile lib.rs
        self.compile_lib_rs(auto_flag);
    }
}
