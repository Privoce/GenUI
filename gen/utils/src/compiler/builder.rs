use std::any::Any;

/// # Builder trait
/// Builder trait is a trait to builder pattern
///
/// **Although there are few methods that need to be implemented uniformly in this trait, it exists as a specification**
pub trait Builder: Any {
    type From;
    type To;
    /// ## From target to builder
    /// consume the target and return the builder
    /// ### Attention
    /// - default use `Self::default()` to create a new builder if the builder has no parent
    /// - if the builder has a parent, you need to implement the method to create a new builder
    fn new(value: Self::From) -> Self;
    /// ## Build the target
    /// consume the builder and return the target
    fn build(self) -> Self::To;   
}
