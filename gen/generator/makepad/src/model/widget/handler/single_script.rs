use crate::{compiler::Context, model::Widget};
use gen_analyzer::Script;
use gen_utils::{common::Source, error::Error};

pub fn single_script(
    context: &mut Context,
    source: Source,
    script: Option<Script>,
    is_entry: bool,

) -> Result<Widget, Error> {
    let script = if let Some(sc) = script {
        Some((sc, context).try_into()?)
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
