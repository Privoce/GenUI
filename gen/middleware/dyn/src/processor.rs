use libloading::{Library, Symbol};
use std::path::PathBuf;

pub struct DynProcessor {
    lib: Option<Library>,
    lib_path: PathBuf,
}

impl DynProcessor {
    pub fn new(lib_path: PathBuf) -> Self {
        Self {
            lib: None,
            lib_path,
        }
    }

    // 加载动态库
    pub fn load_library(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.lib = Some(unsafe { Library::new(&self.lib_path)? });
        Ok(())
    }

    // // 处理宏
    // pub unsafe fn process_macro(
    //     &self,
    //     context: &mut MacroContext,
    // ) -> Result<(), Box<dyn std::error::Error>> {
    //     if let Some(lib) = &self.lib {
    //         let process_func: Symbol<unsafe extern "C" fn(&mut MacroContext)> =
    //             lib.get(b"process_macro")?;
    //         process_func(context);
    //         Ok(())
    //     } else {
    //         Err("Library not loaded".into())
    //     }
    // }
    pub unsafe fn process_macro<T>(&self, inner: &mut T) -> Result<bool, Box<dyn std::error::Error>> {
        if let Some(lib) = &self.lib {
            let process_func: Symbol<unsafe extern "C" fn(&mut T) -> bool> = lib.get(b"process_macro")?;
            Ok(process_func(inner))
        } else {
            Err("Library not loaded".into())
        }
    }
}
