use makepad_widgets::*;
use shader::draw_text::TextWrap;

use crate::{
    active_event, animatie_fn, default_handle_animation, default_hit_finger_down, default_hit_finger_up_some, default_hit_hover_in, default_hit_hover_out, event_option, play_animation, ref_area, ref_area_ext, ref_event_option, ref_play_animation, ref_redraw, ref_render, set_event, set_scope_path, set_text_and_visible_fn, shader::{
        draw_split::{DrawGSplit, GSplitType},
        draw_text::DrawGText,
    }, themes::Themes, utils::{get_font_family, set_cursor, ThemeColor}, widget_area, widget_origin_fn
};

use super::event::*;

live_design! {
    import makepad_draw::shader::std::*;
    GLOBAL_DURATION = 0.25,
    GBreadCrumbItemBase = {{GBreadCrumbItem}}{
        spacing: 4.0,
        flow: Right,
        padding: 0,
        text_walk: {
            height: Fit,
            width: Fit
        },
        stroke_walk: {
            height: 14.0,
            width: 14.0,
        }
        draw_item: {
            fn pixel(self) -> vec4{
                return vec4(0.0);
            }
        },
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_text: {focus: 0.0, hover: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                        focus: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_text: {focus: 0.0, hover: 1.0,}
                    }
                }

                focus = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_text: {focus: 1.0, hover: 0.0,}
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct GBreadCrumbItem {
    #[live(Themes::Dark)]
    pub theme: Themes,
    // text -------------------
    #[live]
    pub color: Option<Vec4>,
    #[live]
    pub text_hover_color: Option<Vec4>,
    #[live]
    pub text_focus_color: Option<Vec4>,
    #[live(9.0)]
    pub font_size: f64,
    // #[live(1.0)]
    // pub brightness: f32,
    // #[live(0.5)]
    // pub curve: f32,
    #[live(1.5)]
    pub line_spacing: f64,
    // #[live(0.0)]
    // pub top_drop: f64,
    #[live(0.0)]
    pub height_factor: f64,
    #[live(TextWrap::Line)]
    pub wrap: TextWrap,
    #[live]
    pub font_family: LiveDependency,
    #[live]
    pub text: ArcStringMut,
    #[live]
    pub text_walk: Walk,
    // split -------------------
    #[live(1.0)]
    pub stroke_brightness: f32,
    #[live(0.6)]
    pub stroke_curve: f32,
    #[live(0.5)]
    pub stroke_linearize: f32,
    #[live(1.0)]
    pub stroke_scale: f64,
    #[live]
    pub stroke_color: Option<Vec4>,
    #[live(1.0)]
    pub stroke_draw_depth: f32,
    #[live]
    pub stroke_walk: Walk,
    #[live(GSplitType::Spliter)]
    pub split_type: GSplitType,
    // deref -------------------
    #[live]
    pub draw_text: DrawGText,
    #[live]
    pub draw_split: DrawGSplit,
    #[redraw]
    #[live]
    pub draw_item: DrawQuad,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[live(true)]
    pub grab_key_focus: bool,
    // animator -----------------
    #[live(true)]
    pub animation_key: bool,
    #[animator]
    pub animator: Animator,
    #[live(true)]
    pub visible: bool,
    #[live(Some(MouseCursor::Hand))]
    pub cursor: Option<MouseCursor>,
    #[live(true)]
    pub event_key: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
}

impl Widget for GBreadCrumbItem {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        self.set_scope_path(&scope.path);
        self.draw_item.begin(cx, walk, self.layout);

        self.draw_split.draw_walk(cx, self.stroke_walk);
        let font = get_font_family(&self.font_family, cx);
        self.draw_text.text_style.font = font;
        self.draw_text
            .draw_walk(cx, self.text_walk, Align::default(), self.text.as_ref());

        self.draw_item.end(cx);
        DrawStep::done()
    }
    fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
        sweep_area: Area,
    ) {
        let hit = event.hits_with_options(
            cx,
            self.area(),
            HitOptions::new().with_sweep_area(sweep_area),
        );

        self.handle_widget_event(cx, event, scope, hit, sweep_area)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let focus_area = self.area();
        let hit = event.hits(cx, self.area());
        self.handle_widget_event(cx, event, scope, hit, focus_area)
    }
    set_text_and_visible_fn!();
}

impl LiveHook for GBreadCrumbItem {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.visible {
            return;
        }
        self.render(cx);
    }
}

impl GBreadCrumbItem {
    set_scope_path!();
    play_animation!();
    widget_area! {
        area, draw_item,
        area_split, draw_split,
        area_text, draw_text
    }
    active_event! {
        active_hover_in: GBreadCrumbItemEvent::HoverIn |e: Option<FingerHoverEvent>| => GBreadCrumbItemHoverParam {e},
        active_hover_out: GBreadCrumbItemEvent::HoverOut |e: Option<FingerHoverEvent>| => GBreadCrumbItemHoverParam {e},
        active_focus: GBreadCrumbItemEvent::Focus |e: Option<FingerDownEvent>| => GBreadCrumbItemFocusParam {e},
        active_focus_lost: GBreadCrumbItemEvent::FocusLost |e: Option<FingerUpEvent>| => GBreadCrumbItemFocusLostParam {e}
    }
    pub fn active_clicked(&self, cx: &mut Cx, e: Option<FingerUpEvent>) {
        if self.event_key {
            self.scope_path.as_ref().map(|path| {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    GBreadCrumbItemEvent::Clicked(GBreadCrumbItemClickedParam {
                        text: self.text(),
                        e,
                    }),
                );
            });
        }
    }
    event_option! {
        clicked : GBreadCrumbItemEvent::Clicked => GBreadCrumbItemClickedParam,
        hover_in : GBreadCrumbItemEvent::HoverIn => GBreadCrumbItemHoverParam,
        hover_out : GBreadCrumbItemEvent::HoverOut => GBreadCrumbItemHoverParam,
        focus : GBreadCrumbItemEvent::Focus => GBreadCrumbItemFocusParam,
        focus_lost : GBreadCrumbItemEvent::FocusLost => GBreadCrumbItemFocusLostParam
    }
    pub fn clear_animation(&mut self, cx: &mut Cx) {
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
    }
    pub fn render(&mut self, cx: &mut Cx) {
        // ------------------ text ------------------------------------------------------
        let color = self.color.get(self.theme, 50);
        let text_hover_color = self.text_hover_color.get(self.theme, 25);
        let text_focus_color = self.text_focus_color.get(self.theme, 100);
        // ------------------stroke color -----------------------------------------------
        let stroke_color = self.stroke_color.get(self.theme, 50);

        self.draw_split.apply_over(
            cx,
            live! {
                brightness: (self.stroke_brightness),
                color: (stroke_color),
                curve: (self.stroke_curve),
                draw_depth: (self.stroke_draw_depth),
                linearize: (self.stroke_linearize),
            },
        );
        self.draw_split.apply_split_type(self.split_type.clone());

        self.draw_text.apply_over(
            cx,
            live! {
                color: (color),
                stroke_hover_color: (text_hover_color),
                stroke_focus_color: (text_focus_color),
                text_style: {
                    // brightness: (self.brightness),
                    // curve: (self.curve),
                    line_spacing: (self.line_spacing),
                    // top_drop: (self.top_drop),
                    font_size: (self.font_size),
                    height_factor: (self.height_factor),
                }
            },
        );
        self.draw_text.wrap = self.wrap.clone();
    }
    pub fn redraw(&self, cx: &mut Cx) {
        self.draw_text.redraw(cx);
        self.draw_split.redraw(cx);
        self.draw_item.redraw(cx);
    }
    pub fn handle_widget_event(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        _scope: &mut Scope,
        hit: Hit,
        focus_area: Area,
    ) {
        default_handle_animation!(self, cx, event);
        match hit {
            Hit::FingerDown(e) => {
                default_hit_finger_down!(self, cx, focus_area, Some(e));
            }
            Hit::FingerHoverIn(e) => {
                default_hit_hover_in!(self, cx, Some(e));
            }
            Hit::FingerHoverOut(e) => {
                default_hit_hover_out!(self, cx, Some(e));
            }
            Hit::FingerUp(e) => {
                default_hit_finger_up_some!(self, cx, e);
            }
            _ => (),
        }
    }
    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) -> () {
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
    }
    pub fn animate_focus_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.draw_text.apply_over(
            cx,
            live! {
                focus: 1.0
            },
        );
    }
    pub fn animate_focus_off(&mut self, cx: &mut Cx) -> () {
        self.draw_text.apply_over(
            cx,
            live! {
                focus: 0.0
            },
        );
    }
}

impl GBreadCrumbItemRef {
    ref_area!();
    ref_area_ext! {
        area_split,
        area_text
    }
    ref_render!();
    ref_redraw!();
    widget_origin_fn!(GBreadCrumbItem);
    ref_event_option! {
        clicked => GBreadCrumbItemClickedParam,
        hover_in => GBreadCrumbItemHoverParam,
        hover_out => GBreadCrumbItemHoverParam,
        focus => GBreadCrumbItemFocusParam,
        focus_lost => GBreadCrumbItemFocusLostParam
    }
    animatie_fn! {
        clear_animation,
        animate_hover_on,
        animate_hover_off,
        animate_focus_on,
        animate_focus_off
    }
    ref_play_animation!{
        play_hover_on: id!(hover.on),
        play_hover_off: id!(hover.off),
        play_focus_on: id!(hover.focus),
        play_focus_off: id!(hover.off)
    }
}

impl GBreadCrumbItemSet {
    set_event! {
        clicked => GBreadCrumbItemClickedParam,
        hover_in => GBreadCrumbItemHoverParam,
        hover_out => GBreadCrumbItemHoverParam,
        focus => GBreadCrumbItemFocusParam,
        focus_lost => GBreadCrumbItemFocusLostParam
    }
}
