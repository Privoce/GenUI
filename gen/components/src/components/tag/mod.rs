pub mod event;
mod register;

use event::{GTagClickedParam, GTagEvent, GTagHoverParam};
pub use register::register;

use crate::shader::draw_icon_pixel::DrawGIconPixel;
use crate::shader::draw_svg::DrawGSvg;
use crate::shader::draw_text::DrawGText;
use crate::utils::{get_font_family, set_cursor, BoolToF32, ThemeColor};
use crate::{animatie_fn, event_option, ref_event_option, set_event, set_text_and_visible_fn, widget_area};
use crate::{shader::draw_view::DrawGView, themes::Themes};
use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;
    GLOBAL_DURATION = 0.25;
    GTagBase = {{GTag}}{
        clip_x: false,
        clip_y: false,
        shadow_offset: vec2(0.0, 2.0),
        height: Fit,
        width: Fit,
        text_walk: {
            height: Fit,
            width: Fit,
        }
        cursor: Hand,
        icon_walk: {
            margin: 0,
        },
        icon_layout: {
            padding: 0,
        },
        draw_close: {
            fn pixel(self) -> vec4{
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.move_to(self.pos.x + 0.5, self.pos.y + 0.5);
                sdf.line_to(self.rect_size.x - 0.5, self.rect_size.y - 0.5);
                sdf.move_to(self.rect_size.x - 0.5, self.pos.y - 0.5);
                sdf.line_to(self.pos.x + 0.5, self.rect_size.y - 0.5);
                sdf.stroke(self.color, 1.2);
                return sdf.result;
            }
        },
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_tag: {hover: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_tag: { hover: 1.0}
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct GTag {
    #[live]
    pub theme: Themes,
    #[live]
    pub background_color: Option<Vec4>,
    #[live(true)]
    pub background_visible: bool,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub stroke_hover_color: Option<Vec4>,
    #[live]
    pub pressed_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(0.0)]
    pub border_width: f32,
    #[live(2.0)]
    pub border_radius: f32,
    #[live]
    pub shadow_color: Option<Vec4>,
    #[live(4.0)]
    pub spread_radius: f32,
    #[live(4.8)]
    pub blur_radius: f32,
    #[live]
    pub shadow_offset: Vec2,
    #[live(false)]
    pub round: bool,
    // text ----------------------------
    #[live]
    pub text: ArcStringMut,
    #[live(10.0)]
    pub font_size: f64,
    #[live]
    pub color: Option<Vec4>,
    #[live]
    pub font_family: LiveDependency,
    // #[live(1.1)]
    // pub top_drop: f64,
    #[live(1.3)]
    pub height_factor: f64,
    #[live(0.88)]
    pub line_scale: f64,
    // icon ----------------------------
    #[live]
    pub cursor: Option<MouseCursor>,
    #[live]
    pub closeable: bool,
    #[live]
    pub src: LiveDependency,
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
    // visible -------------------------
    #[live(true)]
    pub visible: bool,
    // define area ---------------------
    #[live]
    draw_text: DrawGText,
    #[live]
    text_walk: Walk,
    #[live(true)]
    grab_key_focus: bool,
    #[live]
    draw_icon: DrawGSvg,
    #[live]
    draw_close: DrawGIconPixel,
    #[live]
    icon_walk: Walk,
    #[live]
    icon_layout: Layout,
    // deref -----------------
    #[redraw]
    #[live]
    draw_tag: DrawGView,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    // animator -----------------
    #[live(false)]
    pub animation_key: bool,
    #[animator]
    animator: Animator,
    #[live(true)]
    pub event_key: bool,
}

impl Widget for GTag {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let font = get_font_family(&self.font_family, cx);
        self.draw_text.text_style.font = font;

        self.icon_walk.height = Size::Fixed(self.font_size);
        self.icon_walk.width = Size::Fixed(self.font_size);
        // self.text_walk.margin.top = self.font_size / 4.0;
        let _ = self.draw_tag.begin(cx, walk, self.layout);
        let _ = self.draw_icon.draw_walk(cx, self.icon_walk);

        let _ = self
            .draw_text
            .draw_walk(cx, self.text_walk, Align::default(), self.text.as_ref());
        if self.closeable {
            let _ = self.draw_close.draw_walk(cx, self.icon_walk);
        }
        self.draw_tag.end(cx);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();

        if self.animation_key {
            if self.animator_handle_event(cx, event).must_redraw() {
                self.draw_tag.redraw(cx);
            }
        }

        if self.closeable {
            match event.hits(cx, self.area_close()) {
                Hit::FingerUp(f_up) => {
                    if f_up.is_over {
                        cx.widget_action(
                            uid,
                            &scope.path,
                            GTagEvent::Close(self.text.as_ref().to_string()),
                        );
                    }
                }
                _ => {}
            }
        }

        match event.hits(cx, self.area()) {
            Hit::FingerHoverIn(f_in) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                self.animator_play(cx, id!(hover.on));
                cx.widget_action(uid, &scope.path, GTagEvent::Hover(GTagHoverParam{
                    text: self.text.as_ref().to_string(),
                    e: f_in.clone(),
                }));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerUp(f_up) => {
                if f_up.is_over {
                    cx.widget_action(uid, &scope.path, GTagEvent::Clicked(GTagClickedParam{
                        text: self.text.as_ref().to_string(),
                        e: f_up.clone(),
                    }));
                    if f_up.device.has_hovers() {
                        self.animator_play(cx, id!(hover.on));
                    } else {
                        self.animator_play(cx, id!(hover.off));
                    }
                } else {
                    self.animator_play(cx, id!(hover.off));
                }
            }
            _ => (),
        }
    }
    set_text_and_visible_fn!();
}

impl LiveHook for GTag {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.visible {
            return;
        }
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.get(self.theme, 500);
        let shadow_color = self.shadow_color.get(self.theme, 700);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.get(self.theme, 400);
        // ------------------ pressed color ---------------------------------------------
        let pressed_color = self.pressed_color.get(self.theme, 600);
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 800);
        // ------------------ font ------------------------------------------------------
        let font_color = self.color.get(self.theme, 100);
        // ------------------icon color -----------------------------------------------
        let icon_color = self.icon_color.get(self.theme, 100);
        let stroke_hover_color = self.stroke_hover_color.get(self.theme, 50);
        // ------------------ round -----------------------------------------------------

        if self.round {
            self.border_radius = match self.walk.height {
                Size::Fixed(h) => (h * 0.25) as f32,
                Size::Fit => {
                    let mut radius = ((self.draw_text.text_style.font_size
                        + self.walk.margin.top
                        + self.walk.margin.bottom
                        + self.layout.padding.top
                        + self.layout.padding.bottom)
                        * 0.25) as f32;
                    radius += self.border_width;
                    radius += self.font_size as f32 / 8.0 + radius / 10.0;
                    radius
                }
                _ => panic!("round only support fixed and fit"),
            };
        }
        let background_visible = self.background_visible.to_f32();
        self.draw_tag.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                background_visible: (background_visible),
                border_color: (border_color),
                border_width: (self.border_width),
                border_radius: (self.border_radius),
                pressed_color: (pressed_color),
                hover_color: (hover_color),
                shadow_color: (shadow_color),
                shadow_offset: (self.shadow_offset),
                spread_radius: (self.spread_radius),
                blur_radius: (self.blur_radius)
            },
        );
        self.draw_icon.apply_over(
            cx,
            live! {
                stroke_hover_color: (stroke_hover_color),
                color: (icon_color),
                brightness: (self.icon_brightness),
                curve: (self.icon_curve),
                linearize: (self.icon_linearize),
                scale: (self.icon_scale),
                draw_depth: (self.icon_draw_depth),
            },
        );

        self.draw_icon.set_src(self.src.clone());
        self.draw_text.apply_over(
            cx,
            live! {
                color: (font_color),
                text_style: {
                    font_size: (self.font_size),
                    // brightness: (default_text_style.brightness),
                    // curve: (default_text_style.curve),
                    // line_spacing: (self.line_spacing),
                    line_scale: (self.line_scale)
                    // top_drop: (self.top_drop),
                    height_factor: (self.height_factor),
                },
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

        self.draw_tag.redraw(cx);
        self.draw_text.redraw(cx);
        self.draw_icon.redraw(cx);
    }
}

impl GTag {
    widget_area! {
        area, draw_tag,
        area_icon, draw_icon,
        area_text, draw_text
    }
    pub fn area_close(&self) -> Area {
        if self.closeable {
            return self.draw_close.area;
        }
        return Area::Empty;
    }
    event_option! {
        clicked: GTagEvent::Clicked => GTagClickedParam,
        hover: GTagEvent::Hover => GTagHoverParam,
        close: GTagEvent::Close => String
    }
    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
        self.draw_tag.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) -> () {
        self.draw_tag.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
    }
}

impl GTagRef {
    pub fn area(&self) -> Area {
        if let Some(btn_ref) = self.borrow() {
            return btn_ref.draw_tag.area();
        }
        Area::Empty
    }
    ref_event_option! {
        clicked => GTagClickedParam,
        hover => GTagHoverParam,
        close => String
    }
    animatie_fn! {
        animate_hover_on,
        animate_hover_off
    }
}

impl GTagSet {
    set_event! {
        clicked => GTagClickedParam,
        hover => GTagHoverParam,
        close => String
    }
}