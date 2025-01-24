use std::path::PathBuf;

use gen_utils::compiler::Builder;

use super::{api_version::ApiVersion, ArkCompiler};

/// # Builder for ArkCompiler
pub struct CompilerBuilder{
    /// ArkTS API Version, which is used to check the environment
    pub api_version: ApiVersion,
    /// check the environment and others ... if true, then can compile (default false)
    pub check: bool,
}

impl CompilerBuilder {
    /// 
    pub fn version(mut self, v: ApiVersion) -> Self{
        self.api_version = v;
        self
    }
    pub fn check(mut self) -> Self{
        self.check = true;
        self
    }
}


// impl Builder for CompilerBuilder {
//     type Target = ArkCompiler;

//     fn build(self) -> Self::Target {
//         todo!();
//         // let origin_path = std::env::current_dir().unwrap();
//         // let config = 


//         // ArkCompiler{
//         //     api_version: self.api_version,
//         //     check: self.check,
//         //     origin_path: self.origin_path,
//         //     compiled_path: self.compiled_path,
//         //     config,
//         // }
//     }
// }