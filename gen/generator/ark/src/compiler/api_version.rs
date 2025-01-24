#[allow(unused_imports)]
use std::{default, process::Command, str::FromStr};

use gen_utils::{
    compiler::Version,
    error::{CompilerError, Errors},
};
use which::which;

const JDK_MISSING: &str = r#"❗Can not find JDK in your system, please install it!
    1. Download JDK from official website: https://www.oracle.com/java/technologies/javase-jdk17-downloads.html
    2. Install JDK and set JAVA_HOME environment variable;"#;
const COMMAND_LINE_TOOL_MISSING: &str = r#"❗Can not find command line tool in your system, please install it!:
You can download here: https://developer.huawei.com/consumer/cn/download/command-line-tools-for-hmos
Version: Command Line Tools for HarmonyOS NEXT Developer Beta1(5.0.3+)
SHA-256: a81aa868064ac1f143a8de9d4782ef60ae1d5d39fd614abaa4a729dcf97bd943"#;
/// # ArkTs API Version
/// ApiVersion provide the ability to check CommandLineTool through version
/// See [HarmonyOS Dev](https://developer.huawei.com/consumer/cn/doc/harmonyos-releases-V5/releasenotes-baseline-V5?catalogVersion=V5)
/// - **V12: HarmonyOS ArkTS API 12**
///     - Nodejs: version >= 14.19.1
///     - codelinter: version >= 5.0.0 (recommend 5.0.2)
///     - hstack: version >= 5.0.0
///     - ohpm: version >= 5.0.0 (recommend 5.0.2)
///     - hvigorw: exist (5.0.0)
#[derive(Debug, Clone, Copy, Default)]
pub enum ApiVersion {
    #[default]
    V12,
}

impl ApiVersion {
    /// ## Check API ToolChain Version
    pub fn check(&self) -> Result<(), Errors> {
        match self {
            ApiVersion::V12 => {
                //check Node.js version is 14.19.1<= version <= 17.0.0 -----------------------
                let nodejs_version = Self::nodejs_version()?;
                if !(Version::from_str("14.19.1")? <= nodejs_version) {
                    return Err(Errors::CompilerError(CompilerError::env_check(
                        "Node.js",
                        "Node.js version should be 14.19.1<= version",
                        Some(format!("Current version: {}", nodejs_version).as_str()),
                    )));
                }
                // check codelinter version >= 5.0.0 -----------------------------------------
                let codelinter_version = Self::codelinter_version()?;
                if codelinter_version < Version::from_str("5.0.0")? {
                    return Err(Errors::CompilerError(CompilerError::env_check(
                        "codelinter",
                        "codelinter version should be >= 5.0.0",
                        Some(format!("Current version: {}", codelinter_version).as_str()),
                    )));
                }
                // check hstack version >= 5.0.0 ---------------------------------------------
                let hstack_version = Self::hstack_version()?;
                if hstack_version < Version::from_str("5.0.0")? {
                    return Err(Errors::CompilerError(CompilerError::env_check(
                        "hstack",
                        "hstack version should be >= 5.0.0",
                        Some(format!("Current version: {}", hstack_version).as_str()),
                    )));
                }
                // check ohpm version >= 5.0.0 ----------------------------------------------
                let ohpm_version = Self::ohpm_version()?;
                if ohpm_version < Version::from_str("5.0.0")? {
                    return Err(Errors::CompilerError(CompilerError::env_check(
                        "ohpm",
                        "ohpm version should be >= 5.0.0",
                        Some(format!("Current version: {}", ohpm_version).as_str()),
                    )));
                }
                // check hvigorw version exist ----------------------------------------------
                let _ = Self::hvigorw_exist()?;
                Ok(())
            }
        }
    }
    #[allow(dead_code)]
    fn ci_check() -> Result<(), Errors> {
        // check JDK version is 17 --------------------------------------------------
        let jdk_version: Version = Self::jdk_version()?;
        if jdk_version.match_major(17) != 0 {
            return Err(Errors::CompilerError(CompilerError::env_check(
                "JDK",
                "JDK version should be 17",
                None,
            )));
        }
        Ok(())
    }
    /// get tool version used by tool chain or other tools(jdk, nodejs)
    fn tool_version<F>(
        name: &str,
        recommand: Option<&str>,
        mut f: F,
        is_bat: bool,
    ) -> Result<Version, Errors>
    where
        F: FnMut(&str) -> &str,
    {
        match which(name) {
            Ok(_) => {
                // get version ----------------------------------------------------------
                let output = if is_bat {
                    let bat_name = format!("{}.bat", name);
                    Command::new("cmd")
                        .args(&["/C", &bat_name, "-v"])
                        .output()
                        .map_err(|e| {
                            Errors::CommandError(format!("Failed to execute {}: {}", name, e))
                        })?
                } else {
                    Command::new(name).arg("-v").output().map_err(|e| {
                        Errors::CommandError(format!("Failed to execute {}: {}", name, e))
                    })?
                };

                let output_str = String::from_utf8(output.stdout).unwrap();

                Version::from_str(f(output_str.trim()))
            }
            Err(_) => {
                return Err(Errors::CompilerError(CompilerError::env_check(
                    name,
                    recommand.unwrap_or(COMMAND_LINE_TOOL_MISSING),
                    None,
                )))
            }
        }
    }
    fn codelinter_version() -> Result<Version, Errors> {
        Self::tool_version("codelinter", None, |s| s, true)
    }
    fn hstack_version() -> Result<Version, Errors> {
        Self::tool_version("hstack", None, |s| s, true)
    }
    fn hvigorw_exist() -> Result<(), Errors> {
        match which("hvigorw") {
            Ok(_) => Ok(()),
            Err(_) => Err(Errors::CompilerError(CompilerError::env_check(
                "hvigorw",
                "hvigorw is not installed",
                None,
            ))),
        }
    }

    /// ## get ohos version
    fn ohpm_version() -> Result<Version, Errors> {
        Self::tool_version("ohpm", None, |s| s, true)
    }
    /// ## Get Node.js version
    /// Get Node.js version from the system if is installed
    fn nodejs_version() -> Result<Version, Errors> {
        Self::tool_version(
            "node",
            Some("Nodejs is not installed or config into env"),
            |s| s.trim_start_matches('v'),
            false,
        )
    }

    /// ## Get JDK version (only in CI/CD)
    /// Get JDK version from the system if is installed
    /// ### Return
    /// return `Result<Version, Errors>`
    /// - `Ok(Version)` if JDK is installed and config is correct
    /// - `Err(Errors)` if JDK is not installed or config is incorrect
    fn jdk_version() -> Result<Version, Errors> {
        match which("java") {
            Ok(_) => {
                // get version ----------------------------------------------------------
                let output = Command::new("java")
                    .arg("--version")
                    .output()
                    .map_err(|e| {
                        Errors::CommandError(format!("Failed to execute command: {}", e))
                    })?;

                let output_str = String::from_utf8(output.stdout).unwrap();
                Version::from_str(
                    output_str
                        .trim()
                        .split_whitespace()
                        .nth(1)
                        .unwrap()
                        .trim_matches('"'),
                )
            }
            Err(_) => {
                return Err(Errors::CompilerError(CompilerError::env_check(
                    "JDK",
                    JDK_MISSING,
                    None,
                )))
            }
        }
    }
}

#[cfg(test)]
mod test_api_version {
    use super::*;

    #[test]
    fn test_check() {
        let api = ApiVersion::V12;
        let res = api.check();
        assert!(res.is_ok());
    }

    #[test]
    fn test_check_jdk() {
        let res = ApiVersion::jdk_version();
        assert!(res.is_ok());
    }
    #[test]
    fn test_check_node() {
        let res = ApiVersion::nodejs_version();
        assert!(res.is_ok());
    }
    #[test]
    fn test_check_hstack() {
        let res = ApiVersion::hstack_version();
        assert!(res.is_ok());
    }
    #[test]
    fn test_check_ohpm() {
        let res = ApiVersion::ohpm_version();
        assert!(res.is_ok());
    }
}
