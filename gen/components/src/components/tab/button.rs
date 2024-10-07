use makepad_widgets::*;

use crate::{
    shader::{draw_view::DrawGView, draw_icon_pixel::DrawGIconPixel, draw_tab::DrawTabBtn},
    themes::{get_color, hex_to_vec4, Themes},
    utils::{get_font_family, set_cursor, AbsExt, DefaultTextStyle},
};

live_design! {
    import makepad_draw::shader::std::*;

    GTabButtonBase = {{GTabButton}}{
        spacing: 6.0,
        text_walk: {
            height: Fit,
            width: Fit,
            margin: 0,
        },
        msg_count_walk: {
            height: Fit,
            width: Fit,
            margin: {left: 6.0, top: 0.0, right: 6.0, bottom: 0.0},
        },
        draw_close: {
            fn pixel(self) -> vec4{
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let offset = 1.0;
                sdf.move_to(self.pos.x, self.pos.y);
                sdf.line_to(self.rect_size.x - offset, self.rect_size.y - offset);
                sdf.move_to(self.rect_size.x - offset, self.pos.y);
                sdf.line_to(self.pos.x, self.rect_size.y - offset);
                sdf.stroke(self.color, 1.46);
                return sdf.result;
            }
        },
        icon_walk: {
            height: 9.0,
            width: 9.0,
            margin: 0,
        },
    }
}

#[derive(Live, Widget)]
pub struct GTabButton {
    #[live]
    pub theme: Themes,
    #[live]
    pub background_color: Option<Vec4>,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub pressed_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(0.0)]
    pub border_width: f32,
    #[live(4.0)]
    pub border_radius: f32,
    #[live]
    pub color: Option<Vec4>,
    #[live]
    pub font_family: LiveDependency,
    #[live]
    pub text: ArcStringMut,
    #[live]
    pub selected: bool,
    #[live]
    pub closeable: bool,
    #[live]
    pub plain: bool,
    // icon -------------------
    #[live(1.0)]
    pub icon_brightness: f32,
    #[live(0.6)]
    pub icon_curve: f32,
    #[live(0.5)]
    pub icon_linearize: f32,
    #[live(1.0)]
    pub icon_scale: f64,
    #[live]
    pub icon_color: Option<Vec4>,
    #[live(1.0)]
    pub icon_draw_depth: f32,
    // text -------------------
    #[live(9.0)]
    pub font_size: f64,
    #[live]
    pub show_msg_count: bool,
    #[live(0_u32)]
    pub msg_count: u32,
    #[live]
    pub draw_text: DrawText,
    #[live]
    pub draw_msg_count: DrawText,
    #[live]
    pub draw_msg_wrap: DrawGView,
    #[live]
    pub draw_close: DrawGIconPixel,
    #[redraw]
    #[live]
    pub draw_tab_btn: DrawTabBtn,
    #[walk]
    pub walk: Walk,
    #[live]
    pub text_walk: Walk,
    #[live]
    icon_walk: Walk,
    #[live]
    pub msg_count_walk: Walk,
    #[layout]
    pub layout: Layout,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum GTabButtonEvent {
    HoverIn,
    HoverOut,
    /// The button is selected
    Selected,
    /// The close button was clicked
    Close,
    None,
}

impl Widget for GTabButton {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_tab_btn.begin(cx, walk, self.layout);
        let font = get_font_family(&self.font_family, cx);

        self.draw_text.text_style.font = font.clone();
        self.draw_text.draw_walk(
            cx,
            self.text_walk.with_add_padding(Padding {
                top: 4.0,
                bottom: 4.0,
                ..Default::default()
            }),
            Align::default(),
            self.text.as_ref(),
        );
        if self.show_msg_count {
            self.draw_msg_wrap.begin(
                cx,
                Walk {
                    width: Size::Fit,
                    height: Size::Fit,
                    ..Default::default()
                },
                Layout {
                    align: Align { x: 0.5, y: 0.5 },
                    padding: Padding {
                        top: 2.5,
                        bottom: 2.5,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
            self.draw_msg_count.text_style.font = font;
            self.draw_msg_count.draw_walk(
                cx,
                self.msg_count_walk,
                Align::default(),
                &self.msg_count.to_string(),
            );

            self.draw_msg_wrap.end(cx);
        }

        if self.closeable {
            self.draw_close.draw_walk(cx, self.icon_walk);
        }

        self.draw_tab_btn.end(cx);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        match event.hits(cx, self.draw_tab_btn.area()) {
            Hit::FingerHoverIn(_) => {
                set_cursor(cx, Some(&MouseCursor::Hand));
                cx.widget_action(uid, &scope.path, GTabButtonEvent::HoverIn);
            }
            Hit::FingerHoverOut(f_out) => {
                if f_out
                    .abs
                    .is_in(&self.draw_close.rect_pos, &self.draw_close.rect_size)
                {
                    set_cursor(cx, Some(&MouseCursor::Hand));
                } else {
                    set_cursor(cx, None);
                    cx.widget_action(uid, &scope.path, GTabButtonEvent::HoverOut);
                }
            }
            Hit::FingerUp(f_up) => {
                if f_up
                    .abs
                    .is_in(&self.draw_close.rect_pos, &self.draw_close.rect_size)
                {
                    self.selected = false;
                    cx.widget_action(uid, &scope.path, GTabButtonEvent::Close);
                } else {
                    self.selected = true;
                    cx.widget_action(uid, &scope.path, GTabButtonEvent::Selected);
                }
                self.render(cx);
            }

            _ => {}
        }
    }
    fn text(&self) -> String {
        self.text.as_ref().to_string()
    }
    fn set_text(&mut self, v: &str) {
        self.text.as_mut_empty().push_str(v);
    }
    fn set_text_and_redraw(&mut self, cx: &mut Cx, v: &str) {
        self.text.as_mut_empty().push_str(v);
        self.redraw(cx)
    }
}

impl LiveHook for GTabButton {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        self.render(cx);
    }
}

impl GTabButton {
    pub fn handle_event_actions(
        &self,
        cx: &mut Cx,
        event: &Event,
        action_fn: &mut dyn FnMut(&mut Cx, GTabButtonEvent),
    ) {
        match event.hits(cx, self.draw_tab_btn.area()) {
            Hit::FingerHoverIn(_) => {
                set_cursor(cx, Some(&MouseCursor::Hand));
            }
            Hit::FingerHoverOut(f_out) => {
                if f_out
                    .abs
                    .is_in(&self.draw_close.rect_pos, &self.draw_close.rect_size)
                {
                    set_cursor(cx, Some(&MouseCursor::Hand));
                } else {
                    set_cursor(cx, None);
                }
            }
            Hit::FingerUp(f_up) => {
                if f_up
                    .abs
                    .is_in(&self.draw_close.rect_pos, &self.draw_close.rect_size)
                {
                    // self.selected = false;
                    action_fn(cx, GTabButtonEvent::Close);
                } else {
                    // self.selected = true;
                    action_fn(cx, GTabButtonEvent::Selected);
                }
                // self.render(cx);
            }

            _ => {}
        }
    }
    pub fn area(&self) -> Area {
        self.draw_tab_btn.area()
    }
    pub fn area_close(&self) -> Area {
        self.draw_close.area()
    }
    pub fn render(&mut self, cx: &mut Cx) {
        let plain_color = |origin: u32, or: u32| -> u32 {
            if self.plain {
                or
            } else {
                origin
            }
        };

        let select_color = |c1: Vec4, c2: &str| -> Vec4 {
            if self.selected {
                c1
            } else {
                hex_to_vec4(c2)
            }
        };

        // ------------------ font ------------------------------------------------------
        let font_color = select_color(
            get_color(self.theme, self.color.as_ref(), plain_color(100, 600)),
            "#667085",
        );
        let text_style = DefaultTextStyle::default();
        // ----------------- background color -------------------------------------------
        let bg_color = get_color(self.theme, self.background_color.as_ref(), plain_color(500, 600));
        let msg_bg_color = select_color(
            get_color(self.theme, self.background_color.as_ref(), plain_color(600, 100)),
            "#ECEFF3",
        );
        // ------------------ hover color -----------------------------------------------
        let hover_color = get_color(self.theme, self.hover_color.as_ref(), plain_color(400, 600));
        // ------------------ pressed color ---------------------------------------------
        let pressed_color = get_color(self.theme, self.pressed_color.as_ref(), plain_color(600, 600));
        // ------------------ border color ----------------------------------------------
        let border_color = get_color(self.theme, self.border_color.as_ref(), plain_color(800, 600));
        let icon_color = select_color(
            get_color(self.theme, self.icon_color.as_ref(), plain_color(100, 600)),
            "#667085",
        );

        if self.show_msg_count {
            self.draw_msg_wrap.apply_over(
                cx,
                live! {
                    background_color: (msg_bg_color),
                    background_visible: 1.0,
                    border_radius: ((self.font_size + 8.0)  * 0.25),
                },
            );
            self.draw_msg_count.apply_over(
                cx,
                live! {
                    color: (font_color),
                    text_style: {
                        // brightness: (text_style.brightness),
                        // curve: (text_style.curve),
                        line_spacing: (text_style.line_spacing),
                        // top_drop: (text_style.top_drop),
                        font_size: (self.font_size - 1.0),
                        height_factor: (text_style.height_factor),
                        line_scale: (text_style.line_scale),
                    }
                },
            );
            self.draw_msg_count.redraw(cx);
            self.draw_msg_wrap.redraw(cx);
        }
        self.draw_tab_btn.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                border_color: (border_color),
                border_width: (self.border_width),
                border_radius: (self.border_radius),
                pressed_color: (pressed_color),
                hover_color: (hover_color),
                selected: (self.selected as u32 as f32),
                plain: (self.plain as u32 as f32),
            },
        );
        self.draw_text.apply_over(
            cx,
            live! {
                color: (font_color),
                text_style: {
                    // brightness: (text_style.brightness),
                    // curve: (text_style.curve),
                    line_spacing: (text_style.line_spacing),
                    // top_drop: (text_style.top_drop),
                    font_size: (self.font_size),
                    height_factor: (text_style.height_factor),
                    line_scale: (text_style.line_scale),
                }
            },
        );
        if self.closeable {
            self.draw_close.apply_over(
                cx,
                live! {
                    brightness: (self.icon_brightness),
                    color: (icon_color),
                    curve: (self.icon_curve),
                    draw_depth: (self.icon_draw_depth),
                    linearize: (self.icon_linearize),
                },
            );
            self.draw_close.redraw(cx);
        }
        self.draw_text.redraw(cx);
    }
}

impl GTabButtonRef {
    pub fn as_origin(&self) -> Option<std::cell::Ref<GTabButton>> {
        self.borrow()
    }
    pub fn as_origin_mut(&self) -> Option<std::cell::RefMut<GTabButton>> {
        self.borrow_mut()
    }
}
