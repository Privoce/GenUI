use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use gen_dyn_run::DynProcessor;
use gen_plugin::Token as PluginToken;
use gen_utils::error::Error;
// use rssyin::{makepad::MakepadChainExpand, visitor::chain::VisitorChain};

use crate::model::{AbsWidget, SimpleAppMain};

use super::RouterBuilder;

/// in other: Key: WidgetID, Value: AbsWidget
/// in ctx(define widget poll): Key: WidgetName, Value: AbsWidget
pub type WidgetPoll = HashMap<String, AbsWidget>;

pub struct Context {
    pub app_main: SimpleAppMain,
    /// 额外需要在lib.rs中导入的内容
    pub lib_content: Option<String>,
    // /// 虚拟组件
    // /// 在GenUI中，存在虚拟组件的概念，这个虚拟组件是由编译器生成的，用于帮助GenUI构建出Makepad的组件
    // /// 例如for循环，if判断产生的组件，这些组件都会变编译器生成单独的Makepad组件，然后会替换掉原始语法部分
    // /// ```
    // /// GenUI For：<view :for="item in 5"></view>
    // /// ```
    // /// 例如上面的代码，编译器会并不会直接生成5个view组件，而是生成一个Makepad中使用`Vec<(LiveId, View)>`的组件
    // /// 这个组件会生成到新的文件中，然后在原始的view组件中替换掉
    // pub virtual_components: HashSet<SimpleVirtualWidget>
    /// rssyin访问者链, 用于处理GenUI的脚本 废弃，使用rssyin::ScriptAnalyzer
    // pub sc_visitor_chain: VisitorChain,
    /// 存储了所有自定义组件的池
    pub define_widget_poll: WidgetPoll,
    /// plugins
    pub plugins: Option<HashSet<PluginToken>>,
    pub dyn_processor: Option<DynProcessor>,
    /// routers
    pub router: Option<RouterBuilder>,
    // /// global active router
    // pub active_router: Option<RouterBuilder>
}

impl Default for Context {
    fn default() -> Self {
        Self {
            app_main: Default::default(),
            // sc_visitor_chain: VisitorChain::build(),
            define_widget_poll: Default::default(),
            plugins: None,
            dyn_processor: None,
            lib_content: None,
            router: None,
            // active_router: None
        }
    }
}

impl Context {
    pub fn push_widget(&mut self, key: String, value: AbsWidget) {
        self.define_widget_poll.insert(key, value);
    }
    pub fn load_router<P1, P2>(&mut self, router: P1, from_path: P2) -> Result<(), Error>
    where
        P1: AsRef<Path>,
        P2: AsRef<Path>,
    {
        let router = RouterBuilder::new(router, &from_path)?;
        self.router = Some(router);
        Ok(())
    }
}
