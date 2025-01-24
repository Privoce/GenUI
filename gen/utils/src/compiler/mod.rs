mod builder;
mod check;
mod config;
mod execute;
mod model_node;
mod project;
mod rs;
mod underlayer;
mod version;

use std::{collections::HashMap, path::PathBuf, process::exit};

pub use builder::*;
pub use check::*;
pub use config::*;
pub use execute::*;
pub use model_node::*;
pub use project::*;
pub use rs::*;
pub use underlayer::*;
pub use version::*;

use crate::error::Error;

/// # Compiler Impl
/// each compiler should implement this trait
/// ```txt
/// ┌────────────┐                     
/// │  Compiler  │                     
/// └────┬───────┘                     
///      │       ─────────────────────────────┐                      
///      ▼                                    │
///     init                                  │
///      │                                    │
///      │       before_compile ───────┐      │
///      ▼                             │      │
///     execute  ───► compile ─────► update  run
///      │                             │      │
///      │       after_compile ────────┘      │
///      ▼                                    │
///     exit     ─────────────────────────────┘                      
///                                  
/// ```
pub trait CompilerImpl {
    /// ## execute auxiliaries
    /// execute auxiliaries for the compiler, such as:
    /// - fresh wasm
    fn execute_auxiliaries(&mut self, executor: Executor) -> ();
    /// ## init compiled project
    /// check the compiled project path is exist or not
    /// - if exist, you can do some other things, such as check the config files, environment, etc.
    /// - if not exist, you can create the compiled project path
    /// ### more details
    /// see `generator/makepad/src/compiler/mod.rs` to know what we do in this function
    fn init(&mut self) -> Result<(), Error>;
    fn send_plugins(&mut self) -> Result<(), Error>;
    fn recv_plugins(&mut self, plugins: Option<&HashMap<String, PathBuf>>) -> Result<(), Error>;
    /// ## do something before compile
    fn before_compile(&mut self) -> Result<(), Error>;
    /// ## do something after compile
    fn after_compile(&mut self) -> Result<(), Error>;
    /// ## compile
    /// compile the target file
    fn compile(&mut self, path: PathBuf) -> Result<(), Error>;

    fn remove(&mut self, path: PathBuf) -> Result<Option<Vec<PathBuf>>, Error>;
    /// ## update after do compile
    fn update(&mut self) -> Result<(), Error>;
    // /// ## insert node into compiler tree
    // fn insert(&mut self, node: Box<dyn Any>) -> ();
    // /// ## get node from compiler tree
    // fn get(&self, key: &Source) -> Option<Box<dyn ModelNodeImpl>>;

    /// ## execute compiler
    fn execute(&mut self) -> Result<(), Error> {
        // [compile] ------------------------------------------
        self.compile(PathBuf::new())?;
        // [update] -------------------------------------------
        self.update()
    }

    /// ## execute lifetime
    fn execute_lifetime(&mut self) -> Result<(), Error> {
        // [init] ---------------------------------------------
        let _ = self.init()?;
        // [before_compile] -----------------------------------
        let _ = self.before_compile()?;
        // [execute] ------------------------------------------
        let _ = self.execute()?;
        // [after_compile] ------------------------------------
        let _ = self.after_compile()?;
        Ok(())
    }

    /// ## Compiler run
    /// run the compiler
    fn run(&mut self) -> () {
        // [state] -------------------------------------------
        let state: CompilerResult = self.execute_lifetime().into();
        // [err or exit] -------------------------------------
        state.err_or_exit(&mut |e| {
            eprintln!("{}", e);
            self.exit();
        });
    }

    /// ## Exit Compiler
    /// you can do some clean work in this function
    fn exit(&mut self) -> () {
        exit(1);
    }
}

pub type CompilerResult = ResultExecutor;

pub struct ResultExecutor(Result<(), Error>);

impl ResultExecutor {
    pub fn err_or_exit<Exit>(&self, f: &mut Exit) -> ()
    where
        Exit: FnMut(&Error) -> (),
    {
        match &self.0 {
            Ok(_) => (),
            Err(e) => f(e),
        }
    }
}

impl From<Result<(), Error>> for ResultExecutor {
    fn from(result: Result<(), Error>) -> Self {
        Self(result)
    }
}
