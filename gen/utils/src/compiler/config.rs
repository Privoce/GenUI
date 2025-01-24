use std::str::FromStr;

use crate::error::Error;

/// # Configer trait
/// Configer trait is used to handle the config file
/// ## Attention
/// ❗Make sure the Config struct's path field is not empty(`!= PathBuf::new() or None`)
/// ## Interfaces
/// - exists
/// - try_exists
/// - read
/// - write
/// - create
/// - create_new
/// - delete
/// - parse_to`<T>`
/// - from_path
pub trait Configer: FromStr {
    /// check the config file is exists or not
    fn exists(&self) -> bool;
    /// check the config file is exists or not by use `fs::try_exists` see: `fn try_exists` in ./fs.rs
    fn try_exists(&self) -> Result<bool, Error>;
    /// read the content of the config file and back Self use deserialize if needed
    fn read(&self) -> Result<Self, Error> where Self: Sized + FromStr;
    /// serialize the Self and write to the config file
    fn write(&self) -> Result<(), Error>;
    /// create the config file if not exists, depends on the self.path
    fn create(&self) -> Result<(), Error>;
    /// create a new config file whatever the file is exists or not
    fn create_new(&self) -> Result<(), Error>;
    /// delete the config file
    fn delete(&self) -> Result<(), Error>;
    /// parse the config file content to T which is implement the `FromStr` trait
    fn parse_to<T>(&self) -> Result<T, Error>
    where
        T: FromStr;
    /// get the config struct from the path
    fn from_path<P>(path: P) -> Result<Self, Error> where P: AsRef<std::path::Path>;
}

/// # FromConfig trait
/// FromConfig trait is used to parse the config file content to the config struct
pub trait FromConfig {
    type From;

    fn from_config(from: &Self::From) -> Self;
}