use std::{collections::HashMap, fmt::Debug, str::FromStr};

use gen_utils::error::{ConvertError, Error};

use crate::two_way_binding::TwoWayBindImpl;

use super::{
    widget::{
        Button, Checkbox, CheckboxGroup, Divider, Image, Input, Label, Link, Radio, RadioGroup, Root, Svg, Tag, View, WidgetImpl
    },
    BuiltinWidget,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinWidgetType {
    // makepad basic
    Root,
    // basic
    Button,
    Label,
    View,
    Divider,
    Image,
    Svg,
    Link,
    ScrollBars,
    // nav
    Window,
    // form
    Radio,
    RadioGroup,
    Checkbox,
    CheckboxGroup,
    Input,
    // data
    Tag
}

impl BuiltinWidgetType {
    pub fn name(&self) -> &str {
        match self {
            BuiltinWidgetType::Root => "Root",
            BuiltinWidgetType::Button => "GButton",
            BuiltinWidgetType::Label => "GLabel",
            BuiltinWidgetType::View => "GView",
            BuiltinWidgetType::Divider => "GDivider",
            BuiltinWidgetType::Image => "GImage",
            BuiltinWidgetType::Svg => "GSvg",
            BuiltinWidgetType::Link => "GLink",
            BuiltinWidgetType::Window => "GWindow",
            BuiltinWidgetType::Radio => "GRadio",
            BuiltinWidgetType::Checkbox => "GCheckbox",
            BuiltinWidgetType::Input => "GInput",
            BuiltinWidgetType::RadioGroup => "GRadioGroup",
            BuiltinWidgetType::CheckboxGroup => "GCheckboxGroup",
            BuiltinWidgetType::ScrollBars => "GScrollBars",
            BuiltinWidgetType::Tag => "GTag",
        }
    }
    pub fn snake_name(&self) -> &str {
        match self {
            BuiltinWidgetType::Root => "root",
            BuiltinWidgetType::Button => "gbutton",
            BuiltinWidgetType::Label => "glabel",
            BuiltinWidgetType::View => "gview",
            BuiltinWidgetType::Divider => "gdivider",
            BuiltinWidgetType::Image => "gimage",
            BuiltinWidgetType::Svg => "gsvg",
            BuiltinWidgetType::Link => "glink",
            BuiltinWidgetType::Window => "gwindow",
            BuiltinWidgetType::Radio => "gradio",
            BuiltinWidgetType::Checkbox => "gcheckbox",
            BuiltinWidgetType::Input => "ginput",
            BuiltinWidgetType::RadioGroup => "gradio_group",
            BuiltinWidgetType::CheckboxGroup => "gcheckbox_group",
            BuiltinWidgetType::ScrollBars => "gscroll_bars",
            BuiltinWidgetType::Tag => "gtag",
        }
    }
    pub fn event_ty_map(&self) -> Option<HashMap<String, String>> {
        match self {
            BuiltinWidgetType::Root => None,
            BuiltinWidgetType::Button => Button::event_ty_map(),
            BuiltinWidgetType::Label => Label::event_ty_map(),
            BuiltinWidgetType::View => None,
            BuiltinWidgetType::Divider => None,
            BuiltinWidgetType::Image => Image::event_ty_map(),
            BuiltinWidgetType::Svg => Svg::event_ty_map(),
            BuiltinWidgetType::Link => Link::event_ty_map(),
            BuiltinWidgetType::Window => None,
            BuiltinWidgetType::Radio => Radio::event_ty_map(),
            BuiltinWidgetType::Checkbox => Checkbox::event_ty_map(),
            BuiltinWidgetType::Input => Input::event_ty_map(),
            BuiltinWidgetType::RadioGroup => RadioGroup::event_ty_map(),
            BuiltinWidgetType::CheckboxGroup => CheckboxGroup::event_ty_map(),
            BuiltinWidgetType::ScrollBars => None,
            BuiltinWidgetType::Tag => Tag::event_ty_map(),
        }
    }
    pub fn twb_event(&self, prop: &str) -> Option<String> {
        match self {
            BuiltinWidgetType::Root => Root::twb_event(prop),
            BuiltinWidgetType::Button => Button::twb_event(prop),
            BuiltinWidgetType::Label => Label::twb_event(prop),
            BuiltinWidgetType::View => View::twb_event(prop),
            BuiltinWidgetType::Divider => Divider::twb_event(prop),
            BuiltinWidgetType::Image => Image::twb_event(prop),
            BuiltinWidgetType::Svg => Svg::twb_event(prop),
            BuiltinWidgetType::Link => Link::twb_event(prop),
            BuiltinWidgetType::Window => None,
            BuiltinWidgetType::Radio => Radio::twb_event(prop),
            BuiltinWidgetType::Checkbox => Checkbox::twb_event(prop),
            BuiltinWidgetType::Input => Input::twb_event(prop),
            BuiltinWidgetType::RadioGroup => RadioGroup::twb_event(prop),
            BuiltinWidgetType::CheckboxGroup => CheckboxGroup::twb_event(prop),
            BuiltinWidgetType::ScrollBars => None,
            BuiltinWidgetType::Tag => Tag::twb_event(prop),
        }
    }
}

impl From<&BuiltinWidget> for BuiltinWidgetType {
    fn from(value: &BuiltinWidget) -> Self {
        match value {
            BuiltinWidget::Root(_) => BuiltinWidgetType::Root,
            BuiltinWidget::Button(_) => BuiltinWidgetType::Button,
            BuiltinWidget::Label(_) => BuiltinWidgetType::Label,
            BuiltinWidget::View(_) => BuiltinWidgetType::View,
            BuiltinWidget::Divider(_) => BuiltinWidgetType::Divider,
            BuiltinWidget::Image(_) => BuiltinWidgetType::Image,
            BuiltinWidget::Svg(_) => BuiltinWidgetType::Svg,
            BuiltinWidget::Link(_) => BuiltinWidgetType::Link,
            BuiltinWidget::Window(_) => BuiltinWidgetType::Window,
            BuiltinWidget::Radio(_) => BuiltinWidgetType::Radio,
            BuiltinWidget::Checkbox(_) => BuiltinWidgetType::Checkbox,
            BuiltinWidget::Input(_) => BuiltinWidgetType::Input,
            BuiltinWidget::RadioGroup(_) => BuiltinWidgetType::RadioGroup,
            BuiltinWidget::CheckboxGroup(_) => BuiltinWidgetType::CheckboxGroup,
            BuiltinWidget::ScrollBars(_) => BuiltinWidgetType::ScrollBars,
            BuiltinWidget::Tag(_) => BuiltinWidgetType::Tag,
        }
    }
}

impl FromStr for BuiltinWidgetType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "root" => Ok(BuiltinWidgetType::Root),
            "button" => Ok(BuiltinWidgetType::Button),
            "label" => Ok(BuiltinWidgetType::Label),
            "view" => Ok(BuiltinWidgetType::View),
            "divider" => Ok(BuiltinWidgetType::Divider),
            "image" => Ok(BuiltinWidgetType::Image),
            "svg" => Ok(BuiltinWidgetType::Svg),
            "link" => Ok(BuiltinWidgetType::Link),
            "scroll_bars" => Ok(BuiltinWidgetType::ScrollBars),
            "window" => Ok(BuiltinWidgetType::Window),
            "radio" => Ok(BuiltinWidgetType::Radio),
            "radio_group" => Ok(BuiltinWidgetType::RadioGroup),
            "checkbox" => Ok(BuiltinWidgetType::Checkbox),
            "checkbox_group" => Ok(BuiltinWidgetType::CheckboxGroup),
            "input" => Ok(BuiltinWidgetType::Input),
            "tag" => Ok(BuiltinWidgetType::Tag),
            _ => Err(ConvertError::FromTo {
                from: s.to_string(),
                to: "GenUI Builtin Component".to_string(),
            }
            .into()),
        }
    }
}

/// # 可以被继承的组件类型
/// 在Makepad中其实只要是组件都可以继承，但GenUI为了开发者更好的使用，只允许部分组件被继承
/// 这里列出了可以被继承的组件类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InheritWidgetType {
    Root,
    Window,
    #[default]
    View,
}

impl FromStr for InheritWidgetType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "root" => Ok(InheritWidgetType::Root),
            "window" => Ok(InheritWidgetType::Window),
            "view" => Ok(InheritWidgetType::View),
            _ => Err(Error::from(format!(
                "can not convert {} to InheritWidgetType",
                s
            ))),
        }
    }
}
