use gen_parser::Props;
use gen_utils::{
    common::{camel_to_snake, snake_to_camel},
    error::{CompilerError, Error},
};

pub use ty::{BuiltinWidgetType, InheritWidgetType};
use widget::{
    Button, Checkbox, CheckboxGroup, Divider, Image, Input, Label, Link, Radio, RadioGroup, Root,
    ScrollBars, Svg, View, Window,
};

use crate::{from_builtin_widget, token::ToLiveDesign};

pub mod event;
// mod handler;
mod macros;
pub mod prop;
mod shader;
mod ty;
pub mod widget;

#[derive(Debug, Clone)]
pub enum BuiltinWidget {
    // makepad basic
    Root(Root),
    // basic
    Button(Button),
    Label(Label),
    View(View),
    Divider(Divider),
    Image(Image),
    Svg(Svg),
    Link(Link),
    ScrollBars(ScrollBars),
    Window(Window),
    // form
    Radio(Radio),
    RadioGroup(RadioGroup),
    Checkbox(Checkbox),
    CheckboxGroup(CheckboxGroup),
    Input(Input),
}

impl BuiltinWidget {
    /// 获取组件的双向绑定的事件
    pub fn twb_event(widget: &str, prop: &str) -> Option<String> {
        if let Ok(widget) = BuiltinWidget::is_built_in(widget) {
            widget.twb_event(prop)
        } else {
            None
        }
    }

    pub fn is_built_in(widget: &str) -> Result<BuiltinWidgetType, Error> {
        widget.parse()
    }
    /// 判断是否是继承组件
    pub fn is_inherits(widget: &str) -> Result<InheritWidgetType, Error> {
        widget.parse()
    }
    pub fn snake_name(&self) -> String {
        BuiltinWidgetType::from(self).snake_name().to_string()
    }
    pub fn builtin_name_or_snake(name: &str) -> String {
        if let Ok(builtin) = BuiltinWidget::is_built_in(name) {
            builtin.snake_name().to_string()
        } else {
            camel_to_snake(name)
        }
    }
    pub fn builtin_ref_or_camel(name: &str) -> String {
        BuiltinWidget::is_built_in(name).map_or_else(
            |_| format!("{}Ref", snake_to_camel(name)),
            |builtin| format!("{}Ref", builtin.name()),
        )
    }
}

/// from: (name, props, , is_root)
impl TryFrom<(String, Option<Props>, bool)> for BuiltinWidget {
    type Error = Error;

    fn try_from(value: (String, Option<Props>, bool)) -> Result<Self, Self::Error> {
        let (name, props, is_root) = value;
        if is_root {
            Self::is_inherits(&name).map_or_else(
                |e| {
                    let is_built_in = Self::is_built_in(&name).is_ok();
                    if is_built_in {
                        Err(CompilerError::runtime(
                            "Makepad Compiler - Template",
                            &format!(
                                "{} can not be root component, only root, window, view can be root",
                                name
                            ),
                        )
                        .into())
                    } else {
                        Err(e)
                    }
                },
                |w| match w {
                    InheritWidgetType::Root => Root::try_from(props).and_then(|x| Ok(x.into())),
                    InheritWidgetType::Window => Window::try_from(props).and_then(|x| Ok(x.into())),
                    InheritWidgetType::View => View::try_from(props).and_then(|x| Ok(x.into())),
                },
            )
        } else {
            Self::is_built_in(&name).and_then(|w| match w {
                BuiltinWidgetType::Root => Err(Error::from("Root widget must be root")),
                BuiltinWidgetType::Button => Button::try_from(props).and_then(|x| Ok(x.into())),
                BuiltinWidgetType::Label => Label::try_from(props).and_then(|x| Ok(x.into())),
                BuiltinWidgetType::View => View::try_from(props).and_then(|x| Ok(x.into())),
                BuiltinWidgetType::Divider => Divider::try_from(props).and_then(|x| Ok(x.into())),
                BuiltinWidgetType::Image => Image::try_from(props).and_then(|x| Ok(x.into())),
                BuiltinWidgetType::Svg => Svg::try_from(props).and_then(|x| Ok(x.into())),
                BuiltinWidgetType::Link => Link::try_from(props).and_then(|x| Ok(x.into())),
                BuiltinWidgetType::Window => Window::try_from(props).and_then(|x| Ok(x.into())),
                BuiltinWidgetType::Radio => Radio::try_from(props).and_then(|x| Ok(x.into())),
                BuiltinWidgetType::Checkbox => Checkbox::try_from(props).and_then(|x| Ok(x.into())),
                BuiltinWidgetType::Input => Input::try_from(props).and_then(|x| Ok(x.into())),
                BuiltinWidgetType::RadioGroup => {
                    RadioGroup::try_from(props).and_then(|x| Ok(x.into()))
                }
                BuiltinWidgetType::CheckboxGroup => {
                    CheckboxGroup::try_from(props).and_then(|x| Ok(x.into()))
                }
                BuiltinWidgetType::ScrollBars => {
                    ScrollBars::try_from(props).and_then(|x| Ok(x.into()))
                }
            })
        }
    }
}

impl ToLiveDesign for BuiltinWidget {
    fn name(&self) -> proc_macro2::TokenStream {
        match self {
            BuiltinWidget::Root(root) => root.name(),
            BuiltinWidget::Button(button) => button.name(),
            BuiltinWidget::Label(label) => label.name(),
            BuiltinWidget::View(view) => view.name(),
            BuiltinWidget::Divider(divider) => divider.name(),
            BuiltinWidget::Image(img) => img.name(),
            BuiltinWidget::Svg(svg) => svg.name(),
            BuiltinWidget::Link(link) => link.name(),
            BuiltinWidget::Window(window) => window.name(),
            BuiltinWidget::Radio(radio) => radio.name(),
            BuiltinWidget::RadioGroup(radio_group) => radio_group.name(),
            BuiltinWidget::Checkbox(checkbox) => checkbox.name(),
            BuiltinWidget::CheckboxGroup(checkbox_group) => checkbox_group.name(),
            BuiltinWidget::Input(input) => input.name(),
            BuiltinWidget::ScrollBars(scroll_bars) => scroll_bars.name(),
        }
    }

    fn props(&self) -> Option<proc_macro2::TokenStream> {
        match self {
            BuiltinWidget::Root(root) => root.props(),
            BuiltinWidget::Button(button) => button.props(),
            BuiltinWidget::Label(label) => label.props(),
            BuiltinWidget::View(view) => view.props(),
            BuiltinWidget::Divider(divider) => divider.props(),
            BuiltinWidget::Image(img) => img.props(),
            BuiltinWidget::Svg(svg) => svg.props(),
            BuiltinWidget::Link(link) => link.props(),
            BuiltinWidget::Window(window) => window.props(),
            BuiltinWidget::Radio(radio) => radio.props(),
            BuiltinWidget::RadioGroup(radio_group) => radio_group.props(),
            BuiltinWidget::Checkbox(checkbox) => checkbox.props(),
            BuiltinWidget::CheckboxGroup(checkbox_group) => checkbox_group.props(),
            BuiltinWidget::Input(input) => input.props(),
            BuiltinWidget::ScrollBars(scroll_bars) => scroll_bars.props(),
        }
    }
}

from_builtin_widget! {
    Root => BuiltinWidget::Root,
    Window => BuiltinWidget::Window,
    View => BuiltinWidget::View,
    Label => BuiltinWidget::Label,
    Button => BuiltinWidget::Button,
    Divider => BuiltinWidget::Divider,
    Image => BuiltinWidget::Image,
    Svg => BuiltinWidget::Svg,
    Link => BuiltinWidget::Link,
    Radio => BuiltinWidget::Radio,
    RadioGroup => BuiltinWidget::RadioGroup,
    Checkbox => BuiltinWidget::Checkbox,
    CheckboxGroup => BuiltinWidget::CheckboxGroup,
    Input => BuiltinWidget::Input,
    ScrollBars => BuiltinWidget::ScrollBars
}
