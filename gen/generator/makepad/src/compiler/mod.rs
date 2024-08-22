pub mod builder;
pub mod target;
pub mod wasm;

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    process::Child,
    sync::Mutex,
};

use gen_converter::model::Model;
use gen_utils::{
    common::{
        string::{format_live_design, format_live_design_tk, pub_mod_non_snake_case},
        RustDependence,
    },
    compiler::{CompilerImpl, ModelNodeImpl},
    error::{Errors, FsError},
    wasm::WasmImpl,
};
use lazy_static::lazy_static;
use target::Makepad;

use toml_edit::DocumentMut;
use wasm::Wasm;

use crate::widget::model::safe_widget::SafeWidget;

lazy_static! {
    /// built-in widgets, if a widget's role is for or if_else, it should be in this list
    ///
    /// after all, depend on this list, generate a `auto/lib.rs` and builtin widgets rs files, and add them to the `src_gen` project
    /// then insert into live_registers and compiler tree
    ///
    /// in gen: `<button :for="item in list" :text="item"></button>`
    ///
    /// in makepad: generate a for loop widget
    /// ```rust
    /// live_design! {
    ///    LoopButton_${sha_value} = {{LoopButton_${sha_value}}}{
    ///         item: <Button>{}
    ///    }
    /// }
    ///
    /// pub struct LoopButton_${sha_value} {
    ///     #[live]
    ///     item: Option<LivePtr>,
    ///     #[rust]
    ///     children: ComponentMap<LiveId, ButtonRef>,
    ///     // ...
    /// }
    /// ```
    pub static ref AUTO_BUILTIN_WIDGETS: Mutex<Vec<SafeWidget>> = Mutex::new(Vec::new());
    pub static ref ROOT_COMPILED: bool = false;
    /// virtual map is used to store the k-v of the virtual widget and its' source widget
    pub static ref VIRTUAL_MAP: Mutex<Option<VMap>> = Mutex::new(None);
}

/// # Makepad Compiler
#[derive(Debug)]
pub struct Compiler {
    /// origin path is the project path
    pub origin_path: PathBuf,
    pub compiled_path: PathBuf,
    /// entry file name, default is app
    pub entry: String,
    /// root path of the project
    pub root: Option<PathBuf>,
    /// rust dependencies in Cargo.toml
    /// it depends on the target
    /// - makepad: makepad-widgets
    /// > **you can add more other dependencies which you need**
    pub dependencies: Vec<RustDependence>,
    /// use wasm to run ?
    /// makepad wasm
    pub wasm: Option<Wasm>,
    /// child wasm process
    pub wasm_process: Option<Child>,
    /// makepad target
    pub target: Option<Makepad>,
}

impl CompilerImpl for Compiler {
    fn execute_auxiliaries(&mut self, executor: gen_utils::compiler::Executor) -> () {
        match self.fresh_wasm() {
            Ok(success) => {
                if success {
                    executor.success_fn("")
                } else {
                    executor.ignore_fn()
                }
            }
            Err(e) => executor.fail_fn(e),
        }
    }
    fn exist_or_create(&self) -> () {
        let compiled_dir = self.compiled_path.clone();
        // read the origin project's Cargo.toml file and move the [dependencies] to the src_gen project except gen's dependencies
        let origin_toml_path = &self.origin_path.join("Cargo.toml");
        if !origin_toml_path.exists() {
            panic!("Cargo.toml not found in the origin project");
        }
        let origin_toml_content = fs::read_to_string(origin_toml_path.as_path())
            .expect("failed to read origin project's Cargo.toml");
        let origin_toml = origin_toml_content
            .parse::<DocumentMut>()
            .expect("Failed to parse Cargo.toml");
        // get the dependencies table and remove the gen's dependencies
        let mut origin_dependencies = origin_toml["dependencies"]
            .as_table()
            .expect("dependencies not found in Cargo.toml")
            .clone();
        origin_dependencies.retain(|k, _| !k.starts_with("gen"));
        // write the dependencies to the src_gen project's Cargo.toml file
        let compiled_toml_path = &compiled_dir.join("Cargo.toml");
        // find the src_gen project's Cargo.toml file's [dependencies] table and replace the origin project's dependencies
        let compiled_toml_content = fs::read_to_string(compiled_toml_path.as_path())
            .expect("failed to read src_gen project's Cargo.toml");
        let mut compiled_toml = compiled_toml_content
            .parse::<DocumentMut>()
            .expect("Failed to parse Cargo.toml");
        let compiled_dependencies = compiled_toml["dependencies"]
            .as_table_mut()
            .expect("dependencies not found in Cargo.toml");

        // add dependencies to the src_gen project from compiler dependencies
        for dep in self.dependencies.iter() {
            let (name, value) = dep.to_table_value();
            origin_dependencies[name.as_str()] = value;
        }

        let _ = std::mem::replace(compiled_dependencies, origin_dependencies);

        // compiled_dependencies.extend(origin_dependencies.iter());
        // write back
        fs::write(compiled_toml_path.as_path(), compiled_toml.to_string())
            .expect("failed to write src_gen project's Cargo.toml");
    }
    fn before_compile(&mut self) -> () {
        {
            let mut vmap = VIRTUAL_MAP.lock().unwrap();
            vmap.replace(VMap::new(self.compiled_path.as_path(), &self.entry));
            let _ = vmap.as_ref().unwrap().before_compile();
        }
        let main_rs = Makepad::create_main_rs(&self.entry, self.origin_path.as_path());
        let widget_tree =
            Makepad::create_widget_tree(self.origin_path.as_path(), self.root.as_ref());
        // let app_main =
        //     Makepad::create_app_main(&self.entry, self.origin_path.as_path(), &widget_tree);
        let app_main = Makepad::create_default_app_main(&self.entry, self.origin_path.as_path());
        self.target.replace(Makepad {
            app_main,
            tree: Some(widget_tree),
            main_rs,
        });
    }

    /// init makepad project
    /// - create main.rs
    /// - create app entry rs file (eg: app.rs)
    /// - create lib.rs (depend on root)
    fn compile(&mut self, gen_files: Option<&Vec<&PathBuf>>) -> () {
        let _ = self.target.as_mut().unwrap().compile(
            &self.entry,
            self.origin_path.as_path(),
            gen_files,
        );
    }

    fn insert(&mut self, node: Box<dyn std::any::Any>) -> () {
        let node = node.downcast::<Model>().unwrap();

        let _ = self.target.as_mut().unwrap().insert(*node);
    }

    fn get(&self, key: &gen_utils::common::Source) -> Option<Box<dyn ModelNodeImpl>> {
        self.target
            .as_ref()
            .unwrap()
            .get(key)
            .map(|node| Box::new(node) as Box<dyn ModelNodeImpl>)
    }
}

impl Compiler {
    /// set wasm
    pub fn wasm<W>(&mut self, wasm: Box<W>) -> &mut Self
    where
        W: WasmImpl,
    {
        if let Some(wasm) = wasm.as_any().downcast_ref::<Wasm>() {
            self.wasm.replace(wasm.clone());
        };
        self
    }
    /// ## Fresh Wasm
    /// fresh wasm when the wasm file is modified
    /// - if process work successfully, return `Ok(true)`
    /// - else return `Err(msg)`
    /// - if wasm is not enabled, return `Ok(false)`
    pub fn fresh_wasm(&mut self) -> Result<bool, Errors> {
        if self.wasm.is_some() {
            // close last wasm process if exist
            if let Some(process) = self.wasm_process.as_mut() {
                let _ = process.kill();
            }
            let mut super_workspace_path = self.origin_path.clone();
            super_workspace_path.pop();
            match self
                .wasm
                .as_ref()
                .unwrap()
                .run(super_workspace_path.as_path())
            {
                Ok(cmd) => {
                    self.wasm_process.replace(cmd);
                    return Ok(true);
                }
                Err(e) => return Err(e),
            }
        }
        Ok(false)
    }
    /// only wasm is Some, this function can work
    ///
    /// then check makepad wasm
    /// - return `Ok(true)` if makepad wasm is installed
    /// - return `Ok(false)` if makepad wasm not need to check
    /// - return `Err` if makepad wasm is not installed
    pub fn check_wasm(&self) -> Result<bool, Errors> {
        self.wasm.as_ref().unwrap().check_wasm()
    }
}

#[derive(Debug, Default)]
pub struct VMap {
    pub auto_lib_path: PathBuf,
    pub app_main_path: PathBuf,
    /// new virtual map, need to compile
    pub new: HashMap<PathBuf, Vec<String>>,
    /// old virtual map, need to clear
    pub old: HashMap<PathBuf, Vec<String>>,
}

impl VMap {
    pub fn new<P>(path: P, entry: &str) -> Self
    where
        P: AsRef<Path>,
    {
        let auto_lib_path = path.as_ref().join("src").join("auto");
        Self {
            auto_lib_path,
            app_main_path: path.as_ref().join("src").join(format!("{}.rs", entry)),
            new: HashMap::new(),
            old: HashMap::new(),
        }
    }
    pub fn before_compile(&self) -> Result<(), Errors> {
        // judget if src/auto dir exists, if exists, remove all files in it, if not exists, create it
        let auto_dir = self.auto_lib_path.as_path();
        let auto_file = auto_dir.join("mod.rs");
        if auto_dir.exists() {
            std::fs::remove_dir_all(auto_dir).map_err(|e| {
                Errors::FsError(FsError::Delete {
                    path: auto_dir.to_path_buf(),
                    reason: e.to_string(),
                })
            })?;
        }
        // create auto dir path --------------------------------------------------
        let _ = std::fs::create_dir(auto_dir).map_err(|e| {
            Errors::FsError(FsError::Create {
                path: auto_dir.to_path_buf(),
                reason: e.to_string(),
            })
        });
        // crate auto lib path ---------------------------------------------------
        match gen_utils::common::fs::create_file(auto_file.as_path()) {
            Ok(_) => Ok(()),
            Err(e) => Err(Errors::FsError(FsError::Create {
                path: auto_file,
                reason: e.to_string(),
            })),
        }
    }
    pub fn get_or_insert<P>(&mut self, k: P, v: Vec<String>)
    where
        P: AsRef<Path>,
    {
        if let Some(values) = self.new.get(k.as_ref()) {
            // if not empty remove current value to old
            self.old.insert(k.as_ref().to_path_buf(), values.clone());
        } else {
            self.new.insert(k.as_ref().to_path_buf(), vec![]);
        }

        let _ = self.new.insert(k.as_ref().to_path_buf(), v);
    }
    pub fn clear(&mut self) -> () {
        self.old.clear();
        self.new.clear();
    }
    /// > [!NOTE] you should call vmap.old.clear() after `update_app_main_registers` method
    /// > because the `update_app_main_registers` method will use the old data
    /// > so this `clear_old` will not clear the old data
    pub fn clear_old(&mut self) -> () {
        if self.old.is_empty() {
            return;
        }

        // loop old and remove from auto dir
        let auto_mod = self.auto_lib_path.join("mod.rs");
        let mut content = gen_utils::common::fs::read(auto_mod.as_path())
            .unwrap()
            .replace("\r\n", " ");
        let mut app_main = gen_utils::common::fs::read(self.app_main_path.as_path()).unwrap();
        self.old.iter().for_each(|(_, values)| {
            for (index, v) in values.iter().enumerate() {
                // remove from auto dir --------------------------------------------------------------------
                let p = self.auto_lib_path.join(format!("{}.rs", v));
                // remove from auto/mod.rs -----------------------------------------------------------------
                content = content.replace(
                    &pub_mod_non_snake_case(p.file_stem().unwrap().to_str().unwrap()),
                    "",
                );

                let _ = gen_utils::common::fs::delete(p).unwrap();
                // remove from app main --------------------------------------------------------------------

                let live_design = format!("auto :: {}", v);
                app_main = app_main.replace(
                    format_live_design_tk(&live_design).as_str(),
                    if index == values.len() - 1 {
                        "$new_auto_live_registers"
                    } else {
                        ""
                    },
                );
            }
        });
        let _ = gen_utils::common::fs::write(auto_mod.as_path(), &content).unwrap();
        let _ = gen_utils::common::fs::write(self.app_main_path.as_path(), &app_main).unwrap();
    }

    pub fn live_registers(&self) -> Option<Vec<String>> {
        Some(self.new.iter().fold(Vec::new(), |mut acc, (_, v)| {
            acc.extend(
                v.iter()
                    .map(|x| format_live_design(&format!("auto::{}", x))),
            );
            acc
        }))
    }

    pub fn update_app_main_registers(&self) -> () {
        if self.old.is_empty() {
            return;
        }
        let mut app_main = gen_utils::common::fs::read(self.app_main_path.as_path()).unwrap();
        app_main = app_main.replace(
            "$new_auto_live_registers",
            &self.old.iter().fold(String::new(), |mut acc, (k, _)| {
                acc.push_str(
                    &self
                        .new
                        .get(k)
                        .unwrap()
                        .iter()
                        .fold(String::new(), |mut bcc, v| {
                            bcc.push_str(&format_live_design_tk(format!("auto :: {}", v).as_str()));
                            bcc
                        }),
                );
                acc
            }),
        );

        let _ = gen_utils::common::fs::write(self.app_main_path.as_path(), &app_main).unwrap();
    }
}
