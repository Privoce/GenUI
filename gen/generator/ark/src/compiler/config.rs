use std::{collections::HashMap, path::PathBuf, str::FromStr};

use gen_utils::{
    compiler::{fs, Configer},
    error::Errors,
};
use serde::{Deserialize, Serialize};

/// # The Configure for ArkTS
/// which will compile to `oh-package.json5` in entry directory
/// - get the basic confs from GenUI project `Cargo.toml` (name, version, description, author, license etc.)
/// - do not copy the `dependencies` from toml, because Rust dependencies is not the same as HarmonyOS dependencies
/// - use compiler to pass needed dependencies into the Config struct
/// - the Config struct will be used to generate the `oh-package.json5` file
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// The path of the project config file
    /// in ark, the file path always in `path/to/entry/oh-package.json5`
    /// so the path need to be passed by the compiler(which know where the compiled path is)
    /// and the path must be ignore when serialize to the file
    #[serde(skip)]
    pub path: PathBuf,
    /// Specifies the name of a third-party database.
    /// The value is in the format of @group/packagename and is globally unique.
    /// In addition to @ and /, group and packagename can contain only lowercase letters, digits, underscores (_), and hyphens (-).
    /// The total length is less than or equal to 128 characters.
    /// In addition, group and packagename must start with a letter and cannot be reserved keywords of ArkTS.
    pub name: String,
    /// Third-party library version.Comply with the semver semantic specification
    pub version: String,
    /// Describes the third-party database information, which is helpful for search and discovery.
    pub description: String,
    /// A person who has been involved in creating or maintaining this package.
    pub author: String,
    /// The license under which the package is released.
    pub license: String,
    /// The main field is a module ID that is the primary entry point to your program.
    pub main: String,
    /// Dependencies of the third-party library
    /// ⚠️ todo!(need to use Dep struct to handle the dependencies)
    pub dependencies: HashMap<String, String>,
}

impl Config {
    /// ## check the config file is exists or not 
    /// - if exists, read the file and parse to Config struct (deserialize) then return Config
    /// - if not exists, create a new default Config struct and write to the file (serialize)
    pub fn check<P>(path: P) -> Result<Self, Errors> where P: AsRef<std::path::Path>
    {
        todo!()
    //    fs::try_exists(path.as_ref()).and_then(|exists| {
    //        if exists {
    //            Ok(())
    //        } else {
    //            let config = Config::from_path(path)?;
    //            config.write()
    //        }
    //    })
    }
}

impl Configer for Config {
    fn exists(&self) -> bool {
        fs::exists(self.path.as_path())
    }

    fn try_exists(&self) -> Result<bool, Errors> {
        fs::try_exists(self.path.as_path())
    }

    fn write(&self) -> Result<(), Errors> {
        // deserialize the Config struct to string and write to the file
        let content =
            serde_json::to_string_pretty(self).map_err(|e| Errors::ParseError(e.to_string()))?;
        fs::write(self.path.as_path(), &content)
    }

    fn create(&self) -> Result<(), Errors> {
        fs::create(self.path.as_path())
    }

    fn create_new(&self) -> Result<(), Errors> {
        fs::create_new(self.path.as_path())
    }

    fn delete(&self) -> Result<(), Errors> {
        fs::delete(self.path.as_path())
    }

    fn parse_to<T>(&self) -> Result<T, Errors>
    where
        T: FromStr,
    {
        let self_str =
            serde_json::to_string(self).map_err(|e| Errors::ParseError(e.to_string()))?;
        self_str.parse::<T>().map_err(|_| {
            Errors::ParseError(format!("Parse to {} fail", std::any::type_name::<T>()))
        })
    }

    fn read(&self) -> Result<Self, Errors>
    where
        Self: Sized + FromStr,
    {
        fs::parse_to(self.path.as_path())
    }

    fn from_path<P>(path: P) -> Result<Self, Errors>
    where
        P: AsRef<std::path::Path>,
    {
        let path = path.as_ref().to_path_buf();
        match fs::parse_to::<Self, &std::path::Path>(path.as_path()) {
            Ok(mut config) => {
                config.path = path;
                Ok(config)
            }
            Err(e) => Err(e),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            path: PathBuf::new(),
            name: "entry".to_string(),
            version: "1.0.0".to_string(),
            description: Default::default(),
            author: Default::default(),
            license: Default::default(),
            main: Default::default(),
            dependencies: Default::default(),
        }
    }
}

impl FromStr for Config {
    type Err = Errors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // deserialize the string to Config struct
        let config: Config =
            serde_json::from_str(s).map_err(|e| Errors::ParseError(e.to_string()))?;
        Ok(config)
    }
}

#[cfg(test)]
mod test_config {
    use super::*;

    #[test]
    fn test_config() {
        let config = Config::from_path("E:/Ark/projects/test1/entry/oh-package.json5").unwrap();
        dbg!(config);
    }

    #[test]
    fn test_write() {
        let mut config = Config::from_path("E:/Ark/projects/test1/entry/oh-package.json5").unwrap();
        config.author = "syf20020816".to_string();
        let res = config.write().is_ok();
        dbg!(res);
    }
}
