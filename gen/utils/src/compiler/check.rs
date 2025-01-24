use crate::error::Error;

/// # Checker trait
/// CompilerChecker trait is used to check the compiler status
/// If the environment is not ready, it will return an error
pub trait Checker {
    /// check everything is ready
    fn check(&self) -> Result<(), Error> {
        self.check_env().and_then(|_| self.check_other())
    }
    /// check the environment
    fn check_env(&self) -> Result<(), Error>;
    /// check other
    fn check_other(&self) -> Result<(), Error> {
        Ok(())
    }
}
