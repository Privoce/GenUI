use std::{hash::Hash, io::Write, path::PathBuf};

use gen_converter::model::Model;
use gen_utils::{common::fs::create_file, compiler::ModelNodeImpl};
use proc_macro2::TokenStream;

use crate::{
    compiler::{AUTO_BUILTIN_WIDGETS, VIRTUAL_MAP}, widget::model::{auto_builtin_widgets::AutoBuiltinCompile, widget::Widget, ToLiveDesign}, ToToken
};

use super::RsFile;

/// # Model Node
/// Model Node is the basic unit of the model tree which must impl ModelNodeImpl
/// - each node can be a widget or a rs file or other kinds of file (depends on the project)
/// - each node must has a source, which is the source trace of the node
/// - each node must has a content, which is the content of the node (file content)
#[derive(Debug, Clone)]
pub enum ModelNode {
    Widget(Widget),
    RsFile(RsFile),
}

impl PartialEq for ModelNode {
    fn eq(&self, other: &Self) -> bool {
        self.source().unwrap() == other.source().unwrap()
    }
}

impl Eq for ModelNode {}

impl Hash for ModelNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.source().unwrap().hash(state);
    }
}

impl ModelNode {
    pub fn super_ui_root(&self) -> (String, String) {
        match self {
            ModelNode::Widget(widget) => {
                let root = widget
                    .source
                    .as_ref()
                    .expect("first ui root need source")
                    .source_name_lower();
                (root, widget.id.as_ref().unwrap().to_string())
            }
            ModelNode::RsFile(_) => panic!("super ui root not exist in rs file"),
        }
    }
    /// ## 用于判断当前Widget编译后是否需要对AUTO_BUILTIN_WIDGETS进行clear
    /// AUTO_BUILTIN_WIDGETS是一个全局变量，用于存储编译过程中生成的Widget，用于后续的编译
    /// 当前它存储某个Widget中含有if_widget或for_widget的虚拟Widget，如何含有则需要清空，否则不需要
    /// 为了避免重复编译，所以需要清空，而判断依据是当前Widget中has_virtual_widget字段是否为true
    pub fn need_clear_auto_builtins(&self) -> bool {
        match self {
            ModelNode::Widget(widget) => widget.has_virtual_widget,
            ModelNode::RsFile(_) => false,
        }
    }
}

impl ModelNodeImpl for ModelNode {
    fn source(&self) -> Option<&gen_utils::common::Source> {
        match self {
            ModelNode::Widget(widget) => widget.source.as_ref(),
            ModelNode::RsFile(rs) => Some(&rs.source),
        }
    }

    fn content(&self) -> TokenStream {
        match self {
            ModelNode::Widget(widget) => widget.to_live_design().to_token_stream(),
            ModelNode::RsFile(rs) => rs.content(),
        }
    }

    fn level(&self) -> (usize, PathBuf) {
        let path = self.source().unwrap().level_gen();
        (path.components().count(), path)
    }

    fn compile(&self) -> () {
        // get content from the model node --------------------------------------------------------------------
        let content = self.content().to_string();
        // judge need to clear auto builtins ------------------------------------------------------------------
        if self.need_clear_auto_builtins(){
            let mut auto_widgets = AUTO_BUILTIN_WIDGETS.lock().unwrap();
            let mut vmap = VIRTUAL_MAP.lock().unwrap();

            auto_widgets.compile(vmap.as_ref().unwrap().auto_lib_path.join("mod.rs"));
            auto_widgets.clear();
            vmap.as_mut().unwrap().clear_old();
            vmap.as_ref().unwrap().update_app_main_registers();
            vmap.as_mut().unwrap().old.clear();
        
        };
        // write content to file --------------------------------------------------------------------------------
        let mut file = create_file(self.source().unwrap().compiled_file.as_path()).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }
}

impl From<Model> for ModelNode {
    fn from(value: Model) -> Self {
        let source = &value.special;
        match &value.strategy {
            gen_parser::Strategy::None => RsFile::new_empty(source.clone()).into(),
            gen_parser::Strategy::SingleScript => RsFile::from(value).into(),
            gen_parser::Strategy::Error(e) => panic!("{}", e),
            _ => Widget::from(value).into(),
        }
    }
}

impl From<Widget> for ModelNode {
    fn from(value: Widget) -> Self {
        ModelNode::Widget(value)
    }
}

impl From<RsFile> for ModelNode {
    fn from(value: RsFile) -> Self {
        ModelNode::RsFile(value)
    }
}