//! 将GenUI的Model转换为Makepad的Model，分为两种形式
//! - 1. AppMain：表示整个应用的入口
//! - 2. Widget：表示一个组件
//! 这两种形式都会包含两个部分：
//! - live_design! 宏编写的DSL模板部分（必须有）
//! - 构建这个模板的代码部分（可能有）
//!  
use core::panic;
use std::fmt::Display;
#[allow(unused_imports)]
use std::{collections::HashMap, default, fmt::Debug};

use gen_converter::model::{script::PropFn, PropTree};
use gen_parser::{PropsKey, Value, BUILTIN_PROPS};
use gen_utils::{common::snake_to_camel, error::Errors};
use proc_macro2::TokenStream;
use syn::{Ident, ItemStruct};

use crate::{str_to_string_try_from, ToToken};

pub mod area;
pub mod button;
pub mod checkbox;
pub mod color_picker;
pub mod desktop_button;
pub mod drop_down;
pub mod fold_button;
pub mod fold_header;
pub mod html;
pub mod icon;
pub mod image;
pub mod label;
pub mod link_label;
pub mod markdown;
pub mod model;
pub mod radio;
pub mod root;
pub mod rotated_image;
pub mod scroll;
pub mod shader;
pub mod slide;
pub mod slider;
pub mod splitter;
pub mod text_input;
pub mod utils;
pub mod view;
pub mod window;
pub mod window_menu;

const WINDOW: &str = "Window";
const VIEW: &str = "View";
const LABEL: &str = "Label";
const BUTTON: &str = "Button";
const AREA: &str = "Area";
/// 表示GenUI的声明的单独的一个组件，不是内置组件
/// 但它会直接认为是Makepad的Area
const COMPONENT: &str = "Component";
const ICON: &str = "Icon";
const IMAGE: &str = "Image";
const RADIO: &str = "RadioButton";
const CHECKBOX: &str = "CheckBox";
const TEXT_INPUT: &str = "TextInput";
const ROOT: &str = "Root";
const SCROLLXVIEW: &str = "ScrollXView";
const SCROLLYVIEW: &str = "ScrollYView";
const SCROLLXYVIEW: &str = "ScrollXYView";
const ROUNDEDVIEW: &str = "RoundedView";
const ROUNDEDSHADOWVIEW: &str = "RoundedShadowView";
const RECTVIEW: &str = "RectView";
const RECTSHADOWVIEW: &str = "RectShadowView";
const SOLIDVIEW: &str = "SolidView";
const DROP_DOWN: &str = "DropDown";
// const SHADER: &str = "Shader";
const LINK_LABEL: &str = "LinkLabel";
const DESKTOP_BUTTON: &str = "DesktopButton";
const SPLITTER: &str = "Splitter";
const ROTATED_IMAGE: &str = "RotatedImage";
const FOLD_BUTTON: &str = "FoldButton";
const FOLD_HEADER: &str = "FoldHeader";
const SLIDER: &str = "Slider";
const SLIDER_BIG: &str = "SliderBig";
const SLIDE: &str = "Slide";
const SLIDES_VIEW: &str = "SlidesView";
const SLIDE_BODY: &str = "SlideBody";
const SLIDE_CHAPTER: &str = "SlideChapter";
const SCROLL_BAR: &str = "ScrollBar";
const SCROLL_BARS: &str = "ScrollBars";
const MARKDOWN: &str = "Markdown";
const HTML: &str = "Html";

/// 判断是否是内置属性， 内置属性需要忽略
/// 事实上，这里只是个防备，因为这些内置属性一般都会在处理之前移除
/// 如果这个函数返回true，说明当前开发者忘记移除内置属性，这是个bug
/// 这会直接panic
pub fn prop_ignore(prop: &str) -> bool {
    BUILTIN_PROPS.contains(&prop)
}

#[derive(Debug, Clone, Default)]
pub enum BuiltIn {
    Window,
    View,
    ScrollXView,
    ScrollYView,
    ScrollXYView,
    SolidView,
    RectView,
    RectShadowView,
    RoundedView,
    RoundedShadowView,
    TextInput,
    Label,
    Button,
    #[default]
    Area,
    Icon,
    Image,
    CheckBox,
    Radio,
    Root,
    DropDown,
    LinkLabel,
    DesktopButton,
    Splitter,
    RotatedImage,
    FoldButton,
    FoldHeader,
    Slider,
    SliderBig,
    SlidesView,
    Slide,
    SlideBody,
    SlideChapter,
    ScrollBar,
    ScrollBars,
    Markdown,
    Html,
}

impl BuiltIn {
    /// 处理内置组件绑定动态属性
    pub fn prop_bind(
        &self,
        prop: &PropsKey,
        value: &Value,
        is_prop: bool,
        ident: &str,
    ) -> TokenStream {
        match self {
            BuiltIn::Window => window::WindowProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::View => view::ViewProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::Label => label::LabelProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::Button => button::ButtonProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::Area => todo!("area do not need to bind prop"),
            BuiltIn::Icon => icon::IconProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::Image => image::ImageProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::CheckBox => checkbox::CheckBoxProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::Radio => radio::RadioButtonProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::Root => root::RootProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::ScrollXView => view::ScrollXViewProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::ScrollYView => view::ScrollYViewProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::ScrollXYView => {
                view::ScrollXYViewProps::prop_bind(prop, value, is_prop, ident)
            }
            BuiltIn::TextInput => {
                text_input::TextInputProps::prop_bind(prop, value, is_prop, ident)
            }
            BuiltIn::SolidView => view::SolidViewProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::RectView => view::RectViewProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::RectShadowView => {
                view::RectShadowViewProps::prop_bind(prop, value, is_prop, ident)
            }
            BuiltIn::RoundedView => view::RoundedViewProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::RoundedShadowView => {
                view::RoundedShadowViewProps::prop_bind(prop, value, is_prop, ident)
            }
            BuiltIn::DropDown => drop_down::DropDownProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::LinkLabel => {
                link_label::LinkLabelProps::prop_bind(prop, value, is_prop, ident)
            }
            BuiltIn::DesktopButton => {
                desktop_button::DesktopButtonProps::prop_bind(prop, value, is_prop, ident)
            }
            BuiltIn::Splitter => splitter::SplitterProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::RotatedImage => {
                rotated_image::RotatedImageProps::prop_bind(prop, value, is_prop, ident)
            }
            BuiltIn::FoldButton => {
                fold_button::FoldButtonProps::prop_bind(prop, value, is_prop, ident)
            }
            BuiltIn::FoldHeader => {
                fold_header::FoldHeaderProps::prop_bind(prop, value, is_prop, ident)
            }
            BuiltIn::Slider => slider::SliderProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::SliderBig => slider::SliderBigProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::SlidesView => slide::SlidesViewProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::Slide => slide::SlideProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::SlideBody => slide::SlideBodyProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::SlideChapter => {
                slide::SlideChapterProps::prop_bind(prop, value, is_prop, ident)
            }
            BuiltIn::ScrollBar => scroll::ScrollBarProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::ScrollBars => scroll::ScrollBarsProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::Markdown => markdown::MarkdownProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::Html => html::HtmlProps::prop_bind(prop, value, is_prop, ident),
        }
    }
    /// 对内置组件的属性进行处理
    pub fn props(&self, props: &HashMap<PropsKey, Value>) -> TokenStream {
        match self {
            BuiltIn::Window => window::WindowProps::props(props).to_token_stream(),
            BuiltIn::View => view::ViewProps::props(props).to_token_stream(),
            BuiltIn::Label => label::LabelProps::props(props).to_token_stream(),
            BuiltIn::Button => button::ButtonProps::props(props).to_token_stream(),
            BuiltIn::Icon => icon::IconProps::props(props).to_token_stream(),
            BuiltIn::Image => image::ImageProps::props(props).to_token_stream(),
            BuiltIn::CheckBox => checkbox::CheckBoxProps::props(props).to_token_stream(),
            BuiltIn::Radio => radio::RadioButtonProps::props(props).to_token_stream(),
            BuiltIn::Root => root::RootProps::props(props).to_token_stream(),
            BuiltIn::ScrollXView => view::ScrollXViewProps::props(props).to_token_stream(),
            BuiltIn::ScrollYView => view::ScrollYViewProps::props(props).to_token_stream(),
            BuiltIn::ScrollXYView => view::ScrollXYViewProps::props(props).to_token_stream(),
            BuiltIn::Area => todo!("area do not need to bind static prop"),
            BuiltIn::TextInput => text_input::TextInputProps::props(props).to_token_stream(),
            BuiltIn::SolidView => view::SolidViewProps::props(props).to_token_stream(),
            BuiltIn::RectView => view::RectViewProps::props(props).to_token_stream(),
            BuiltIn::RectShadowView => view::RectShadowViewProps::props(props).to_token_stream(),
            BuiltIn::RoundedView => view::RoundedViewProps::props(props).to_token_stream(),
            BuiltIn::RoundedShadowView => {
                view::RoundedShadowViewProps::props(props).to_token_stream()
            }
            BuiltIn::DropDown => drop_down::DropDownProps::props(props).to_token_stream(),
            BuiltIn::LinkLabel => link_label::LinkLabelProps::props(props).to_token_stream(),
            BuiltIn::DesktopButton => {
                desktop_button::DesktopButtonProps::props(props).to_token_stream()
            }
            BuiltIn::Splitter => splitter::SplitterProps::props(props).to_token_stream(),
            BuiltIn::RotatedImage => {
                rotated_image::RotatedImageProps::props(props).to_token_stream()
            }
            BuiltIn::FoldButton => fold_button::FoldButtonProps::props(props).to_token_stream(),
            BuiltIn::FoldHeader => fold_header::FoldHeaderProps::props(props).to_token_stream(),
            BuiltIn::Slider => slider::SliderProps::props(props).to_token_stream(),
            BuiltIn::SliderBig => slider::SliderBigProps::props(props).to_token_stream(),
            BuiltIn::SlidesView => slide::SlidesViewProps::props(props).to_token_stream(),
            BuiltIn::Slide => slide::SlideProps::props(props).to_token_stream(),
            BuiltIn::SlideBody => slide::SlideBodyProps::props(props).to_token_stream(),
            BuiltIn::SlideChapter => slide::SlideChapterProps::props(props).to_token_stream(),
            BuiltIn::ScrollBar => scroll::ScrollBarProps::props(props).to_token_stream(),
            BuiltIn::ScrollBars => scroll::ScrollBarsProps::props(props).to_token_stream(),
            BuiltIn::Markdown => markdown::MarkdownProps::props(props).to_token_stream(),
            BuiltIn::Html => html::HtmlProps::props(props).to_token_stream(),
        }
    }
    pub fn to_token_stream(&self, ptr: &ItemStruct) -> TokenStream {
        match self {
            BuiltIn::Window => window::WindowPropPtr::from(ptr).to_token_stream(),
            BuiltIn::View => view::ViewPropPtr::from(ptr).to_token_stream(),
            BuiltIn::Label => label::LabelPropPtr::from(ptr).to_token_stream(),
            BuiltIn::Button => button::ButtonPropPtr::from(ptr).to_token_stream(),
            BuiltIn::Area => area::AreaPropPtr::from(ptr).to_token_stream(),
            BuiltIn::Icon => icon::IconPropPtr::from(ptr).to_token_stream(),
            BuiltIn::Image => image::ImagePropPtr::from(ptr).to_token_stream(),
            BuiltIn::CheckBox => checkbox::CheckBoxPropPtr::from(ptr).to_token_stream(),
            BuiltIn::Radio => radio::RadioButtonPropPtr::from(ptr).to_token_stream(),
            BuiltIn::Root => root::RootPropPtr::from(ptr).to_token_stream(),
            BuiltIn::ScrollXView
            | BuiltIn::ScrollYView
            | BuiltIn::ScrollXYView
            | BuiltIn::SolidView
            | BuiltIn::RectView
            | BuiltIn::RectShadowView
            | BuiltIn::RoundedView
            | BuiltIn::RoundedShadowView
            | BuiltIn::Slide
            | BuiltIn::SlideChapter => {
                panic!("child view can not be inherited you need to inherits View")
            }
            BuiltIn::TextInput => text_input::TextInputPropPtr::from(ptr).to_token_stream(),
            BuiltIn::DropDown => drop_down::DropDownPropPtr::from(ptr).to_token_stream(),
            BuiltIn::LinkLabel => link_label::LinkLabelPropPtr::from(ptr).to_token_stream(),
            BuiltIn::DesktopButton => {
                desktop_button::DesktopButtonPropPtr::from(ptr).to_token_stream()
            }
            BuiltIn::Splitter => splitter::SplitterPropPtr::from(ptr).to_token_stream(),
            BuiltIn::RotatedImage => {
                rotated_image::RotatedImagePropPtr::from(ptr).to_token_stream()
            }
            BuiltIn::FoldButton => fold_button::FoldButtonPropPtr::from(ptr).to_token_stream(),
            BuiltIn::FoldHeader => fold_header::FoldHeaderPropPtr::from(ptr).to_token_stream(),
            BuiltIn::Slider => slider::SliderPropPtr::from(ptr).to_token_stream(),
            BuiltIn::SliderBig => {
                panic!("SliderBig can not be inherited you need to inherits Slider")
            }
            BuiltIn::SlidesView => slide::SlidesViewPropPtr::from(ptr).to_token_stream(),
            BuiltIn::SlideBody => {
                panic!("SlideBody can not be inherited you need to inherits Label")
            }
            BuiltIn::ScrollBar => scroll::ScrollBarPropPtr::from(ptr).to_token_stream(),
            BuiltIn::ScrollBars => scroll::ScrollBarsPropPtr::from(ptr).to_token_stream(),
            BuiltIn::Markdown => markdown::MarkdownPropPtr::from(ptr).to_token_stream(),
            BuiltIn::Html => html::HtmlPropPtr::from(ptr).to_token_stream(),
        }
    }
    pub fn has_event(&self) -> bool {
        match self {
            BuiltIn::Button => true,
            _ => false,
        }
    }
    /// you mut be sure that the value is a built-in widget
    pub fn from(value: &str) -> Self {
        value.try_into().expect(&format!("BuiltIn convert fail, seems the current widget: {} is not a built-in widget", value))
    }
    /// 处理widget的draw_walk绘制函数
    pub fn draw_walk(&self, draw_walk: &Option<TokenStream>) -> TokenStream {
        match self {
            BuiltIn::Window => todo!(),
            BuiltIn::View => view::draw_walk(),
            BuiltIn::Label => todo!(),
            BuiltIn::Button => todo!(),
            BuiltIn::Area => area::draw_walk(draw_walk),
            BuiltIn::Icon => todo!(),
            BuiltIn::Image => todo!(),
            BuiltIn::CheckBox => todo!(),
            BuiltIn::Radio => todo!(),
            BuiltIn::Root => root::draw_walk(),
            BuiltIn::ScrollXView
            | BuiltIn::ScrollYView
            | BuiltIn::ScrollXYView
            | BuiltIn::SolidView
            | BuiltIn::RectView
            | BuiltIn::RectShadowView
            | BuiltIn::RoundedView
            | BuiltIn::RoundedShadowView
            | BuiltIn::Slide
            | BuiltIn::SlideChapter => {
                panic!("child view can not be inherited, so that it can not draw_walk, you need to inherits View")
            }
            BuiltIn::TextInput => todo!(),
            BuiltIn::DropDown => todo!(),
            BuiltIn::LinkLabel => todo!(),
            BuiltIn::DesktopButton => todo!(),
            BuiltIn::Splitter => todo!(),
            BuiltIn::RotatedImage => todo!(),
            BuiltIn::FoldButton => todo!(),
            BuiltIn::FoldHeader => todo!(),
            BuiltIn::Slider => todo!(),
            BuiltIn::SliderBig => panic!("SliderBig can not be inherited, so that it can not draw_walk, you need to inherits Slider"),
            BuiltIn::SlidesView => todo!(),
            BuiltIn::SlideBody => panic!("SlideBody can not be inherited, so that it can not draw_walk, you need to inherits Label"),
            BuiltIn::ScrollBar => todo!(),
            BuiltIn::ScrollBars => todo!(),
            BuiltIn::Markdown => todo!(),
            BuiltIn::Html => todo!(),
        }
    }
    /// 处理widget的事件处理函数
    pub fn handle_event(
        &self,
        event: &Option<Vec<PropFn>>,
        binds: &PropTree,
        instance_name: Option<&Ident>,
        prop_fields: Option<&Vec<Ident>>,
    ) -> TokenStream {
        match self {
            BuiltIn::Window => todo!(),
            BuiltIn::View => view::handle_event(event, binds, instance_name, prop_fields),
            BuiltIn::Label => todo!(),
            BuiltIn::Button => todo!(),
            BuiltIn::Area => area::handle_event(event, binds, instance_name, prop_fields),
            BuiltIn::Icon => todo!(),
            BuiltIn::Image => todo!(),
            BuiltIn::CheckBox => todo!(),
            BuiltIn::Radio => todo!(),
            BuiltIn::Root => root::handle_event(event, binds, instance_name, prop_fields),
            BuiltIn::ScrollXView
            | BuiltIn::ScrollYView
            | BuiltIn::ScrollXYView
            | BuiltIn::SolidView
            | BuiltIn::RectView
            | BuiltIn::RectShadowView
            | BuiltIn::RoundedView
            | BuiltIn::RoundedShadowView
            | BuiltIn::Slide
            | BuiltIn::SlideChapter => {
                panic!("child view can not be inherited, so that it can not handle_event, you need to inherits View")
            }
            BuiltIn::TextInput => todo!(),
            BuiltIn::DropDown => todo!(),
            BuiltIn::LinkLabel => todo!(),
            BuiltIn::DesktopButton => todo!(),
            BuiltIn::Splitter => todo!(),
            BuiltIn::RotatedImage => todo!(),
            BuiltIn::FoldButton => todo!(),
            BuiltIn::FoldHeader => todo!(),
            BuiltIn::Slider => todo!(),
            BuiltIn::SliderBig => panic!("SliderBig can not be inherited, so that it can not handle_event, you need to inherits Slider"),
            BuiltIn::SlidesView => todo!(),
            BuiltIn::SlideBody => panic!("SlideBody can not be inherited, so that it can not handle_event, you need to inherits Label"),
            BuiltIn::ScrollBar => todo!(),
            BuiltIn::ScrollBars => todo!(),
            BuiltIn::Markdown => todo!(),
            BuiltIn::Html => todo!(),
        }
    }
    pub fn animation_applys(&self) -> Vec<&str> {
        match self {
            BuiltIn::Window => todo!(),
            BuiltIn::View => view::ViewProps::animation_applys(),
            BuiltIn::ScrollXView => todo!(),
            BuiltIn::ScrollYView => todo!(),
            BuiltIn::ScrollXYView => todo!(),
            BuiltIn::SolidView => todo!(),
            BuiltIn::RectView => todo!(),
            BuiltIn::RectShadowView => todo!(),
            BuiltIn::RoundedView => todo!(),
            BuiltIn::RoundedShadowView => todo!(),
            BuiltIn::TextInput => todo!(),
            BuiltIn::Label => todo!(),
            BuiltIn::Button => todo!(),
            BuiltIn::Area => todo!(),
            BuiltIn::Icon => todo!(),
            BuiltIn::Image => todo!(),
            BuiltIn::CheckBox => todo!(),
            BuiltIn::Radio => todo!(),
            BuiltIn::Root => todo!(),
            BuiltIn::DropDown => todo!(),
            BuiltIn::LinkLabel => todo!(),
            BuiltIn::DesktopButton => todo!(),
            BuiltIn::Splitter => todo!(),
            BuiltIn::RotatedImage => todo!(),
            BuiltIn::FoldButton => todo!(),
            BuiltIn::FoldHeader => todo!(),
            BuiltIn::Slider => todo!(),
            BuiltIn::SliderBig => todo!(),
            BuiltIn::SlidesView => todo!(),
            BuiltIn::Slide => todo!(),
            BuiltIn::SlideBody => todo!(),
            BuiltIn::SlideChapter => todo!(),
            BuiltIn::ScrollBar => todo!(),
            BuiltIn::ScrollBars => todo!(),
            BuiltIn::Markdown => todo!(),
            BuiltIn::Html => todo!(),
        }
    }
    pub fn default_deref_ptr(&self, name: &str) -> TokenStream {
        match self {
            BuiltIn::Window => todo!(),
            BuiltIn::View => view::ViewPropPtr::deref_struct_ptr(name),
            BuiltIn::ScrollXView => todo!(),
            BuiltIn::ScrollYView => todo!(),
            BuiltIn::ScrollXYView => todo!(),
            BuiltIn::SolidView => todo!(),
            BuiltIn::RectView => todo!(),
            BuiltIn::RectShadowView => todo!(),
            BuiltIn::RoundedView => todo!(),
            BuiltIn::RoundedShadowView => todo!(),
            BuiltIn::TextInput => todo!(),
            BuiltIn::Label => todo!(),
            BuiltIn::Button => todo!(),
            BuiltIn::Area => todo!(),
            BuiltIn::Icon => todo!(),
            BuiltIn::Image => todo!(),
            BuiltIn::CheckBox => todo!(),
            BuiltIn::Radio => todo!(),
            BuiltIn::Root => root::RootPropPtr::deref_struct_ptr(name),
            BuiltIn::DropDown => todo!(),
            BuiltIn::LinkLabel => todo!(),
            BuiltIn::DesktopButton => todo!(),
            BuiltIn::Splitter => todo!(),
            BuiltIn::RotatedImage => todo!(),
            BuiltIn::FoldButton => todo!(),
            BuiltIn::FoldHeader => todo!(),
            BuiltIn::Slider => todo!(),
            BuiltIn::SliderBig => todo!(),
            BuiltIn::SlidesView => todo!(),
            BuiltIn::Slide => todo!(),
            BuiltIn::SlideBody => todo!(),
            BuiltIn::SlideChapter => todo!(),
            BuiltIn::ScrollBar => todo!(),
            BuiltIn::ScrollBars => todo!(),
            BuiltIn::Markdown => todo!(),
            BuiltIn::Html => todo!(),
        }
    }
}

impl TryFrom<&str> for BuiltIn {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let widget_name = snake_to_camel(value);
        match widget_name.as_str() {
            WINDOW => Ok(BuiltIn::Window),
            VIEW => Ok(BuiltIn::View),
            LABEL => Ok(BuiltIn::Label),
            BUTTON => Ok(BuiltIn::Button),
            AREA => Ok(BuiltIn::Area),
            COMPONENT => Ok(BuiltIn::Area),
            ICON => Ok(BuiltIn::Icon),
            IMAGE => Ok(BuiltIn::Image),
            CHECKBOX => Ok(BuiltIn::CheckBox),
            RADIO => Ok(BuiltIn::Radio),
            ROOT => Ok(BuiltIn::Root),
            SCROLLXVIEW => Ok(BuiltIn::ScrollXView),
            SCROLLYVIEW => Ok(BuiltIn::ScrollYView),
            SCROLLXYVIEW => Ok(BuiltIn::ScrollXYView),
            SOLIDVIEW => Ok(BuiltIn::SolidView),
            RECTVIEW => Ok(BuiltIn::RectView),
            RECTSHADOWVIEW => Ok(BuiltIn::RectShadowView),
            ROUNDEDVIEW => Ok(BuiltIn::RoundedView),
            ROUNDEDSHADOWVIEW => Ok(BuiltIn::RoundedShadowView),
            TEXT_INPUT => Ok(BuiltIn::TextInput),
            DROP_DOWN => Ok(BuiltIn::DropDown),
            LINK_LABEL => Ok(BuiltIn::LinkLabel),
            DESKTOP_BUTTON => Ok(BuiltIn::DesktopButton),
            SPLITTER => Ok(BuiltIn::Splitter),
            ROTATED_IMAGE => Ok(BuiltIn::RotatedImage),
            FOLD_BUTTON => Ok(BuiltIn::FoldButton),
            FOLD_HEADER => Ok(BuiltIn::FoldHeader),
            SLIDER => Ok(BuiltIn::Slider),
            SLIDER_BIG => Ok(BuiltIn::SliderBig),
            SLIDES_VIEW => Ok(BuiltIn::SlidesView),
            SLIDE => Ok(BuiltIn::Slide),
            SLIDE_BODY => Ok(BuiltIn::SlideBody),
            SLIDE_CHAPTER => Ok(BuiltIn::SlideChapter),
            SCROLL_BAR => Ok(BuiltIn::ScrollBar),
            SCROLL_BARS => Ok(BuiltIn::ScrollBars),
            MARKDOWN => Ok(BuiltIn::Markdown),
            HTML => Ok(BuiltIn::Html),
            _ => Err(Errors::BuiltInConvertFail),
        }
    }
}

str_to_string_try_from!(BuiltIn);

impl TryFrom<Option<&String>> for BuiltIn {
    type Error = Errors;
    fn try_from(value: Option<&String>) -> Result<Self, Self::Error> {
        if let Some(target) = value {
            target.try_into()
        } else {
            Ok(BuiltIn::Area)
        }
    }
}

impl Display for BuiltIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            BuiltIn::Window => WINDOW,
            BuiltIn::View => VIEW,
            BuiltIn::Label => LABEL,
            BuiltIn::Button => BUTTON,
            BuiltIn::Area => AREA,
            BuiltIn::Icon => ICON,
            BuiltIn::Image => IMAGE,
            BuiltIn::CheckBox => CHECKBOX,
            BuiltIn::Radio => RADIO,
            BuiltIn::Root => ROOT,
            BuiltIn::ScrollXView => SCROLLXVIEW,
            BuiltIn::ScrollYView => SCROLLYVIEW,
            BuiltIn::ScrollXYView => SCROLLXYVIEW,
            BuiltIn::TextInput => TEXT_INPUT,
            BuiltIn::SolidView => SOLIDVIEW,
            BuiltIn::RectView => RECTVIEW,
            BuiltIn::RectShadowView => RECTSHADOWVIEW,
            BuiltIn::RoundedView => ROUNDEDVIEW,
            BuiltIn::RoundedShadowView => ROUNDEDSHADOWVIEW,
            BuiltIn::DropDown => DROP_DOWN,
            BuiltIn::LinkLabel => LINK_LABEL,
            BuiltIn::DesktopButton => DESKTOP_BUTTON,
            BuiltIn::Splitter => SPLITTER,
            BuiltIn::RotatedImage => ROTATED_IMAGE,
            BuiltIn::FoldButton => FOLD_BUTTON,
            BuiltIn::FoldHeader => FOLD_HEADER,
            BuiltIn::Slider => SLIDER,
            BuiltIn::SliderBig => SLIDER_BIG,
            BuiltIn::Slide => SLIDE,
            BuiltIn::SlidesView => SLIDES_VIEW,
            BuiltIn::SlideBody => SLIDE_BODY,
            BuiltIn::SlideChapter => SLIDE_CHAPTER,
            BuiltIn::ScrollBar => SCROLL_BAR,
            BuiltIn::ScrollBars => SCROLL_BARS,
            BuiltIn::Markdown => MARKDOWN,
            BuiltIn::Html => HTML,
        })
    }
}

pub trait StaticProps: Debug + ToToken {
    fn props(props: &HashMap<PropsKey, Value>) -> Self
    where
        Self: Sized;
    /// handle single GenUI prop to makepad prop and bind to struct
    fn prop(&mut self, prop_name: &str, value: &Value) -> ();
    // /// convert GenUI prop (from prop manuel) to makepad prop
    // /// this fn can be used in animation
    // fn prop_convert(&self, prop_name: &str) -> Result<&str, Errors>;
}

pub trait DynProps {
    fn prop_bind(prop: &PropsKey, value: &Value, is_prop: bool, ident: &str) -> TokenStream;
}

pub trait AnimationApplys {
    fn animation_applys() -> Vec<&'static str>;
}
