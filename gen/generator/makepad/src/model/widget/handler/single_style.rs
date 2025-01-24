use gen_converter::ConvertStyle;
use gen_utils::{common::Source, error::Error};

use crate::model::{widget::role::Role, Widget, WidgetTemplate, WidgetType};

pub fn single_style(
    source: Source,
    style: Option<ConvertStyle>,
    is_entry: bool,
) -> Result<Widget, Error> {
    let template = WidgetTemplate {
        id: None,
        is_root: true,
        as_prop: None,
        is_static: true,
        ty: WidgetType::Global(style),
        children: None,
        role: Role::default(),
        binds: None,
    };

    Ok(Widget {
        source,
        template: Some(template),
        script: None,
        is_entry,
        has_plugin: false,
        template_ptrs: None,
    })
}
