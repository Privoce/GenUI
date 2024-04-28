use std::{collections::HashMap, fmt::Display};

use gen_converter::error::Errors;
use gen_parser::{PropsKey, Value};
use proc_macro2::TokenStream;

use crate::{
    prop::{
        builtin::{draw_color::DrawColor, EventOrder, Layout, MouseCursor, ViewOptimize, Walk},
        ABS_POS, ALIGN, BLOCK_SIGNAL_EVENT, CLIP_X, CLIP_Y, CURSOR, DRAW_BG, EVENT_ORDER, FLOW,
        GRAB_KEY_FOCUS, HEIGHT, LINE_SPACING, MARGIN, OPTIMIZE, PADDING, SCROLL, SHOW_BG, SPACING,
        VISIBLE, WIDTH,
    },
    widget::{prop_ignore, utils::bool_prop, StaticProps},
    ToToken,
};

#[derive(Debug, Clone, Default)]
pub struct ViewProps {
    pub draw_bg: Option<DrawColor>,
    pub show_bg: Option<bool>,
    pub layout: Option<Layout>,
    pub walk: Option<Walk>,
    pub optimize: Option<ViewOptimize>,
    pub event_order: Option<EventOrder>,
    pub visible: Option<bool>,
    pub grab_key_focus: Option<bool>,
    pub block_signal_event: Option<bool>,
    pub cursor: Option<MouseCursor>,
}

impl StaticProps for ViewProps {
    fn props(props: &HashMap<PropsKey, Value>) -> Self {
        let mut view = ViewProps::default();
        for (k, v) in props {
            view.prop(k.name(), v.clone())
        }
        view
    }
    fn prop(&mut self, prop_name: &str, value: Value) -> () {
        let _ = match prop_name {
            DRAW_BG => self.draw_bg(&value),
            SHOW_BG => self.show_bg(&value),
            // ----------------- layout -----------------
            SCROLL => self.scroll(&value),
            CLIP_X => self.clip_x(&value),
            CLIP_Y => self.clip_y(&value),
            PADDING => self.padding(&value),
            ALIGN => self.align(&value),
            FLOW => self.flow(&value),
            SPACING => self.spacing(&value),
            LINE_SPACING => self.line_spacing(&value),
            // ----------------- walk -----------------
            HEIGHT => self.height(&value),
            WIDTH => self.width(&value),
            ABS_POS => self.abs_pos(&value),
            MARGIN => self.margin(&value),
            // ----------------- other -----------------
            OPTIMIZE => self.optimize(&value),
            EVENT_ORDER => self.event_order(&value),
            VISIBLE => self.visible(&value),
            GRAB_KEY_FOCUS => self.grab_key_focus(&value),
            BLOCK_SIGNAL_EVENT => self.block_signal_event(&value),
            CURSOR => self.mouse_cursor(&value),
            _ => {
                if !prop_ignore(prop_name) {
                    panic!("cannot match prop");
                } else {
                    panic!("unslolved prop");
                }
            }
        };
    }
}

impl ToToken for ViewProps {
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        self.to_string().parse::<TokenStream>().unwrap()
    }
}

impl ViewProps {
    fn show_bg(&mut self, value: &Value) -> Result<(), Errors> {
        bool_prop(value, |b| {
            self.show_bg = Some(b);
        })
    }
    fn draw_bg(&mut self, value: &Value) -> Result<(), Errors> {
        if let Some(s) = value.is_unknown_and_get() {
            match DrawColor::try_from((s, false)) {
                Ok(color) => {
                    self.draw_bg = Some(color);
                    Ok(())
                }
                Err(_) => Err(Errors::PropConvertFail(format!(
                    "{} can not convert to draw_bg",
                    value
                ))),
            }
        } else {
            value
                .is_string_and_get()
                .map(|s| {
                    if let Ok(color) = DrawColor::try_from(s) {
                        self.draw_bg = Some(color);
                    }
                    Ok(())
                })
                .unwrap_or_else(|| {
                    Err(Errors::PropConvertFail(format!(
                        "{} can not convert to draw_bg",
                        value
                    )))
                })
        }
    }
    fn check_walk(&mut self) -> &mut Walk {
        if self.walk.is_none() {
            self.walk = Some(Walk::default());
        }
        self.walk.as_mut().unwrap()
    }
    fn height(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_walk().height(value)
    }
    fn width(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_walk().width(value)
    }
    fn align(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().align(value)
    }
    fn visible(&mut self, value: &Value) -> Result<(), Errors> {
        bool_prop(value, |b| {
            self.visible = Some(b);
        })
    }
    fn grab_key_focus(&mut self, value: &Value) -> Result<(), Errors> {
        bool_prop(value, |b| {
            self.grab_key_focus = Some(b);
        })
    }
    fn block_signal_event(&mut self, value: &Value) -> Result<(), Errors> {
        bool_prop(value, |b| {
            self.block_signal_event = Some(b);
        })
    }
    fn optimize(&mut self, value: &Value) -> Result<(), Errors> {
        self.optimize = Some(value.try_into()?);
        Ok(())
    }
    fn mouse_cursor(&mut self, value: &Value) -> Result<(), Errors> {
        self.cursor = Some(value.try_into()?);
        Ok(())
    }
    fn event_order(&mut self, value: &Value) -> Result<(), Errors> {
        self.event_order = Some(value.try_into()?);
        Ok(())
    }
    fn abs_pos(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_walk().abs_pos(value)
    }
    fn margin(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_walk().margin(value)
    }
    fn check_layout(&mut self) -> &mut Layout {
        if self.layout.is_none() {
            self.layout = Some(Layout::default());
        }
        self.layout.as_mut().unwrap()
    }
    fn scroll(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().scroll(value)
    }
    fn clip_x(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().clip_x(value)
    }
    fn clip_y(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().clip_y(value)
    }
    fn padding(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().padding(value)
    }
    fn flow(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().flow(value)
    }
    fn spacing(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().spacing(value)
    }
    fn line_spacing(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().line_spacing(value)
    }
}

impl Display for ViewProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(draw_bg) = &self.draw_bg {
            let _ = f.write_fmt(format_args!("draw_bg: {{{}}}, ", draw_bg));
        }
        if let Some(show_bg) = &self.show_bg {
            let _ = f.write_fmt(format_args!("show_bg: {}, ", show_bg));
        }
        if let Some(layout) = &self.layout {
            let _ = f.write_fmt(format_args!("{}, ", layout));
        }
        if let Some(walk) = &self.walk {
            let _ = f.write_fmt(format_args!("{}, ", walk));
        }
        if let Some(optimize) = &self.optimize {
            let _ = f.write_fmt(format_args!("optimize: {}, ", optimize));
        }
        if let Some(event_order) = &self.event_order {
            let _ = f.write_fmt(format_args!("event_order: {}, ", event_order));
        }
        if let Some(visible) = &self.visible {
            let _ = f.write_fmt(format_args!("visible: {}, ", visible));
        }
        if let Some(grab_key_focus) = &self.grab_key_focus {
            let _ = f.write_fmt(format_args!("grab_key_focus: {}, ", grab_key_focus));
        }
        if let Some(block_signal_event) = &self.block_signal_event {
            let _ = f.write_fmt(format_args!("block_signal_event: {}, ", block_signal_event));
        }
        if let Some(cursor) = &self.cursor {
            let _ = f.write_fmt(format_args!("cursor: {}, ", cursor));
        }
        f.write_str("")
    }
}

#[cfg(test)]
mod test_view_props {

    use super::*;
    #[test]
    fn to_tk() {
        let mut view = ViewProps::default();
        view.block_signal_event = Some(true);
        view.draw_bg = Some(DrawColor::default());
        view.show_bg = Some(true);
        let mut layout = Layout::default();
        layout.spacing = Some(10_f64);
        layout.line_spacing = Some(1.5_f64);
        layout.clip_x = Some(true);
        layout.clip_y = Some(false);
        layout.padding = Some("4 10".try_into().unwrap());
        layout.align = Some("0.5 1".try_into().unwrap());
        layout.flow = Some("Down".try_into().unwrap());
        layout.scroll = Some("1 2".try_into().unwrap());
        view.layout = Some(layout);
        let mut walk = super::Walk::default();
        walk.abs_pos = Some("10 10".try_into().unwrap());
        walk.margin = Some("10 10 10 10".try_into().unwrap());
        walk.width = Some("100".try_into().unwrap());
        walk.height = Some("100".try_into().unwrap());
        view.walk = Some(walk);
        view.optimize = Some(ViewOptimize::default());
        view.event_order = Some(EventOrder::default());
        view.visible = Some(true);
        view.grab_key_focus = Some(true);
        view.cursor = Some(MouseCursor::Hand);
        let tk = view.to_token_stream();
        let prop = "draw_bg : { ## 000 } , show_bg : true , scroll : { x : 1 , y : 2 } , clip_x : true , clip_y : false , padding : { top : 10 , right : 4 , bottom : 10 , left : 4 } , align : { x : 0.5 , y : 1 } , flow : Down , spacing : 10 , line_spacing : 1.5 ,, abs_pos : { x : 10 , y : 10 } , margin : { top : 10 , right : 10 , bottom : 10 , left : 10 } , width : 100 , height : 100 ,, optimize : None , event_order : Up , visible : true , grab_key_focus : true , block_signal_event : true , cursor : Hand ,";

        assert_eq!(prop, tk.to_string().as_str());
    }
}