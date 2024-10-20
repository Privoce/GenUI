use super::TargetCompiler;
use crate::{calc_hash, info, is_eq_path, FileState};
use gen_utils::common::msg::{
    CACHE_NOT_EXIST, CACHE_OPEN_CREATE_FAIL, CACHE_WRITE, CACHE_WRITE_FAIL,
};
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

/// ## Gen compile cache
/// use msgpack to serialize and deserialize
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cache {
    /// cache file path
    path: PathBuf,
    /// current os
    os: String,
    /// compile target, default => makepad
    target: TargetCompiler,
    /// cache values, key is file path, value is file hash value
    values: Option<HashMap<PathBuf, String>>,
}

impl Cache {
    pub fn new<P>(origin_path: P, target: TargetCompiler) -> Self
    where
        P: AsRef<Path>,
    {
        let mut path = origin_path.as_ref().to_path_buf();
        let _ = path.push(".gen_cache");

        // current instance
        let instance = Self {
            path: path.clone(),
            os: std::env::consts::OS.to_string(),
            target,
            values: None,
        };

        // check cache file is exist? if existed, read and deserialize it to new cache instance and compare to current system
        let cache = Cache::read(path.as_path())
            .map(|v| {
                return if v.is_same_except_values(&instance) {
                    // same means no need to create a new cache file, back current
                    v
                } else {
                    // back new instacne
                    let _ = instance.write();
                    instance.clone()
                };
            })
            .unwrap_or_else(|_| {
                // create a new cache file and return instance
                let _ = instance.write();
                instance
            });

        // now we can get the cache file instance
        cache
    }
    /// compare two cache instance is same or not (except values field)
    ///
    /// if same return true, else return false
    pub fn is_same_except_values(&self, another: &Cache) -> bool {
        let another_path = another.path.to_str().unwrap();
        let self_path = self.path.to_str().unwrap();

        match (
            self_path.eq(another_path),
            self.os.eq(&another.os),
            self.target.eq(&another.target),
        ) {
            (true, true, true) => true,
            _ => false,
        }
    }
    /// compare two cache instance is same or not
    /// all field is same return true
    pub fn is_same(&self, another: &Cache) -> bool {
        if self.is_same_except_values(another) {
            return self.values.eq(&another.values);
        } else {
            return false;
        }
    }
    // read cache file by path and deserialize it to cache instance
    pub fn read<P>(path: P) -> Result<Cache, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        return if path.as_ref().exists() {
            let f = File::open(path)?;
            let mut decode = Deserializer::new(f);
            let cache: Cache = Deserialize::deserialize(&mut decode)?;
            Ok(cache)
        } else {
            // cache file not exist
            Err(CACHE_NOT_EXIST.into())
        };
    }

    // create cache file and write cache instance to it
    pub fn write(&self) -> () {
        let cache_path = self.path.as_path();
        let mut file = if !cache_path.exists() {
            // create a new file
            File::options()
                .write(true)
                .read(true)
                .create_new(true)
                .open(cache_path)
        } else {
            File::options().write(true).read(true).open(cache_path)
        }
        .expect(CACHE_OPEN_CREATE_FAIL);

        let mut buf = Vec::new();

        let _ = self.serialize(&mut Serializer::new(&mut buf)).unwrap();

        let _ = file.write(&buf).expect(CACHE_WRITE_FAIL);

        info(CACHE_WRITE);
    }
    pub fn insert<P>(&mut self, key: P, value: String) -> ()
    where
        P: AsRef<Path>,
    {
        match &mut self.values {
            Some(values) => {
                values.insert(key.as_ref().to_path_buf(), value);
            }
            None => {
                let mut values = HashMap::new();
                values.insert(key.as_ref().to_path_buf(), value);
                self.values = Some(values);
            }
        }
    }
    pub fn exists<P>(&self, key: P) -> bool
    where
        P: AsRef<Path>,
    {
        match &self.values {
            Some(values) => values.contains_key(key.as_ref()),
            None => false,
        }
    }
    /// if exists, then calc hash with origin, if hash equal, don't insert and return FileState::Unchanged
    ///
    /// if not exists, insert and return FileState::Created
    ///
    /// if exists but hash not equal, insert and return FileState::Modified
    pub fn exists_or_insert<P>(&mut self, key: P) -> Result<FileState, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let hash = calc_hash(key.as_ref())?;
        match &mut self.values {
            Some(values) => {
                if let Some(value) = values.get(key.as_ref()) {
                    // exist cache
                    if value.eq(&hash) {
                        return Ok(FileState::Unchanged);
                    } else {
                        self.insert(key, hash);
                        return Ok(FileState::Modified);
                    }
                } else {
                    self.insert(key, hash);
                    return Ok(FileState::Created);
                }
            }
            None => {
                self.insert(key, hash);
                return Ok(FileState::Created);
            }
        }
    }
    pub fn remove<P>(&mut self, key: P) -> ()
    where
        P: AsRef<Path>,
    {
        match &mut self.values {
            Some(values) => {
                values.remove(key.as_ref());
            }
            None => (),
        }
    }
    /// remove all cache values depend on path(path is dir path)
    /// this function will remove all path which is start with the path
    pub fn remove_all<P>(&mut self, path: P) -> ()
    where
        P: AsRef<Path>,
    {
        if let Some(values) = &mut self.values {
            values.retain(|k, _| !is_eq_path(k.as_path(), path.as_ref(), true));
        }
    }
    pub fn insert_and_hash<P>(&mut self, key: P) -> Result<(), Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let hash = calc_hash(key.as_ref())?;
        self.insert(key, hash);
        Ok(())
    }
    pub fn clear(&mut self) -> () {
        self.values = None;
    }
    pub fn get<P>(&self, key: P) -> Option<&String>
    where
        P: AsRef<Path>,
    {
        match &self.values {
            Some(values) => values.get(key.as_ref()),
            None => None,
        }
    }
    /// get all gen file path from cache values
    pub fn get_gen(&self) -> Option<Vec<&PathBuf>> {
        return if let Some(files) = &self.values {
            Some(
                files
                    .keys()
                    .filter(|item| item.extension().unwrap().eq("gen"))
                    .collect(),
            )
        } else {
            None
        };
    }
    /// get all cache values
    pub fn values(&self) -> Option<Vec<PathBuf>> {
        if let Some(values) = self.values.as_ref() {
            Some(values.keys().map(|k| k.clone()).collect())
        } else {
            None
        }
    }
}
