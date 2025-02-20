mod lifecycle;
mod import;
use std::collections::HashMap;

pub use import::{Imports, Import};
use lifecycle::LifeCycle;
use ra_ap_syntax::ast::Struct;

pub struct ScriptBridger {
    imports: Option<Imports>,
    prop: Struct,
    /// default impl
    instance: Vec<String>,
    events: Vec<String>,
    lifecycles: LifeCycle,
    regular_methods: Vec<String>,
    
    // 非追踪部分
    raw_code: String,
    
    // 依赖关系
    // dependencies: DependencyResolver,
}

