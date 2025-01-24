use gen_utils::wasm::WasmImpl;
use serde::Deserialize;

const TOOLCHAIN_UNINSTALL: &str = "makepad toolchain not found, please install it!";
const TOOLCHAIN_INSTALL: &str = r#"
    makepad toolchain not found, please install it:
    1. 👍 install from makepad project branch `rik`(recommended): `cargo install --path=./tools/cargo_makepad`
    2. 👎 install from crate.io(not recommended): `cargo install cargo-makepad`
    the more information please visit: https://github.com/makepad/makepad/
"#;

#[derive(Debug, Clone, Deserialize)]
pub struct Wasm {
    /// 是否需要在每次Gen更新后重新编译
    pub fresh: bool,
    /// 默认端口 (默认8010)
    pub port: Option<u16>,
}

impl Default for Wasm {
    fn default() -> Self {
        Self {
            fresh: true,
            port: None,
        }
    }
}

impl WasmImpl for Wasm {
    fn new() -> Self {
        Self::default()
    }
    fn port(&mut self, port: u16) -> &mut Self {
        self.port.replace(port);
        self
    }
    // fn check(&mut self) -> &mut Self {
    //     self.check = true;
    //     self
    // }
    fn no_fresh(&mut self) -> &mut Self {
        self.fresh = false;
        self
    }

    // /// path: project path for makepad
    // fn run<P>(&self, path: P) -> Result<Child, Error>
    // where
    //     P: AsRef<Path>,
    // {
    //     // let mut command = "cargo makepad wasm".to_string();
    //     let mut command = Command::new("cargo");
    //     command.args(["makepad", "wasm"]);
    //     if let Some(port) = self.port {
    //         // command.args(["--port=", &port.to_string()]);
    //         command.arg(format!("--port={}", port));
    //     }
    //     command.args(["run", "-p", "src_gen", "--release"]);
    //     // command.push_str(" run -p src_gen --release");
    //     command
    //         .current_dir(path.as_ref())
    //         .spawn()
    //         .map_err(|e| Error::CommandError(format!("makepad wasm run failed: {}", e)))
    // }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
