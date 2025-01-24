// use std::{
//     ffi::OsString,
//     fs,
//     hash::Hash,
//     path::{Path, PathBuf},
// };

// use super::snake_to_camel;

// /// # Source
// /// The Source struct is designed to manage and transform paths related to source and compiled files within a project.
// /// It primarily deals with tracking the origin (source) directories and files and their corresponding compiled directories and files.
// ///
// /// This struct provides utility methods to extract information and perform conversions
// /// that are essential in build processes, such as determining module names, handling directory structures,
// /// and ensuring proper compilation outputs.
// ///
// /// Source结构体旨在管理和转换与项目中的源文件和编译文件相关的路径。
// /// 它主要处理跟踪源目录和文件及其相应的编译目录和文件。
// /// 此结构提供了提取信息和执行构建过程中必不可少的转换的实用方法，例如确定模块名称、处理目录结构和确保正确的编译输出。
// #[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
// pub struct Source {
//     /// source file dir
//     pub origin_dir: PathBuf,
//     pub origin_file: PathBuf,
//     // /// source file path
//     // pub source: PathBuf,
//     /// compiled file path
//     pub compiled_dir: PathBuf,
//     pub compiled_file: PathBuf,
// }

// impl Source {
//     pub fn to_auto(&self, widget_name: &str, ulid: &str) -> PathBuf {
//         self.compiled_dir
//             .as_path()
//             .join("src")
//             .join("auto")
//             .join(&format!("{}_{}.rs", widget_name, ulid))
//     }
//     /// get name from source origin file
//     /// eg: src_gen/widget/hello.gen -> Hello
//     pub fn source_name(&self) -> String {
//         let name = self.source_name_lower();
//         snake_to_camel(&name)
//     }
//     /// get name from source origin file back the file name without suffix
//     pub fn source_name_lower(&self) -> String {
//         self.origin_file
//             .file_name()
//             .unwrap()
//             .to_str()
//             .unwrap()
//             .to_string()
//             .replace(".gen", "")
//     }
//     /// source name lower and use rust style
//     /// ### attention
//     /// if source name is mod, should use the parent fold name
//     pub fn source_name_rs(&self) -> String {
//         let mut name = self.source_name_lower();
//         if name.eq("mod") {
//             let path = self.origin_file.parent().unwrap().to_path_buf();
//             name = path.iter().last().unwrap().to_str().unwrap().to_string();
//         }
//         name
//     }
//     /// get level from source compiled file
//     /// - eg1:
//     ///     - dir: a/b/c
//     ///     - file:  a/b/c/d.gen
//     /// > result:
//     /// - eg2:
//     ///     - dir: a/b/c
//     ///     - file: a/b/c/d/e.gen
//     /// > result: d
//     pub fn level_gen(&self) -> PathBuf {
//         let mut level = self
//             .compiled_file
//             .strip_prefix(self.compiled_dir.as_path())
//             .unwrap()
//             .to_path_buf();

//         // remove the last
//         level.pop();
//         level
//     }
//     /// to_lib can convert Source to lib.rs pub mod
//     pub fn to_lib(&self) -> String {
//         let path = self.remove_src();
//         let target = path
//             .iter()
//             .next()
//             .expect("can not get src following folder or file")
//             .to_str()
//             .unwrap()
//             .to_string();
//         if path.is_file() {
//             // remove suffix
//             target.split_once('.').unwrap().0.to_string()
//         } else {
//             target
//         }
//     }
//     pub fn to_live_register(&self) -> String {
//         let path = self.remove_src();
//         // remove extension
//         let without_ext = path
//             .file_stem()
//             .expect("can not get file name, expect a gen file");
//         let path = path.with_file_name(without_ext);
//         path.components()
//             .map(|item| item.as_os_str().to_string_lossy())
//             .collect::<Vec<_>>()
//             .join("::")
//     }
//     fn remove_src(&self) -> PathBuf {
//         let path = self
//             .compiled_file
//             .strip_prefix(self.compiled_dir.as_path())
//             .unwrap()
//             .to_path_buf();

//         // remove src and get the first path
//         let path = path.strip_prefix("src/").expect("remove src failed");
//         path.to_path_buf()
//     }
//     pub fn as_os_str(&self) -> &std::ffi::OsStr {
//         self.compiled_file.as_os_str()
//     }
//     /// ## origin_dir to compiled dir
//     /// this function will check whether the origin_dir has `mod.gen` or not
//     ///
//     /// if has, it will be generate path under `src_gen/src` else under `src_gen` (if parent dir is project path)
//     /// ### example
//     /// #### has mod.gen
//     /// - origin_dir: `E:/Rust/try/makepad/Gen-UI/examples/simple1/ui/views/a`
//     /// - compiled_dir: `E:/Rust/try/makepad/Gen-UI/examples/simple1/src_gen/src/views/a`
//     pub fn origin_dir_to_compiled<P>(origin_dir: P, path: P) -> PathBuf
//     where
//         P: AsRef<Path>,
//     {
//         /// check whether has mod.gen?
//         fn check<P>(path: P) -> bool
//         where
//             P: AsRef<Path>,
//         {
//             fs::read_dir(path.as_ref().parent().unwrap())
//                 .unwrap()
//                 .any(|item| item.unwrap().file_name().to_str().unwrap().eq("mod.gen"))
//         }
//         let mut compiled_project_path = Source::project_dir_to_compiled(origin_dir.as_ref());
//         let strip_path = path
//             .as_ref()
//             .strip_prefix(origin_dir.as_ref())
//             .unwrap()
//             .to_path_buf();
//         if check(path.as_ref()) || path.as_ref().parent().unwrap().ne(origin_dir.as_ref()) {
//             // strip the origin_dir
//             compiled_project_path.push("src");
//         }
//         // target.push_front("src_gen".into());
//         let path = compiled_project_path.join(strip_path.as_path());
//         path
//     }
//     /// prject_origin_dir to compiled_dir replace origin dir to src_gen
//     /// only use when you need to get path of current project to compiled project's root path
//     pub fn project_dir_to_compiled<P>(origin_dir: P) -> PathBuf
//     where
//         P: AsRef<Path>,
//     {
//         let mut tmp = origin_dir.as_ref().to_path_buf();
//         tmp.pop();
//         tmp.push("src_gen");
//         tmp
//     }
//     /// end with .gen
//     pub fn origin_file_to_compiled<P1, P2>(origin_file: P1, origin_dir: P2) -> PathBuf
//     where
//         P1: Into<PathBuf>,
//         P2: Into<PathBuf>,
//     {
//         Source::origin_file_to_compiled_or(origin_file, origin_dir, true)
//     }
//     /// not end with .gen
//     pub fn origin_file_without_gen<P1, P2>(origin_file: P1, origin_dir: P2) -> PathBuf
//     where
//         P1: Into<PathBuf>,
//         P2: Into<PathBuf>,
//     {
//         Source::origin_file_to_compiled_or(origin_file, origin_dir, false)
//     }
//     fn origin_file_to_compiled_or<P1, P2>(origin_file: P1, origin_dir: P2, compile: bool) -> PathBuf
//     where
//         P1: Into<PathBuf>,
//         P2: Into<PathBuf>,
//     {
//         let mut tmp: PathBuf = origin_dir.into();
//         tmp.pop();

//         let strip_path: PathBuf = origin_file.into();

//         let strip_path = strip_path.strip_prefix(&tmp.as_path()).unwrap();

//         let mut target: Vec<OsString> = strip_path.iter().map(OsString::from).collect();

//         // 检查是否有足够可以修改
//         if !target.is_empty() {
//             // 替换第一个
//             target[0] = "src_gen".into();
//             if target.last().unwrap().to_str().unwrap().ends_with(".gen") {
//                 // 在target[0]后面插入一个src
//                 target.insert(1, "src".into());
//             }
//             if compile {
//                 if let Some(last) = target.last_mut() {
//                     *last = last.to_str().unwrap().replace(".gen", ".rs").into();
//                 }
//             }
//         }

//         // 使用base和修改后的组件重新构建完整的路径
//         tmp.clone().join(PathBuf::from_iter(target))
//     }
// }

// /// one is for source file path another is for source dir
// /// (source file path, source dir path)
// impl<P1, P2> From<(P1, P2)> for Source
// where
//     P1: AsRef<Path>,
//     P2: AsRef<Path>,
// {
//     fn from(value: (P1, P2)) -> Self {
//         let mut tmp = value.1.as_ref().to_path_buf();
//         tmp.pop();

//         let strip_path = value.0.as_ref().to_path_buf();
//         let strip_path = strip_path.strip_prefix(&tmp.as_path()).unwrap();

//         let mut target: Vec<OsString> = strip_path.iter().map(OsString::from).collect();

//         // 检查是否有足够可以修改
//         if !target.is_empty() {
//             // 替换第一个
//             target[0] = "src_gen".into();
//             // 检查当前文件的后缀是否是.gen，如果是则需要将整个父目录移动到src下且将文件后缀改为.rs
//             if target.last().unwrap().to_str().unwrap().ends_with(".gen") {
//                 // 在target[0]后面插入一个src
//                 target.insert(1, "src".into());
//             }

//             if let Some(last) = target.last_mut() {
//                 *last = last.to_str().unwrap().replace(".gen", ".rs").into();
//             }
//         }
//         // 使用base和修改后的组件重新构建完整的路径
//         let compiled_file = tmp.clone().join(PathBuf::from_iter(target));
//         let mut compiled_dir = tmp;
//         compiled_dir.push("src_gen");
//         Source {
//             origin_dir: value.1.as_ref().to_path_buf(),
//             origin_file: value.0.as_ref().to_path_buf(),
//             compiled_dir,
//             compiled_file,
//         }
//     }
// }

// #[cfg(test)]
// mod test_source {
//     use std::{path::PathBuf, str::FromStr};

//     use super::Source;

//     #[test]
//     fn origin_f() {
//         let source1 = Source::origin_file_to_compiled(
//             "E:/Rust/try/makepad/Gen-UI/examples/simple1/ui/views/a/1.gen",
//             "E:/Rust/try/makepad/Gen-UI/examples/simple1/ui",
//         );

//         let source2 = Source::from((
//             "E:/Rust/try/makepad/Gen-UI/examples/simple1/ui/views/a/1.gen",
//             "E:/Rust/try/makepad/Gen-UI/examples/simple1/ui",
//         ));

//         assert_eq!(source1, source2.compiled_file);
//     }

//     #[test]
//     fn origin_dir() {
//         let path1 = Source::origin_dir_to_compiled(
//             "E:\\Rust\\try\\makepad\\Gen-UI\\examples\\simple1\\ui",
//             "E:\\Rust\\try\\makepad\\Gen-UI\\examples\\simple1\\ui\\views\\a",
//         );
//         let compiled1 = PathBuf::from_str(
//             "E:\\Rust\\try\\makepad\\Gen-UI\\examples\\simple1\\src_gen\\src\\views\\a",
//         );

//         let path2 = Source::origin_dir_to_compiled(
//             "E:\\Rust\\try\\makepad\\Gen-UI\\examples\\simple1\\ui",
//             "E:\\Rust\\try\\makepad\\Gen-UI\\examples\\simple1\\ui\\static",
//         );
//         let compiled2 =
//             PathBuf::from_str("E:\\Rust\\try\\makepad\\Gen-UI\\examples\\simple1\\src_gen\\static");

//         assert_eq!(path1, compiled1.unwrap());
//         assert_eq!(path2, compiled2.unwrap());
//     }
// }

use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Source {
    pub path: PathBuf,
    pub from: PathBuf,
    pub to: PathBuf,
}

impl Source {
    pub fn new<P>(path: P, from: P, to: P) -> Self
    where
        P: AsRef<Path>,
    {
        // need to check strip prefix
        let from = if let Ok(from) = from.as_ref().strip_prefix(path.as_ref()) {
            from.to_path_buf()
        } else {
            from.as_ref().to_path_buf()
        };

        let to = if let Ok(to) = to.as_ref().strip_prefix(path.as_ref()) {
            to.to_path_buf()
        } else {
            to.as_ref().to_path_buf()
        };

        Self {
            path: path.as_ref().to_path_buf(),
            from,
            to,
        }
    }
    pub fn from_path(&self) -> PathBuf {
        self.path.join(&self.from)
    }
    pub fn to_path(&self) -> PathBuf {
        self.path.join(&self.to)
    }
}
