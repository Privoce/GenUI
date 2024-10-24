//! # Gen Compiler
//! Gen Compiler is a tool to compile gen-ui project to target project.
//! ## Features
//! - [x] support Makepad
//! - [x] support ArkUI
//! - [x] gen cache
//! - [x] gen ignore
//! - [x] gen logger
//! - [x] gen watcher
//! - [ ] continuous construction (no panic when compiling | panic reload)
mod builder;
mod core;
mod utils;

use builder::CompilerBuilder as UnifiedBuilder;
pub use core::*;
use lazy_static::lazy_static;
use std::sync::Mutex;
use toml_edit::DocumentMut;
pub use utils::*;

pub type MakepadBuilder = makepad_gen_plugin::compiler::builder::CompilerBuilder;
pub use gen_utils::compiler::Builder;
use gen_utils::{compiler::CompilerImpl, error::Errors};

lazy_static! {
    static ref TARGET: Mutex<TargetCompiler> = Mutex::new(TargetCompiler::Makepad);
    static ref CONF: Mutex<Result<DocumentMut, Errors>> = Mutex::new(Err(Errors::ParseError(
        "gen.toml file not found!".to_string()
    )));
}

/// ## compiler app
/// create an app compiler and specify the target
/// ### Attention
/// you should write from project root path as relative path
/// ### Example No `gen.toml`
/// we can create a compiler without `gen.toml` file, but we need to specify the target and other configurations
/// 
/// compiler use `builder` pattern, so you can chain the method to build the compiler, 
/// and finally call `build` method to get the compiler.
/// ```rust
/// use gen_compiler::{app, Target, Builder};
///
/// fn main() {
///     let compiler = Target::makepad()
///         .entry("app")
///         .root("E:/Rust/try/makepad/Gen-UI/examples/gen_makepad_simple/ui/views/root.gen")
///         .add_dep("makepad-widgets")
///         .local("E:/Rust/try/makepad/makepad/rik/makepad/widgets")
///         .build()
///         .wasm() // do not use if you don't need wasm
///         .build()
///         .build();
///
///     // set app and specify target
///     let mut app = app(Some(Box::new(compiler))).build();
///
///     let _ = app.run();
/// }
///
/// ```
/// ---
/// ### Example With `gen.toml`
/// if you have a `gen.toml` file, you can create a compiler without specifying the target and other configurations
/// the `gen.toml` file should be in the project root path, such as:
/// ```text
/// hello
/// ├── src_gen
/// ├────── // ....
/// ├── ui
/// ├────── src
/// ├────── gen.toml
/// ```
/// #### gen.toml
/// ```toml
/// [compiler]
/// target = "makepad"
/// log_level = "info"
/// logo = true
///
/// [makepad]
/// entry = "app"
/// root = "E:/Rust/try/makepad/Gen-UI/examples/gen_makepad_simple/ui/views/root.gen"
/// [makepad.dependencies] 
/// makepad-widgets = { path = "E:/Rust/try/makepad/makepad/rik/makepad/widgets" }
/// ```
/// #### main.rs
/// gen compiler will read the `gen.toml` file and create, so you do not need to pass the compiler
/// 
/// If you pass the compiler, the compiler will be used instead of the `gen.toml` file
/// ```rust
/// use gen_compiler::{app, Builder};
///
/// fn main() {
///     let mut app = app(None).build();
///     let _ = app.run();
/// }
/// ```
pub fn app(compiler: Option<Box<dyn CompilerImpl>>) -> UnifiedBuilder {
    // [init conf] ---------------------------------------------------------------------------------
    if compiler.is_none(){
        let mut conf = CONF.lock().unwrap();
  
        match gen_conf_toml_no_exit(){
            Ok(doc) => {
                *conf = Ok(doc);
            },
            Err(e) => {
                error_and_exit(e.to_string().as_str());
            },
        }
    }

    // [init log service] --------------------------------------------------------------------------
    let _ = init_log();

    let compiler = if let Some(compiler) = compiler {
        compiler
    } else {
        if let Some(compiler) = Target::conf() {
            compiler
        } else {
            std::process::exit(1)
        }
    };

    UnifiedBuilder::new(compiler)
}

#[cfg(test)]
mod test_compiler {
    use gen_utils::compiler::Builder;
    use std::path::PathBuf;

    #[test]
    fn app_conf() {
        let _app = super::app(None);
    }

    #[test]
    fn app_build_test() {
        let compiler = super::Target::makepad()
            .entry("app")
            .root("E:/Rust/try/makepad/Gen-UI/examples/gosim_example/ui/views/root.gen")
            .add_dep("makepad-widgets")
            .local("E:/Rust/try/makepad/makepad/rik/makepad/widgets")
            .build()
            .wasm()
            .no_fresh()
            .port(4568)
            .build()
            .build();

        let _app = super::app(Some(Box::new(compiler)));
    }

    #[test]
    fn end_gen() {
        let path = PathBuf::from("src_gen/main.gen");
        assert_eq!(path.to_str().unwrap().ends_with(".gen"), true);
    }
}
