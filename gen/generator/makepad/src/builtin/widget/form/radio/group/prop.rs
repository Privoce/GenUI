use crate::builtin::prop::{Prop, I32};
use crate::{builtin::widget::ViewProps, from_gen_props, props_to_tokens};
use gen_analyzer::{value::Value, PropKey};
use gen_utils::err_from_to;

#[derive(Debug, Clone)]
pub enum Props {
    Selected(I32),
    DerefWidget(ViewProps),
}

from_gen_props!(Props);

impl TryFrom<(PropKey, Value)> for Props {
    type Error = gen_utils::error::Error;

    fn try_from(value: (PropKey, Value)) -> Result<Self, Self::Error> {
        let name = value.0.name.to_string();
        match name.as_str() {
            "selected" => Ok(Props::Selected(value.1.as_isize()?.into())),
            _ => {
                // handle deref widget
                if let Ok(p) = ViewProps::try_from(value) {
                    return Ok(Props::DerefWidget(p));
                } else {
                    return Err(err_from_to!(
                        "GenUI Props" => &format!("Makepad GRadioGroup Prop, Invalid Prop: {}", name)
                    ));
                }
            }
        }
    }
}

props_to_tokens! {
    Props,
    Props::Selected => selected, false,
    Props::DerefWidget => deref_widget, true
}
