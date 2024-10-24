mod event;
mod register;

pub use event::*;
pub use register::register;

use crate::shader::draw_icon_pixel::DrawGIconPixel;
use crate::shader::draw_svg::DrawGSvg;
use crate::shader::draw_text::DrawGText;
use crate::utils::{get_font_family, set_cursor, BoolToF32, RectExp, ThemeColor};
use crate::{
    active_event, animatie_fn, check_event_scope, default_handle_animation,
    default_hit_finger_down, default_hit_hover_in, default_hit_hover_out, event_option,
    play_animation, ref_area, ref_area_ext, ref_event_option, ref_redraw, ref_render, set_event,
    set_scope_path, set_text_and_visible_fn, widget_area,
};
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
                        draw_tag: {hover: 0.0, focus: 0.0},
                        draw_icon: {hover: 0.0, focus: 0.0},
                        draw_text: {hover: 0.0, focus: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)},
                        focus: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_tag: {hover: 1.0, focus: 0.0},
                        draw_icon: {hover: 1.0, focus: 0.0},
                        draw_text: {hover: 1.0, focus: 0.0}
                    }
                }

                focus = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_tag: {hover: 0.0, focus: 1.0},
                        draw_icon: {hover: 0.0, focus: 1.0},
                        draw_text: {hover: 0.0, focus: 1.0}
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
    pub text_hover_color: Option<Vec4>,
    #[live]
    pub focus_color: Option<Vec4>,
    #[live]
    pub stroke_focus_color: Option<Vec4>,
    #[live]
    pub text_focus_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(0.0)]
    pub border_width: f32,
    #[live(2.0)]
    pub border_radius: f32,
    #[live]
    pub shadow_color: Option<Vec4>,
    #[live(0.0)]
    pub spread_radius: f32,
    #[live(4.8)]
    pub blur_radius: f32,
    #[live]
    pub shadow_offset: Vec2,
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
    #[live(true)]
    pub animation_key: bool,
    #[animator]
    animator: Animator,
    #[live(true)]
    pub event_key: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
}

impl Widget for GTag {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        self.set_scope_path(&scope.path);
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
    fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        _scope: &mut Scope,
        sweep_area: Area,
    ) {
        if !self.visible {
            return;
        }
        let hit = event.hits_with_options(
            cx,
            self.area(),
            HitOptions::new().with_sweep_area(sweep_area),
        );

        self.handle_widget_event(cx, event, hit, sweep_area)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if !self.visible {
            return;
        }
        let focus_area = self.area();
        let hit = event.hits(cx, self.area());
        self.handle_widget_event(cx, event, hit, focus_area)
    }
    set_text_and_visible_fn!();
}

impl LiveHook for GTag {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.visible {
            return;
        }
        self.render(cx);
    }
}

impl GTag {
    set_scope_path!();
    play_animation!();
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
    check_event_scope!();
    active_event! {
        active_hover_in: GTagEvent::HoverIn |e: Option<FingerHoverEvent>| => GTagHoverParam {e},
        active_hover_out: GTagEvent::HoverOut |e: Option<FingerHoverEvent>| => GTagHoverParam {e},
        active_focus: GTagEvent::Focus |e: Option<FingerDownEvent>| => GTagFocusParam {e},
        active_focus_lost: GTagEvent::FocusLost |e: Option<FingerUpEvent>| => GTagFocusLostParam {e}
    }
    pub fn active_clicked(&mut self, cx: &mut Cx, e: Option<FingerUpEvent>) {
        self.check_event_scope().map(|path| {
            cx.widget_action(
                self.widget_uid(),
                path,
                GTagEvent::Clicked(GTagClickedParam {
                    text: self.text.as_ref().to_string(),
                    e,
                }),
            );
        });
    }
    pub fn active_closed(&mut self, cx: &mut Cx, e: Option<FingerUpEvent>) {
        self.check_event_scope().map(|path| {
            cx.widget_action(
                self.widget_uid(),
                path,
                GTagEvent::Closed(GTagClosedParam {
                    text: self.text.as_ref().to_string(),
                    e,
                }),
            );
        });
    }
    event_option! {
        hover_in: GTagEvent::HoverIn => GTagHoverParam,
        hover_out: GTagEvent::HoverOut => GTagHoverParam,
        focus: GTagEvent::Focus => GTagFocusParam,
        focus_lost: GTagEvent::FocusLost => GTagFocusLostParam,
        clicked: GTagEvent::Clicked => GTagClickedParam,
        closed: GTagEvent::Closed => GTagClosedParam
    }
    pub fn clear_animation(&mut self, cx: &mut Cx) {
        self.draw_tag.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
        self.draw_icon.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
        // if self.closeable{
        //     self.draw_close.apply_over(cx, live!{
        //         hover: 0.0,
        //         focus: 0.0
        //     });
        // }
    }
    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.draw_tag.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
        self.draw_icon.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
        self.draw_text.apply_over(
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
        self.draw_icon.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
    }
    pub fn animate_focus_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.draw_tag.apply_over(
            cx,
            live! {
                focus: 1.0,
            },
        );
        self.draw_icon.apply_over(
            cx,
            live! {
                focus: 1.0,
            },
        );
        self.draw_text.apply_over(
            cx,
            live! {
                focus: 1.0,
            },
        );
    }
    pub fn animate_focus_off(&mut self, cx: &mut Cx) -> () {
        self.draw_tag.apply_over(
            cx,
            live! {
                focus: 0.0,
            },
        );
        self.draw_icon.apply_over(
            cx,
            live! {
                focus: 0.0,
            },
        );
        self.draw_text.apply_over(
            cx,
            live! {
                focus: 0.0,
            },
        );
    }
    pub fn redraw(&self, cx: &mut Cx) -> () {
        self.draw_text.redraw(cx);
        self.draw_icon.redraw(cx);
        self.draw_close.redraw(cx);
        self.draw_tag.redraw(cx);
    }
    pub fn render(&mut self, cx: &mut Cx) -> () {
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.get(self.theme, 500);
        let shadow_color = self.shadow_color.get(self.theme, 700);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.get(self.theme, 400);
        // ------------------ focus color ---------------------------------------------
        let focus_color = self.focus_color.get(self.theme, 600);
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 800);
        // ------------------ text ------------------------------------------------------
        let text_color = self.color.get(self.theme, 50);
        let text_hover_color = self.text_hover_color.get(self.theme, 25);
        let text_focus_color = self.text_focus_color.get(self.theme, 100);
        // ------------------icon color -----------------------------------------------
        let icon_color = self.icon_color.get(self.theme, 50);
        let stroke_hover_color = self.stroke_hover_color.get(self.theme, 25);
        let stroke_focus_color = self.stroke_focus_color.get(self.theme, 100);
        let background_visible = self.background_visible.to_f32();
        self.draw_tag.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                background_visible: (background_visible),
                border_color: (border_color),
                border_width: (self.border_width),
                border_radius: (self.border_radius),
                focus_color: (focus_color),
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
                stroke_focus_color: (stroke_focus_color),
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
                color: (text_color),
                stroke_hover_color: (text_hover_color),
                stroke_focus_color: (text_focus_color),
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
        }
    }
    pub fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, focus_area: Area) {
        default_handle_animation!(self, cx, event);
        match hit {
            Hit::FingerHoverIn(e) => {
                default_hit_hover_in!(self, cx, Some(e));
            }
            Hit::FingerHoverOut(e) => {
                default_hit_hover_out!(self, cx, Some(e));
            }
            Hit::FingerDown(e) => {
                default_hit_finger_down!(self, cx, focus_area, Some(e));
            }
            Hit::FingerUp(e) => {
                if e.is_over {
                    if e.device.has_hovers() {
                        self.play_animation(cx, id!(hover.on));
                    } else {
                        self.play_animation(cx, id!(hover.off));
                    }
                    // use is in to judge
                    if self.closeable {
                        if self.area_close().rect(cx).is_in_pos(&e.abs) {
                            self.active_closed(cx, Some(e.clone()));
                            return;
                        }
                    }

                    self.active_clicked(cx, Some(e));
                } else {
                    self.play_animation(cx, id!(hover.off));
                    self.active_focus_lost(cx, Some(e));
                }
            }
            _ => (),
        }
    }
    pub fn set_visible(&mut self, cx: &mut Cx, visible: bool) {
        self.visible = visible;
        if visible {
            self.clear_animation(cx);
            self.redraw(cx);
        }
    }
}

impl GTagRef {
    ref_area!();
    ref_redraw!();
    ref_render!();
    ref_area_ext! {
        area_close,
        area_icon,
        area_text
    }
    ref_event_option! {
        clicked => GTagClickedParam,
        hover_in => GTagHoverParam,
        hover_out => GTagHoverParam,
        focus => GTagFocusParam,
        focus_lost => GTagFocusLostParam,
        closed => GTagClosedParam
    }
    animatie_fn! {
        clear_animation,
        animate_hover_on,
        animate_hover_off,
        animate_focus_on,
        animate_focus_off
    }
    pub fn set_visible(&mut self, cx: &mut Cx, visible: bool) {
        if let Some(mut c_ref) = self.borrow_mut() {
            c_ref.set_visible(cx, visible);
        }
    }
}

impl GTagSet {
    set_event! {
        clicked => GTagClickedParam,
        hover_in => GTagHoverParam,
        hover_out => GTagHoverParam,
        focus => GTagFocusParam,
        focus_lost => GTagFocusLostParam,
        closed => GTagClosedParam
    }
}
