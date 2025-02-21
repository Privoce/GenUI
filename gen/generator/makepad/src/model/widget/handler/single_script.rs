use gen_analyzer::Script;
use gen_utils::{common::Source, error::Error};
use syn::parse_str;
use crate::model::Widget;

pub fn single_script(
    source: Source,
    script: Option<Script>,
    is_entry: bool,
) -> Result<Widget, Error> {
    let script = if let Some(sc) = script {
        let content = match sc {
            Script::Rs(code) => {
                parse_str::<syn::File>(&code).map_err(|e| Error::from(e.to_string()))?
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
