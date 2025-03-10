use gen_analyzer::Props;
use gen_utils::{
    common::{camel_to_snake, snake_to_camel},
    error::{CompilerError, Error},
};

pub use ty::{BuiltinWidgetType, InheritWidgetType};
use widget::{
    Button, Checkbox, CheckboxGroup, Collapse, Dialog, Divider, Drawer, DropDown, Image, Input, Label, Link, Loading, Popup, PopupContainer, Radio, RadioGroup, Root, ScrollBars, Svg, Tag, Toggle, ToolTip, View, Window
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
    Toggle(Toggle),
    // data
    Tag(Tag),
    Collapse(Collapse),
    Loading(Loading),
    // feedback
    DropDown(DropDown),
    Popup(Popup),
    PopupContainer(PopupContainer),
    Drawer(Drawer),
    Dialog(Dialog),
    ToolTip(ToolTip),
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
                BuiltinWidgetType::Tag => Tag::try_from(props).and_then(|x| Ok(x.into())),
                BuiltinWidgetType::Toggle => Toggle::try_from(props).and_then(|x| Ok(x.into())),
                BuiltinWidgetType::Collapse => Collapse::try_from(props).and_then(|x| Ok(x.into())),
                BuiltinWidgetType::Loading => Loading::try_from(props).and_then(|x| Ok(x.into())),
                BuiltinWidgetType::DropDown => DropDown::try_from(props).and_then(|x| Ok(x.into())),
                BuiltinWidgetType::Popup => Popup::try_from(props).and_then(|x| Ok(x.into())),
                BuiltinWidgetType::PopupContainer => {
                    PopupContainer::try_from(props).and_then(|x| Ok(x.into()))
                }
                BuiltinWidgetType::Dialog => Dialog::try_from(props).and_then(|x| Ok(x.into())),
                BuiltinWidgetType::Drawer => Drawer::try_from(props).and_then(|x| Ok(x.into())),
                BuiltinWidgetType::ToolTip => ToolTip::try_from(props).and_then(|x| Ok(x.into())),
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
            BuiltinWidget::Tag(tag) => tag.name(),
            BuiltinWidget::Toggle(toggle) => toggle.name(),
            BuiltinWidget::Collapse(collapse) => collapse.name(),
            BuiltinWidget::Loading(loading) => loading.name(),
            BuiltinWidget::DropDown(drop_down) => drop_down.name(),
            BuiltinWidget::Popup(popup) => popup.name(),
            BuiltinWidget::PopupContainer(popup_container) => popup_container.name(),
            BuiltinWidget::Dialog(dialog) => dialog.name(),
            BuiltinWidget::Drawer(drawer) => drawer.name(),
            BuiltinWidget::ToolTip(tool_tip) => tool_tip.name(),
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
            BuiltinWidget::Tag(tag) => tag.props(),
            BuiltinWidget::Toggle(toggle) => toggle.props(),
            BuiltinWidget::Collapse(collapse) => collapse.props(),
            BuiltinWidget::Loading(loading) => loading.props(),
            BuiltinWidget::DropDown(drop_down) => drop_down.props(),
            BuiltinWidget::Popup(popup) => popup.props(),
            BuiltinWidget::PopupContainer(popup_container) => popup_container.props(),
            BuiltinWidget::Dialog(dialog) => dialog.props(),
            BuiltinWidget::Drawer(drawer) => drawer.props(),
            BuiltinWidget::ToolTip(tool_tip) => tool_tip.props(),
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
    ScrollBars => BuiltinWidget::ScrollBars,
    Tag => BuiltinWidget::Tag,
    Toggle => BuiltinWidget::Toggle,
    Collapse => BuiltinWidget::Collapse,
    Loading => BuiltinWidget::Loading,
    DropDown => BuiltinWidget::DropDown,
    Popup => BuiltinWidget::Popup,
    PopupContainer => BuiltinWidget::PopupContainer,
    Dialog => BuiltinWidget::Dialog,
    Drawer => BuiltinWidget::Drawer,
    ToolTip => BuiltinWidget::ToolTip
}
