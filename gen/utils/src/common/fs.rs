//! # File System Utils
//! recommend to use this module to handle the file system operation instead of `std::fs` to control the error and unify
//! ## Interfaces
//! - exists
//! - try_exists
//! - read
//! - write
//! - append
//! - create
//! - create_new
//! - delete
//! - parse_to`<T> T: FromStr`
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
    thread,
    time::Duration,
};

use crate::error::{Error, FsError, ParseError};

use super::Source;

pub fn exists_dir<P>(path: P) -> bool
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    path.exists() && path.is_dir()
}

/// ## Check the file is exists ?
/// return `true` if the file is exists or `false` if not exists
/// if path is empty return false
/// ### Also
/// If you want to get the Error reason, use `try_exists` which will return `Result<bool, Error>`
pub fn exists<P>(path: P) -> bool
where
    P: AsRef<Path>,
{
    path.as_ref().exists()
}
/// ## Check the file is exists ?
/// - if the file is exists, return `Ok(true)`
/// - if the file is not exists, return `Ok(false)` (empty)
/// - if the file can not be sure exists or not, return `Err` (kind like you have no permission to access the file or the parent directory)
pub fn try_exists<P>(path: P) -> Result<bool, Error>
where
    P: AsRef<Path>,
{
    path.as_ref()
        .try_exists()
        .map_err(|e| Error::Fs(FsError::UnExpected(e.to_string())))
}
/// ## Read the file
/// - if the file is exists, return the content of the file as `String`
/// - if the file is not exists, return `Err` (kind like the file can not be found or no permission)
pub fn read<P>(path: P) -> Result<String, Error>
where
    P: AsRef<Path>,
{
    std::fs::read_to_string(path.as_ref()).map_err(|e| {
        Error::Fs(FsError::Read {
            path: path.as_ref().to_path_buf(),
            reason: e.to_string(),
        })
    })
}
/// ## Write the file
/// - if the file is exists, write the content to the file(which will overwrite the origin content)
/// - if the file is not exists, create the file and write the content to the file
/// - if the write process is success, return `Ok(())`
/// - if the write process is fail, return `Err` (kind like the file can not be write or no permission)
/// ### Also
/// If you want to append the content to the file, use `append` method
pub fn write<P>(path: P, content: &str) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    // use create_file to create the file if not exists
    if !path.as_ref().exists() {
        create_file(path.as_ref())?;
    }

    std::fs::write(path.as_ref(), content).map_err(|e| {
        Error::Fs(FsError::Write {
            path: path.as_ref().to_path_buf(),
            reason: e.to_string(),
        })
    })
}
/// ## Append the content to the file
/// - if the file is exists, append the content to the file
/// - if the file is not exists, create the file and write the content to the file
pub fn append<P>(path: P, content: &str) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path.as_ref())
        .and_then(|mut file| file.write_all(content.as_bytes()))
        .map_err(|e| {
            Error::Fs(FsError::Write {
                path: path.as_ref().to_path_buf(),
                reason: e.to_string(),
            })
        })
}

/// ## Create the directory
/// use std::fs::create_dir to create the directory
pub fn create_dir<P>(path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let path = path.as_ref().to_path_buf();
    std::fs::create_dir_all(path.as_path()).map_err(|e| {
        FsError::Create {
            path,
            reason: e.to_string(),
        }
        .into()
    })
}

pub fn exists_or_create_dir<P>(path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    if exists_dir(path.as_ref()) {
        Ok(())
    } else {
        create_dir(path)
    }
}

/// ## Create the file
/// - if is exists, return `Err` (kind like the file is exists, back FsError::UnExpected)
/// - if is not exists, create the file and return `Ok(())`
/// - if the permission is not enough, return `Err` (kind like you have no permission to create the file)
/// ### Also
/// if you want to create a new file and do not care about the exists one, use `create_new` method
pub fn create<P>(path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    std::fs::File::create(path.as_ref())
        .map(|_| ())
        .map_err(|e| Error::Fs(FsError::UnExpected(e.to_string())))
}
/// ## Remove the file
/// - if the file is exists, remove the file and return `Ok(true)`
/// - if the file is not exists, return `Ok(false)`
/// - if the permission is not enough, return `Err` (kind like you have no permission to remove the file, back FsError::UnExpected)
pub fn delete<P>(path: P) -> Result<bool, Error>
where
    P: AsRef<Path>,
{
    std::fs::remove_file(path.as_ref()).map_or_else(
        |e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                Ok(false)
            } else {
                Err(Error::Fs(FsError::UnExpected(e.to_string())))
            }
        },
        |()| Ok(true),
    )
}

/// ## Remove the directory
pub fn delete_dir<P>(path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    if exists_dir(path.as_ref()) {
        std::fs::remove_dir_all(path.as_ref())
            .map(|_| ())
            .map_err(|e| Error::Fs(FsError::UnExpected(e.to_string())))
    } else {
        Ok(())
    }
}

/// ## Move the directory|file from `from` to `to`
/// use walkdir to move the directory
pub fn move_to<P, Q>(from: P, to: Q) -> Result<(), Error>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    for entry in walkdir::WalkDir::new(from.as_ref())
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        let relative = path.strip_prefix(from.as_ref()).unwrap();
        let target = to.as_ref().join(relative);
        if path.is_dir() {
            std::fs::create_dir_all(target)
                .map_err(|e| Error::Fs(FsError::UnExpected(e.to_string())))?;
        } else {
            std::fs::copy(path, target)
                .map_err(|e| Error::Fs(FsError::UnExpected(e.to_string())))?;
        }
    }
    delete_dir(from)
}

/// ## Create the new file
/// - if is exists, remove the exists one and create a new file, return `Ok(())`
/// - if is not exists, create the file and return `Ok(())`
/// - if the permission is not enough, return `Err` (kind like you have no permission to create the file)
/// ### Also
/// if you want to create a file , but if exists one, return `Err`, use `create` method
pub fn create_new<P>(path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    match delete(path.as_ref()) {
        Ok(_) => create(path),
        Err(e) => Err(e),
    }
}
/// ## Parse the file to `T`
/// - if the file is exists,read and then parse the content to `T`
/// - if the file is not exists, return `Err` (kind like the file can not be found or no permission)
pub fn parse_to<T, P>(path: P) -> Result<T, Error>
where
    T: FromStr,
    P: AsRef<Path>,
{
    read(path).and_then(|content| {
        content
            .parse::<T>()
            .map_err(|_| ParseError::template(std::any::type_name::<T>()).into())
    })
}

/// ## Create the file
/// use create_dir_all to create the parent directory if not exists then create the file
/// ### Error
/// Error
/// This function will return an error in the following situations, but is not
/// limited to just these cases:
///
/// * If any directory in the path specified by `path`
/// does not already exist and it could not be created otherwise. The specific
/// error conditions for when a directory is being created (after it is
/// determined to not exist) are outlined by [`fs::create_dir`].
///
/// Notable exception is made for situations where any of the directories
/// specified in the `path` could not be created as it was being created concurrently.
/// Such cases are considered to be successful. That is, calling `create_dir_all`
/// concurrently from multiple threads or processes is guaranteed not to fail
/// due to a race condition with itself.
pub fn create_file<P>(path: P) -> Result<File, Error>
where
    P: AsRef<Path>,
{
    if let Some(parent_dir) = path.as_ref().parent() {
        if !try_exists(parent_dir)? {
            match create_dir_all(parent_dir) {
                Ok(_) => {}
                Err(e) => {
                    return Err(Error::Fs(FsError::Create {
                        path: parent_dir.to_path_buf(),
                        reason: e.to_string(),
                    }))
                }
            };
        }
    } else {
        return Err(Error::Fs(FsError::UnExpected(
            "Path has no parent directory".to_string(),
        )));
    }

    File::create(path.as_ref()).map_err(|e| {
        Error::Fs(FsError::Create {
            path: path.as_ref().to_path_buf(),
            reason: e.to_string(),
        })
    })
}

/// ## Convert the PathBuf to the string (Prevents platform differences)
/// all the path separator will be replaced by `/`
/// if is windows, the prefix `//?/` will be removed
pub fn path_to_str<P>(path: P) -> String
where
    P: AsRef<Path>,
{
    path.as_ref()
        .to_str()
        .unwrap()
        .replace("\\", "/")
        .replace("//?/", "")
}

pub trait GenUIFs {
    /// ## check the file is gen file
    fn is_gen_file(&self) -> bool;
    /// ## convert file to compiled file
    /// rules:
    /// - is `.gen` file: source -> target/src
    /// - not `.gen` file: source -> target
    fn to_compiled<P>(
        &self,
        prefix: P,
        source: P,
        target: P,
        is_delete: bool,
    ) -> Result<PathBuf, Error>
    where
        P: AsRef<Path>;
    /// ## convert file to compiled file from source
    fn to_compiled_from_source(&self, source: &Source) -> Result<PathBuf, Error>;
    /// ## convert file to compiled file from delete
    fn to_compiled_from_delete(&self, source: &Source) -> Result<PathBuf, Error>;
    /// ## back rs to gen file
    fn back_gen(&self) -> PathBuf;

    fn widget_source(&self, source: &Source) -> Result<Source, Error>;
}

impl<P> GenUIFs for P
where
    P: AsRef<Path>,
{
    fn is_gen_file(&self) -> bool {
        // 后缀名需要处理一下，文件也可能没有后缀名
        self.as_ref().is_file() && self.as_ref().extension().map_or(false, |ext| ext == "gen")
    }

    fn to_compiled<PT>(
        &self,
        prefix: PT,
        source: PT,
        target: PT,
        is_delete: bool,
    ) -> Result<PathBuf, Error>
    where
        PT: AsRef<Path>,
    {
        let mut path = self.as_ref().to_path_buf();
        let mut compiled = prefix.as_ref().join(target.as_ref());

        let flag = if is_delete {
            self.as_ref().extension().map_or(false, |ext| ext == "gen")
        } else {
            self.is_gen_file()
        };

        if flag {
            // add src and change extension to rs
            compiled = compiled.join("src");
            path = path.with_extension("rs");
        }

        Ok(compiled.join(
            path.strip_prefix(prefix.as_ref().join(source.as_ref()))
                .map_err(|e| Error::Fs(FsError::UnExpected(e.to_string())))?,
        ))
    }

    fn to_compiled_from_source(&self, source: &Source) -> Result<PathBuf, Error> {
        self.to_compiled(&source.path, &source.from, &source.to, false)
    }

    fn to_compiled_from_delete(&self, source: &Source) -> Result<PathBuf, Error> {
        self.to_compiled(&source.path, &source.from, &source.to, true)
    }

    fn back_gen(&self) -> PathBuf {
        if self.as_ref().is_file() && self.as_ref().extension().map_or(false, |ext| ext == "rs") {
            return self.as_ref().with_extension("gen");
        }
        self.as_ref().to_path_buf()
    }

    fn widget_source(&self, source: &Source) -> Result<Source, Error> {
        let compiled_path = self.as_ref().to_compiled_from_source(source)?;
        Ok(Source::new(
            source.path.as_path(),
            self.as_ref(),
            compiled_path.as_path(),
        ))
    }
}

/// ## File state enum
/// which should be used to represent the state of a file
///
/// from notify::EventKind, this enum may change in the future if needed
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileState {
    Unchanged,
    Modified,
    Created,
    Deleted,
    Renamed,
}

impl FileState {
    /// match state if is modified or created then do then function
    ///
    /// else do nothing
    pub fn modify_then<T, F>(&self, default: T, f: F) -> Result<T, Error>
    where
        F: FnOnce() -> Result<T, Error>,
    {
        match self {
            FileState::Modified | FileState::Created | FileState::Renamed | FileState::Deleted => {
                f()
            }
            _ => Ok(default),
        }
    }
    pub fn then<F>(&self, f: F) -> Result<(), Error>
    where
        F: FnOnce(&Self) -> Result<(), Error>,
    {
        f(&self)
    }
    pub fn is_modify(&self) -> bool {
        !matches!(self, FileState::Unchanged)
    }
}

/// copy file from source_path to compiled_path
pub fn copy_file<P, Q>(from: P, to: Q) -> Result<(), Error>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    // Extract the directory part from the compiled_path
    if let Some(parent_dir) = to.as_ref().parent() {
        // Check if the directory exists, if not, create it
        if !parent_dir.exists() {
            // Create the directory and any necessary parent directories
            create_dir_all(parent_dir).map_err(|e| {
                Error::Fs(FsError::Create {
                    path: parent_dir.to_path_buf(),
                    reason: e.to_string(),
                })
            })?;
        }
    }

    // Copy the file from source_path to compiled_path
    // fs::copy(from, to).expect("Failed to copy file to compiled project");
    copy_with_retries(from, to, 5, Duration::from_millis(200))
}

/// copy file from source_path to compiled_path with retries
fn copy_with_retries<P, Q>(
    from: P,
    to: Q,
    max_attempts: usize,
    delay: Duration,
) -> Result<(), Error>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let mut attempts = 0;
    loop {
        match std::fs::copy(from.as_ref(), to.as_ref()) {
            Ok(_) => return Ok(()),
            Err(_) if attempts < max_attempts => {
                attempts += 1;
                thread::sleep(delay);
            }
            Err(e) => {
                return Err(FsError::UnExpected(format!(
                    "Failed to copy file to compiled project: {}",
                    e.to_string()
                ))
                .into())
            }
        }
    }
}

pub fn relative_to_absolute<P1, P2>(prefix: P1, path: P2) -> PathBuf
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    let path = path.as_ref();
    if path.is_relative() {
        prefix.as_ref().join(path)
    } else {
        path.to_path_buf()
    }
}

#[cfg(test)]
mod test_fs {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_create_file() {
        let _res = create_file(
            "E:/Rust/try/makepad/Gen-UI/examples/gen_makepad_simple/src_gen/src/views/root.rs",
        );
    }

    #[test]
    fn test_exists() {
        let res = exists(PathBuf::new());
        assert!(!res);
    }
    #[test]
    fn test_try_exists() {
        let res = try_exists(PathBuf::new());
        assert!(res.is_err());
    }
}
