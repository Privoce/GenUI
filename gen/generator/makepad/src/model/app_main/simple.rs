use gen_utils::common::Source;

use super::root::RootRef;

#[derive(Debug, Clone, Default)]
pub struct SimpleAppMain {
    /// app main的源文件地址及编译后的地址
    pub source: Option<Source>,
    /// app main的根引用
    pub root_ref: RootRef,
}

impl SimpleAppMain {
    pub fn has_root(&self) -> bool {
        self.root_ref.source.is_some()
    }
}
