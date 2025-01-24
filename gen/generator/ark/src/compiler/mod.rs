pub mod api_version;
pub mod config;
pub mod tree;
pub mod builder;

use std::path::PathBuf;

use api_version::ApiVersion;
use config::Config;
use gen_utils::compiler::Checker;

/// # HarmonyOS ArkTS Compiler
/// Each compiler need to implement the `Compiler` trait.
#[derive(Debug, Default)]
pub struct ArkCompiler{
    /// ArkTS API Version, which is used to check the environment
    pub api_version: ApiVersion,
    /// check the environment and others ... if true, then can compile (default false)
    pub check: bool,
    /// the origin path of the project, origin path is the project path where you write Gen code
    pub origin_path: PathBuf,
    /// the path of the compiled project, in ark, the compiled project is point to `entry` directory
    /// which is the entry of the HarmonyOS project
    pub compiled_path: PathBuf,
    /// the config of the project, which will be used to generate the `oh-package.json5` file
    pub config: Config,
    


}

impl Checker for ArkCompiler {
    fn check(&self) -> Result<(), gen_utils::error::Errors> {
        if self.check{
            self.check_env().and_then(|_| self.check_other())
        }else{
            Ok(())
        }
    }
    fn check_env(&self) -> Result<(), gen_utils::error::Errors> {
        self.api_version.check()
    }
    fn check_other(&self) -> Result<(), gen_utils::error::Errors> {
        // check oh-package.json5 file -------------------------------
        todo!()
    }
}

#[cfg(test)]
mod test_ark{
    use super::*;
    #[test]
    fn test_check(){
        let ark = ArkCompiler::default();
    }
}