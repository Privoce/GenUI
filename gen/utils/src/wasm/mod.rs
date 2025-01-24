use std::any::Any;

/// # WasmImpl trait
/// WasmImpl trait is used to handle the wasm stratege
pub trait WasmImpl: Any + Clone {
    /// ## create a new wasm instance
    /// default is `WasmImpl::default()` which means the struct must implement the `Default` trait
    fn new() -> Self
    where
        Self: Default,
    {
        Self::default()
    }
    /// ## set the wasm port
    /// set the wasm port which is used to run the wasm server
    ///
    /// Generally speaking, wasm server have default port policies, but if the port is occupied, you can set the port by yourself
    fn port(&mut self, port: u16) -> &mut Self;
    // /// ## check the wasm server or others
    // /// - This is not fixed which means you need to do some check to make sure the wasm server can run
    // /// - For example, you can check the wasm server is installed or not
    // fn check(&mut self) -> &mut Self;
    /// ## close fresh the wasm server
    /// Generally speaking, the wasm server is not fresh which means you need to recompile the wasm file
    fn no_fresh(&mut self) -> &mut Self;
    // /// ## check the wasm is installed or not or others
    // fn check_wasm(&self) -> Result<bool, Error>;
    // /// ## run the wasm server
    // /// run the wasm server and return the `Child` which is used to handle the process
    // /// ### Also
    // /// - You can use the `Child` to kill the process
    // /// - You can use the `Child` to get the process id, status, output and so on
    // /// - You can use fresh to recompile the wasm file
    // fn run<P>(&self, path: P) -> Result<Child, Error>
    // where
    //     P: AsRef<Path>;
    /// ## make the struct to `Any`
    /// - This is used to make the struct to `Any` trait
    /// means you can do some dynamic type handle
    fn as_any(&self) -> &dyn Any;
    /// ## make the struct to `Any + mut`
    /// more info see `as_any()`
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
