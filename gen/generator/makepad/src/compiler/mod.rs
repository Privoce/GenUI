mod conf;
mod context;
mod tree;
// mod wasm;

use std::{collections::HashSet, path::PathBuf};

// pub use wasm::*;
pub use conf::*;
pub use context::*;
use gen_analyzer::Model;
use gen_dyn_run::{compile_dyn_lib, dyn_lib_path, extern_c_fn, DynProcessor};
use gen_plugin::{MacroContext, Repo};
use gen_utils::{
    common::{
        fs::{self, GenUIFs},
        git_download_plugin_from_github, read_to_doc, RustDependence, Source,
    },
    compiler::{CompilerImpl, ToRs, UnderlayerConfImpl},
    err_from_to,
    error::{ConvertError, Error},
};
use proc_macro2::TokenStream;
use toml_edit::{value, Item};
use tree::ModelTree;
use walkdir::WalkDir;

use crate::model::{create_lib_rs, create_main_rs, AppMain, Widget};

// ----------------------------------------------------------------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------------------------------------------------------------
// -------------------------------------------------------- GenUI Makepad Compiler --------------------------------------------------------
// ----------------------------------------------------------------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------------------------------------------------------------
pub struct Compiler {
    /// 存储了编译器源信息，编译器编译的项目地址，编译后的项目根地址
    pub source: Source,
    /// 存储编译器的配置信息
    pub conf: Config,
    /// 组件树
    pub tree: ModelTree,
    /// 编译器上下文，用于存储需要编译过程中需要传递的信息
    pub context: Context,
}

impl Compiler {
    pub fn new(source: Source, conf: &Box<dyn UnderlayerConfImpl>) -> Result<Self, Error> {
        // [convert UnderlayerConfImpl to Config] -------------------------------------------------
        let mut conf = conf.as_any().downcast_ref::<Config>().map_or_else(
            || {
                Err(ConvertError::FromTo {
                    from: "Box<dyn UnderlayerConfImpl>".to_string(),
                    to: "Makepad Config".to_string(),
                })
            },
            |conf| Ok(conf.clone()),
        )?;
        // [get dependencies from source Cargo] ----------------------------------------------------
        let source_cargo = source.from_path().join("Cargo.toml");
        let dependencies = read_to_doc(source_cargo.as_path())?
            .get("dependencies")
            .map_or_else(
                || Err(err_from_to!("Cargo.toml" => "toml[dependencies]")),
                |deps| RustDependence::from_item(deps),
            )?;

        for dep in dependencies {
            conf.push_dep(dep);
        }

        // [root node] ----------------------------------------------------------------------------
        let tree = ModelTree::new(source.to_path().join("src"));
        // [context] -----------------------------------------------------------------------------
        let mut context = Context::default();
        if let Some(routers) = conf.routers.as_ref() {
            context.load_routers(routers, source.from_path())?;
        }

        Ok(Self {
            source,
            conf,
            tree,
            context,
        })
    }

    fn create_lib_rs(&self) -> Result<(), Error> {
        let lib_rs_path = self.source.to_path().join("src").join("lib.rs");
        // [查看plugins并进行模块引入] -------------------------------------------------------------
        let plugin_lib_str = self.context.plugins.as_ref().map(|plugins| {
            plugins.iter().fold(String::new(), |mut acc, plugin| {
                let name = &plugin.plugin.name;
                acc.push_str(&format!("mod {name}; pub use {name}::*;"));
                acc
            })
        });

        let lib_content = self.context.lib_content.as_ref().map_or_else(
            || plugin_lib_str.clone(),
            |content| {
                let mut lib_str = plugin_lib_str.as_ref().cloned().unwrap_or_default();
                lib_str.push_str(content);
                Some(lib_str)
            },
        );

        fs::write(
            lib_rs_path.as_path(),
            &create_lib_rs(
                self.tree.lib_rs(),
                self.conf.entry.as_ref(),
                lib_content.as_ref(),
            )
            .to_string(),
        )
    }

    fn create_main_rs(&self) -> Result<(), Error> {
        let main_rs_path = self.source.to_path().join("src").join("main.rs");
        fs::write(
            main_rs_path.as_path(),
            &create_main_rs(self.source.to.to_str().unwrap()).to_string(),
        )
    }

    fn create_app_main(&mut self) -> Result<(), Error> {
        let source = AppMain::source_from_entry(self.conf.entry.as_ref(), &self.source);
        let mut app_main = AppMain::new(&mut self.context, source, &self.conf.root)?;
        app_main.registers.replace(self.tree.registers());
        fs::write(app_main.source.to_path(), &app_main.content()?.to_string())
    }

    fn download_plugins(&self) -> Result<(), Error> {
        if let Some(plugins) = self.context.plugins.as_ref() {
            let compiled_src = self.source.to_path().join("src");
            for plugin in plugins {
                match &plugin.plugin.repo {
                    Repo::Path(path) => {
                        // copy path to compiled_path/src
                        let to_path = compiled_src.join(plugin.plugin.name.as_str());
                        return fs::move_to(path, to_path.as_path());
                    }
                    Repo::Git(_) => {
                        return git_download_plugin_from_github(
                            &plugin.plugin.name,
                            false,
                            compiled_src.as_path(),
                            |info| println!("{}", info),
                            |err| println!("{}", err),
                        );
                    }
                }
            }
        }

        Ok(())
    }
}

impl CompilerImpl for Compiler {
    fn execute_auxiliaries(&mut self, _executor: gen_utils::compiler::Executor) -> () {}

    fn recv_plugins(
        &mut self,
        plugins: Option<&std::collections::HashMap<String, PathBuf>>,
    ) -> Result<(), Error> {
        if let Some(plugins) = plugins {
            let mut set = HashSet::new();

            for (_, path) in plugins {
                let item =
                    gen_plugin::Token::try_from(path).map_err(|e| Error::from(e.to_string()))?;
                set.insert(item);
            }

            if !set.is_empty() {
                // generate dynamic lib
                let dyn_lib_path = dyn_lib_path(self.source.from_path().as_path(), "gen_plugin");

                let code = extern_c_fn(
                    &MacroContext::to_string_param(),
                    set.iter()
                        .fold(TokenStream::new(), |mut acc, plugin| {
                            acc.extend(plugin.to_dyn_code());
                            acc
                        })
                        .to_string()
                        .as_str(),
                );

                compile_dyn_lib(&MacroContext::to_string_struct(), &code, &dyn_lib_path)
                    .map_err(|e| Error::from(e.to_string()))?;

                self.context.dyn_processor = Some(DynProcessor::new(dyn_lib_path));
                self.context
                    .dyn_processor
                    .as_mut()
                    .unwrap()
                    .load_library()
                    .map_err(|e| Error::from(e.to_string()))?;
                self.context.plugins = Some(set);
            };
        }

        Ok(())
    }

    fn send_plugins(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn init(&mut self) -> Result<(), Error> {
        // [Cargo toml] -----------------------------------------------------------------------
        let cargo_path = self
            .source
            .path
            .join(self.source.to.as_path())
            .join("Cargo.toml");
        let cargo_toml = read_to_doc(cargo_path.as_path())?;

        let target_deps = cargo_toml.get("dependencies").map_or_else(
            || Err(err_from_to!("Cargo.toml" => "toml[dependencies]")),
            |deps| RustDependence::from_item(deps),
        )?;

        // [source Cargo toml] ----------------------------------------------------------------
        let s_cargo_path = self
            .source
            .path
            .join(self.source.from.as_path())
            .join("Cargo.toml");
        let mut source_toml = read_to_doc(s_cargo_path.as_path())?;

        // - [copy source Cargo to target Cargo except name] ---------------------------------
        let sync_flag = if let Some(source_deps) = self.conf.dependencies.as_ref() {
            if !RustDependence::eq(&source_deps, &target_deps) {
                true
            } else {
                false
            }
        } else {
            false
        };

        if sync_flag {
            // [name] -----------------------------------------------------------------------
            source_toml["package"]["name"] = value(self.source.to.to_str().unwrap());
            // [dependencies] ---------------------------------------------------------------
            let deps = if let Some(deps) = self.conf.dependencies.as_ref() {
                RustDependence::vec_to_item(deps)
            } else {
                Item::None
            };
            source_toml.insert("dependencies", deps);
            // [write to target Cargo.toml] -------------------------------------------------
            fs::write(cargo_path.as_path(), &source_toml.to_string())?;
        }
        Ok(())
    }

    fn before_compile(&mut self) -> Result<(), Error> {
        // [create main.rs] -------------------------------------------------------------------
        let _ = self.create_main_rs()?;
        // [create lib.rs] --------------------------------------------------------------------
        let _ = self.create_lib_rs()?;
        // [create app main] ------------------------------------------------------------------
        let _ = self.create_app_main()?;
        // [download plugins] -----------------------------------------------------------------
        let _ = self.download_plugins()?;
        Ok(())
    }

    fn after_compile(&mut self) -> Result<(), Error> {
        println!("after compile");
        Ok(())
    }

    fn compile(&mut self, path: PathBuf) -> Result<(), Error> {
        if path.is_file() && path.file_name().unwrap() == "main.rs" {
            // main.rs文件不需要编译直接复制到lib.rs中
            let lib_content = fs::read(path.as_path())?;
            if !lib_content.is_empty() {
                self.context.lib_content = Some(lib_content);
            }
            return Ok(());
        }

        // 编译gen文件，生成rs文件，生成rs文件后，将rs文件插入到tree中
        // 需要注意的是只有一个root.gen(self.conf.root)的跟节点文件需要处理一些特殊逻辑, 需要将一些信息存储到compiler到上下文中
        let compiled_path = path.as_path().to_compiled_from_source(&self.source)?;
        let widget_source = path.as_path().widget_source(&self.source)?;
        let model = Model::new(widget_source, self.conf.root.is_root(path.as_path()))?;

        if !model.is_empty() {
            // 编译widget
            let widget = Widget::new(&mut self.context, model)?;
            // 将widget插入到tree中
            let _ = self.tree.insert(compiled_path.as_path());
            fs::write(compiled_path, &ToRs::content(&widget)?.to_string())
        } else {
            Ok(())
        }
    }

    fn update(&mut self) -> Result<(), Error> {
        self.create_lib_rs()?;
        self.create_app_main()
    }

    fn remove(&mut self, path: PathBuf) -> Result<Option<Vec<PathBuf>>, Error> {
        let compiled_path = path.as_path().to_compiled_from_delete(&self.source)?;
        if compiled_path.is_file() {
            self.tree.remove(compiled_path.as_path());
            fs::delete(compiled_path.as_path()).map(|_| Some(vec![path]))
        } else {
            // dir, get all files below
            let files = WalkDir::new(compiled_path.as_path())
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
                .map(|e| e.path().to_path_buf())
                .collect::<Vec<PathBuf>>();

            let mut origins = vec![];
            for file in files {
                let path = if self.tree.remove(file.as_path()).is_some() {
                    file.back_gen()
                } else {
                    file
                };
                origins.push(path);
            }

            fs::delete_dir(compiled_path.as_path()).map(|_| Some(origins))
        }
    }
}

#[cfg(test)]
mod tes {
    #[test]
    fn test_pathbuf_eq() {
        let a =
            std::path::PathBuf::from("/Users/shengyifei/projects/gen_ui/GenUI/examples/new_gen");
        let b =
            std::path::PathBuf::from("/Users/shengyifei/projects/gen_ui/GenUI/examples/new_gen");
        assert_eq!(a, b);
    }
}
