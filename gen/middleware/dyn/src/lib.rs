use std::path::{Path, PathBuf};

mod processor;
use gen_utils::common::fs;
pub use processor::DynProcessor;

/// you can generate a code by using `extern_c_fn`
///
/// #[repr(C)]
/// pub struct MacroContext {
///     pub tokens: String,
///     pub path: String,
///     pub result: String,
/// }
pub fn compile_dyn_lib(
    structure: &str,
    code: &str,
    output_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    // 创建临时源文件
    let source_path = output_path.with_extension("rs");

    fs::create_file(&source_path)?;

    // 写入必要的头部代码
    let full_code = format!(
        r#"
#![crate_type="dylib"]
{}
        
#[no_mangle]
{}
        "#,
        structure, code
    );

    // file.write_all(full_code.as_bytes())?;
    fs::write(source_path.as_path(), &full_code)?;

    // 编译代码
    let status = std::process::Command::new("rustc")
        .arg(&source_path)
        .arg("--crate-type=dylib")
        .arg("-o")
        .arg(output_path)
        .status()?;

    if !status.success() {
        return Err("Compilation failed".into());
    }

    Ok(())
}

/// `pub extern "C" fn process_macro(context: &mut MacroContext)`
pub fn extern_c_fn(param: &str, code: &str) -> String {
    format!(
        r#"fn process_macro({}) -> bool {{
            {}
            return false;
        }}"#,
        param, code
    )
}

pub fn dyn_lib_path(prefix: &Path, filename: &str) -> PathBuf {
    prefix.join(PathBuf::from(format!(
        ".plugins/tmp/{}.{}",
        filename,
        std::env::consts::DLL_EXTENSION
    )))
}
