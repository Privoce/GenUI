use gen_parser::Script;
use gen_utils::{common::Source, error::Error};
use rssyin::visitor::chain::traits::ChainVisitor;

use crate::{compiler::Context, model::Widget};

pub fn single_script(
    context: &mut Context,
    source: Source,
    script: Option<Script>,
    is_entry: bool,
) -> Result<Widget, Error> {
    // 在所有的底层框架中，Rust Script都需要引入rssyin这个GenUI的独有的脚本转化库进行处理
    let script = if let Some(sc) = script {
        let content = match sc {
            Script::Rs(block) => {
                let _ = context.sc_visitor_chain.visit_block_with(&block);
                let bridge_sc = context.sc_visitor_chain.bridge.clone();
                // clear bridge
                let _ = context.sc_visitor_chain.bridge.clear();
                bridge_sc
            }
            Script::Other { lang, code } => unimplemented!("{}: {}", lang, code),
        };

        Some(content.into())
    } else {
        None
    };

    Ok(Widget {
        source,
        template: None,
        script,
        is_entry,
        has_plugin: false,
        template_ptrs: None,
    })
}
