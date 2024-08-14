pub mod builder;
pub mod target;
pub mod wasm;

use std::{fs, path::PathBuf, process::Child, sync::Mutex};

use gen_converter::model::Model;
use gen_utils::{
    common::RustDependence,
    compiler::{CompilerImpl, ModelNodeImpl},
    error::Errors,
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
        let main_rs = Makepad::create_main_rs(&self.entry, self.origin_path.as_path());
        let widget_tree =
            Makepad::create_widget_tree(self.origin_path.as_path(), self.root.as_ref());
        let app_main =
            Makepad::create_app_main(&self.entry, self.origin_path.as_path(), &widget_tree);
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
        let _ = self.target.as_mut().unwrap().compile(&self.entry, self.origin_path.as_path(), gen_files);
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
