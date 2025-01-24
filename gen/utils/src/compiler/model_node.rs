use std::path::PathBuf;

use proc_macro2::TokenStream;

use crate::common::Source;

/// # Model Node Impl
/// Model Node is the basic unit of the model tree
/// - each node can be a widget or a rs file or other kinds of file (depends on the project)
/// - each node must has a source, which is the source trace of the node
/// - each node must has a content, which is the content of the node (file content)
pub trait ModelNodeImpl {
    /// ## get mode node source
    fn source(&self) -> Option<&Source>;
    /// ## get content from the model node
    fn content(&self) -> TokenStream;
    /// ## get level from the model node
    /// level is the depth of the node in the model tree
    /// which is used to generate the file path of the node or compare with other nodes
    fn level(&self) -> (usize, PathBuf);
    /// ## compile the model node and write to file
    fn compile(&self) -> ();
}
// : Clone + Hash + PartialEq + Eq
