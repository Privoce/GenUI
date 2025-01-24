use crate::error::Error;

pub type SuccessFn = Box<dyn FnOnce(&str) -> ()>;
pub type FailFn = Box<dyn FnOnce(Error) -> ()>;
pub type IgnoreFn = Box<dyn FnOnce() -> ()>;
/// # Executor
///
/// ## Example
/// ```rust
/// use gen_utils::compiler::Executor;
///
/// // execute auxiliaries for the compiler which use Executor
/// let _ = self.target.execute_auxiliaries(Executor {
///     success: Box::new(|msg| {
///         info(msg);
///     }),
///     fail: Box::new(|e| error(e.to_string().as_str())),
///     ignore: Box::new(|| {
///         ();
///     }),
/// });
///
/// // execute auxiliaries for the compiler which execute Executor
/// impl CompilerImpl for Compiler {
///     fn execute_auxiliaries(&mut self, executor: gen_utils::compiler::Executor) -> () {
///         match self.fresh_wasm() {
///             Ok(success) => {
///                 if success {
///                     executor.success_fn("")
///                 } else {
///                     executor.ignore_fn()
///                 }
///             }
///             Err(e) => executor.fail_fn(e),
///         }
///     }
/// }
/// ``````
pub struct Executor {
    pub success: SuccessFn,
    pub fail: FailFn,
    pub ignore: IgnoreFn,
}

impl Executor {
    pub fn success_fn(self, msg: &str) -> () {
        let execute_fn = self.success;
        execute_fn(msg);
    }
    pub fn fail_fn(self, e: Error) -> () {
        let execute_fn = self.fail;
        execute_fn(e);
    }
    pub fn ignore_fn(self) -> () {
        let execute_fn = self.ignore;
        execute_fn();
    }
}
