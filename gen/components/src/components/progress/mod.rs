pub mod event;
mod register;

use core::panic;

use event::*;
use makepad_widgets::*;
pub use register::register;

use crate::{
    active_event, animatie_fn, default_handle_animation, event_option, play_animation, ref_area,
    ref_event_option, ref_redraw, ref_render, set_event, set_scope_path,
    shader::draw_progress::{DrawGProgress, GProgressType},
    themes::Themes,
    utils::{round_to_two_decimals, set_cursor, BoolToF32, ThemeColor},
    widget_area, widget_origin_fn,
};

live_design! {
    import makepad_draw::shader::std::*;
    GLOBAL_DURATION = 0.25;
    GProgressBase = {{GProgress}}{
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_progress: {hover: 0.0, focus: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)},
                        focus: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_progress: {hover: 1.0, focus: 0.0}
                    }
                }

                focus = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_progress: {hover: 0.0, focus: 1.0}
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct GProgress {
    #[live]
    pub theme: Themes,
    #[live]
    pub background_color: Option<Vec4>,
    #[live(true)]
    pub background_visible: bool,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub focus_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(2.0)]
    pub border_radius: f32,
    #[live(1.0)]
    pub border_width: f32,
    #[live]
    pub stroke_color: Option<Vec4>,
    #[live]
    pub stroke_hover_color: Option<Vec4>,
    #[live]
    pub stroke_focus_color: Option<Vec4>,
    #[live(MouseCursor::Hand)]
    pub cursor: Option<MouseCursor>,
    #[live]
    pub progress_type: GProgressType,
    // deref -------------------
    #[redraw]
    #[live]
    pub draw_progress: DrawGProgress,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[live(true)]
    pub grab_key_focus: bool,
    // visible -------------------
    #[live(true)]
    pub visible: bool,
    // animator -----------------
    #[live(true)]
    pub animation_key: bool,
    #[animator]
    pub animator: Animator,
    #[live(0.0)]
    pub min: f64,
    #[live(1.0)]
    pub max: f64,
    #[live(0.01)]
    pub step: f64,
    #[live(0.0)]
    pub value: f64,
    #[rust]
    pub dragging: Option<f64>,
    #[live(true)]
    pub read_only: bool,
    #[live(true)]
    pub event_key: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
}

impl Widget for GProgress {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        self.set_scope_path(&scope.path);
        // self.draw_progress.position = self.value as f32;
        self.draw_progress.position =
            convert_to_pos(self.value, self.min, self.max, self.step) as f32;
        self.draw_progress.begin(cx, walk, self.layout);
        self.draw_progress.end(cx);
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
    fn is_visible(&self) -> bool {
        self.visible
    }
    // fn widget_to_data(
    //     &self,
    //     _cx: &mut Cx,
    //     actions: &Actions,
    //     nodes: &mut LiveNodeVec,
    //     path: &[LiveId],
    // ) -> bool {
    //     match actions.find_widget_action_cast(self.widget_uid()) {
    //         GProgressEvent::TextSlide(v) | GProgressEvent::Slide(v) => {
    //             nodes.write_field_value(path, LiveValue::Float64(v as f64));
    //             true
    //         }
    //         _ => false,
    //     }
    // }
    fn data_to_widget(&mut self, cx: &mut Cx, nodes: &[LiveNode], path: &[LiveId]) {
        if let Some(value) = nodes.read_field_value(path) {
            if let Some(value) = value.as_float() {
                // if self.set_internal(value) {
                //     self.redraw(cx)
                // }
                self.value = value;
                self.fix();
                self.redraw(cx);
            }
        }
    }
}

impl LiveHook for GProgress {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.visible {
            return;
        }
        self.render(cx);
    }
}
impl GProgress {
    set_scope_path!();
    play_animation!();
    widget_area! {
        area, draw_progress
    }
    active_event! {
        active_hover_in: GProgressEvent::HoverIn |e: Option<FingerHoverEvent>| => GProgressHoverParam {e},
        active_hover_out: GProgressEvent::HoverOut |e: Option<FingerHoverEvent>| => GProgressHoverParam {e},
        active_focus_lost: GProgressEvent::FocusLost |e: Option<FingerUpEvent>| => GProgressFocusLostParam {e}
    }
    event_option! {
        hover_in: GProgressEvent::HoverIn => GProgressHoverParam,
        hover_out: GProgressEvent::HoverOut => GProgressHoverParam,
        before_changed: GProgressEvent::BeforeChanged => GProgressBeforeChangedParam,
        focus_lost: GProgressEvent::FocusLost => GProgressFocusLostParam,
        changed: GProgressEvent::Changed => GProgressChangedParam
    }
    pub fn active_before_changed(&mut self, cx: &mut Cx, e: Option<FingerDownEvent>) {
        if self.event_key {
            self.scope_path.as_ref().map(|path| {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    GProgressEvent::BeforeChanged(GProgressBeforeChangedParam {
                        e,
                        value: self.value,
                        step: self.step,
                        range: [self.min, self.max],
                    }),
                );
            });
        }
    }
    pub fn active_changed(&mut self, cx: &mut Cx, e: Option<FingerUpEvent>) {
        if self.event_key {
            self.scope_path.as_ref().map(|path| {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    GProgressEvent::Changed(GProgressChangedParam {
                        e,
                        value: self.value,
                        step: self.step,
                        range: [self.min, self.max],
                    }),
                );
            });
        }
    }
    pub fn clear_animation(&mut self, cx: &mut Cx) {
        self.draw_progress.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
    }
    pub fn render(&mut self, cx: &mut Cx) -> () {
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.get(self.theme, 100);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.get(self.theme, 100);
        // ------------------ focus color -----------------------------------------------
        let focus_color = self.focus_color.get(self.theme, 100);
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 100);
        // ------------------ stroke color ----------------------------------------------
        let stroke_color = self.stroke_color.get(self.theme, 600);
        // ------------------ stroke hover color ----------------------------------------
        let stroke_hover_color = self.stroke_hover_color.get(self.theme, 500);
        // ------------------ stroke focus color ----------------------------------------
        let stroke_focus_color = self.stroke_focus_color.get(self.theme, 500);
        // ------------------ apply to draw_progress ----------------------------------------
        self.draw_progress.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                background_visible: (self.background_visible.to_f32()),
                hover_color: (hover_color),
                focus_color: (focus_color),
                border_color: (border_color),
                border_width: (self.border_width),
                border_radius: (self.border_radius),
                stroke_color: (stroke_color),
                stroke_hover_color: (stroke_hover_color),
                stroke_focus_color: (stroke_focus_color),
            },
        );
        self.draw_progress.apply_type(self.progress_type.clone());
    }
    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.draw_progress.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) -> () {
        self.draw_progress.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
    }
    pub fn animate_focus_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.draw_progress.apply_over(
            cx,
            live! {
                focus: 1.0,
            },
        );
    }
    pub fn animate_focus_off(&mut self, cx: &mut Cx) -> () {
        self.draw_progress.apply_over(
            cx,
            live! {
                focus: 0.0,
            },
        );
    }
    /// convert the external value to an internal value
    // fn convert_to(v: f64) -> f64{

    // }
    /// Convert the internal value to an external value
    // fn to_external(&self) -> f64 {
    //     let val = self.value * (self.max - self.min) + self.min;
    //     let res = if self.step != 0.0 {
    //         (val / self.step).floor() * self.step
    //     } else {
    //         val
    //     };
    //     return round_to_two_decimals(res);
    // }
    // fn set_internal(&mut self, external: f64) -> bool {
    //     let old = self.value;
    //     self.value = round_to_two_decimals((external - self.min) / (self.max - self.min));
    //     old != self.value
    // }
    pub fn get(&self) -> f64 {
        self.value
    }
    pub fn percent(&self) -> f64 {
        // here we need to convert the value to external percent
        convert_to_pos(self.value, self.min, self.max, self.step) * 100.0
    }
    fn fix(&mut self) {
        if self.value < self.min {
            self.value = self.min;
        } else if self.value > self.max {
            self.value = self.max;
        }
    }
    pub fn set(&mut self, v: f64, cx: &mut Cx) -> () {
        self.value = v;
        self.fix();
        self.redraw(cx);
    }
    pub fn set_percent(&mut self, v: f64, cx: &mut Cx) -> () {
        let v = convert_to_value(v, self.min, self.max);
        self.value = v;
        self.fix();
        self.redraw(cx);
    }
    pub fn add(&mut self, v: f64, cx: &mut Cx) -> () {
        if self.value == self.max {
            return;
        }
        self.value += v;
        self.fix();
        // self.set_internal(self.to_external() + v);
        self.redraw(cx);
    }
    /// add percent, v is a percent value [0.0, 1.0] ([0%, 100%])
    pub fn add_percent(&mut self, v: f64, cx: &mut Cx) -> () {
        if self.value == self.max {
            return;
        }
        let v = convert_to_value(v, self.min, self.max);
        self.value += v;
        self.fix();
        self.redraw(cx);
    }
    pub fn sub(&mut self, v: f64, cx: &mut Cx) -> () {
        if self.value == self.min {
            return;
        }
        self.value -= v;
        self.fix();
        // self.set_internal(self.to_external() - v);
        self.redraw(cx);
    }
    pub fn sub_percent(&mut self, v: f64, cx: &mut Cx) -> () {
        if self.value == self.min {
            return;
        }
        let v = convert_to_value(v, self.min, self.max);
        self.value -= v;
        self.fix();
        self.redraw(cx);
    }
    pub fn full(&mut self, cx: &mut Cx) -> () {
        // self.set_internal(self.max);
        self.value = self.max;
        self.redraw(cx);
    }
    pub fn clear(&mut self, cx: &mut Cx) -> () {
        // self.set_internal(0.0);
        self.value = self.min;
        self.redraw(cx);
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
            Hit::FingerHoverIn(e) => {
                if !self.read_only {
                    let _ = set_cursor(cx, self.cursor.as_ref());
                }
                self.play_animation(cx, id!(hover.on));
                self.active_hover_in(cx, Some(e));
            }
            Hit::FingerHoverOut(e) => {
                self.play_animation(cx, id!(hover.off));
                self.active_hover_out(cx, Some(e));
            }
            Hit::FingerDown(e) => {
                if self.grab_key_focus {
                    cx.set_key_focus(focus_area);
                }
                self.dragging = Some(self.value);
                self.play_animation(cx, id!(hover.focus));
                self.active_before_changed(cx, Some(e));
            }
            Hit::FingerUp(e) => {
                if e.is_over {
                    if e.device.has_hovers() {
                        self.play_animation(cx, id!(hover.on));
                    } else {
                        self.play_animation(cx, id!(hover.off));
                    }
                    self.active_changed(cx, Some(e));
                } else {
                    self.dragging = None;
                    self.play_animation(cx, id!(hover.off));
                    self.active_focus_lost(cx, Some(e));
                }
            }
            Hit::FingerMove(e) => {
                if !self.read_only {
                    match self.progress_type {
                        GProgressType::Horizontal => {
                            let real_len = e.abs.x - e.rect.pos.x;
                            // percentage
                            let mut v = real_len / e.rect.size.x;
                            if v < 0.0 {
                                v = 0.0;
                            } else if v > 1.0 {
                                v = 1.0;
                            }
                            self.value = convert_to_value(v, self.min, self.max);
                            self.redraw(cx);
                        }
                        GProgressType::Vertical => {
                            let real_len = e.abs.y - e.rect.pos.y;
                            // percentage
                            let mut v = real_len / e.rect.size.y;
                            if v < 0.0 {
                                v = 0.0;
                            } else if v > 1.0 {
                                v = 1.0;
                            }
                            self.value = convert_to_value(v, self.min, self.max);
                            self.redraw(cx);
                        }
                    }
                }
            }
            _ => (),
        }
    }
    pub fn redraw(&self, cx: &mut Cx) {
        self.draw_progress.redraw(cx);
    }
}

impl GProgressRef {
    ref_redraw!();
    ref_area!();
    ref_render!();
    ref_event_option! {
        hover_in => GProgressHoverParam,
        hover_out => GProgressHoverParam,
        before_changed => GProgressBeforeChangedParam,
        focus_lost => GProgressFocusLostParam,
        changed => GProgressChangedParam
    }
    animatie_fn! {
        animate_hover_on,
        animate_hover_off,
        animate_focus_on,
        animate_focus_off
    }
    widget_origin_fn!(GProgress);
    pub fn get(&self) -> Option<f64> {
        self.borrow().map(|x| x.get())
    }
    pub fn clear(&mut self, cx: &mut Cx) -> () {
        let _ = self.borrow_mut().map(|mut x| x.clear(cx));
    }
    pub fn full(&mut self, cx: &mut Cx) -> () {
        let _ = self.borrow_mut().map(|mut x| x.full(cx));
    }
    pub fn add(&mut self, v: f64, cx: &mut Cx) -> () {
        let _ = self.borrow_mut().map(|mut x| x.add(v, cx));
    }
    pub fn sub(&mut self, v: f64, cx: &mut Cx) -> () {
        let _ = self.borrow_mut().map(|mut x| x.sub(v, cx));
    }
    pub fn percent(&self) -> Option<f64> {
        self.borrow().map(|x| x.percent())
    }
    pub fn add_percent(&mut self, v: f64, cx: &mut Cx) -> () {
        let _ = self.borrow_mut().map(|mut x| x.add_percent(v, cx));
    }
    pub fn sub_percent(&mut self, v: f64, cx: &mut Cx) -> () {
        let _ = self.borrow_mut().map(|mut x| x.sub_percent(v, cx));
    }
    pub fn set(&mut self, v: f64, cx: &mut Cx) -> () {
        let _ = self.borrow_mut().map(|mut x| x.set(v, cx));
    }
    pub fn set_percent(&mut self, v: f64, cx: &mut Cx) -> () {
        let _ = self.borrow_mut().map(|mut x| x.set_percent(v, cx));
    }
}

impl GProgressSet {
    set_event! {
        hover_in => GProgressHoverParam,
        hover_out => GProgressHoverParam,
        before_changed => GProgressBeforeChangedParam,
        focus_lost => GProgressFocusLostParam,
        changed => GProgressChangedParam
    }
}

/// convert value to position
/// position range: [0.0, 1.0]
/// value range: [min, max]
fn convert_to_pos(v: f64, min: f64, max: f64, step: f64) -> f64 {
    if v < min || v > max {
        panic!("value out of range");
    }
    let v = round_to_two_decimals(v);
    // fix step
    let v = if v % step != 0.0 {
        (v / step).floor() * step
    } else {
        v
    };

    round_to_two_decimals((v - min) / (max - min))
}

/// convert position to value
fn convert_to_value(pos: f64, min: f64, max: f64) -> f64 {
    if pos < 0.0 || pos > 1.0 {
        panic!("position out of range");
    }
    round_to_two_decimals(pos * (max - min) + min)
}
