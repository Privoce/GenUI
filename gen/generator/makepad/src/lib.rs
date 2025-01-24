/// makepad compiler
pub mod compiler;
/// makepad builtin widgets and structs
pub mod builtin;
/// makepad model, include AppMain, Widget and Virtual
pub mod model;
/// needed traits for makepad
pub mod traits;
/// makepad props from GenUI
pub mod from_gen;
/// do token stream and makepad live design
pub mod token;
pub mod visitor;
pub mod two_way_binding;
pub mod script;