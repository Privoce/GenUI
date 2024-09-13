mod register;
pub mod event;

use event::*;
pub use register::register;
use makepad_widgets::*;

use crate::{
    animatie_fn, event_option, ref_event_option, set_event, shader::draw_progress::{DrawGProgress, GProgressType}, themes::Themes, utils::{set_cursor, BoolToF32, ThemeColor}, widget_area, widget_origin_fn
};

live_design! {
    import makepad_draw::shader::std::*;
    GProgressBase = {{GProgress}}{
        animator: {
            hover = {
                default: off
                off = {
                    from: {all: Forward {duration: 0.2}}
                    ease: OutQuad
                    apply: {
                        draw_progress: {hover: 0.0}
                    }
                }
                on = {
                    //cursor: Arrow,
                    from: {all: Snap}
                    apply: {
                        draw_progress: {hover: 1.0}
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
    pub border_color: Option<Vec4>,
    #[live(2.0)]
    pub border_radius: f32,
    #[live(1.0)]
    pub border_width: f32,
    #[live]
    pub stroke_color: Option<Vec4>,
    #[live]
    pub stroke_hover_color: Option<Vec4>,
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
    #[live(false)]
    pub animation_open: bool,
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
}

impl Widget for GProgress {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible{
            return DrawStep::done();
        }
        self.draw_progress.position = self.value as f32;
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
                if self.set_internal(value) {
                    self.redraw(cx)
                }
                // self.update_text_input_and_redraw(cx);
            }
        }
    }
}

impl LiveHook for GProgress {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.visible {
            return;
        }
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.get(self.theme, 100);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.get(self.theme, 100);
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 100);
        // ------------------ stroke color ----------------------------------------------
        let stroke_color = self.stroke_color.get(self.theme, 600);
        // ------------------ stroke hover color ----------------------------------------
        let stroke_hover_color = self.stroke_hover_color.get(self.theme, 500);
        // ------------------ apply to draw_progress ----------------------------------------
        self.draw_progress.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                background_visible: (self.background_visible.to_f32()),
                hover_color: (hover_color),
                border_color: (border_color),
                border_width: (self.border_width),
                border_radius: (self.border_radius),
                stroke_color: (stroke_color),
                stroke_hover_color: (stroke_hover_color),
            },
        );
        self.draw_progress.apply_type(self.progress_type.clone());

        self.draw_progress.redraw(cx);
    }
}
impl GProgress {
    widget_area! {
        area, draw_progress
    }
    event_option! {
        before_move: GProgressEvent::BeforeMove => f64,
        moving: GProgressEvent::Moving => f64,
        after_move: GProgressEvent::AfterMove => f64
    }
    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
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
    /// Convert the internal value to an external value
    fn to_external(&self) -> f64 {
        let val = self.value * (self.max - self.min) + self.min;
        if self.step != 0.0 {
            return (val / self.step).floor() * self.step;
        } else {
            val
        }
    }
    fn set_internal(&mut self, external: f64) -> bool {
        let old = self.value;
        self.value = (external - self.min) / (self.max - self.min);
        old != self.value
    }
    pub fn add(&mut self, v: f64) -> (){
        self.set_internal(self.to_external() + v);
    }
    pub fn sub(&mut self, v: f64) -> (){
        self.set_internal(self.to_external() - v);
    }
    pub fn full(&mut self) -> (){
        self.set_internal(1.0);
    }
    pub fn clear(&mut self) -> (){
        self.set_internal(0.0);
    }
    pub fn handle_widget_event(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
        hit: Hit,
        focus_area: Area,
    ) {
        let uid = self.widget_uid();
        if self.animation_open{
            self.animator_handle_event(cx, event);
        }

        match hit {
            Hit::FingerHoverIn(_) => {
                if !self.read_only {
                    let _ = set_cursor(cx, self.cursor.as_ref());
                }
                self.animator_play(cx, id!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerDown(_fe) => {
                if self.grab_key_focus {
                    cx.set_key_focus(focus_area);
                }
                // self.animator_play(cx, id!(drag.on));
                self.dragging = Some(self.value);
                cx.widget_action(uid, &scope.path, GProgressEvent::BeforeMove(self.value));
            }
            Hit::FingerUp(fe) => {
                // self.text_input.read_only = false;
                // if the finger hasn't moved further than X we jump to edit-all on the text thing
                // self.text_input.create_external_undo();
                // self.animator_play(cx, id!(drag.off));
                if fe.is_over && fe.device.has_hovers() {
                    self.animator_play(cx, id!(hover.on));
                } else {
                    self.animator_play(cx, id!(hover.off));
                }
                self.dragging = None;
                cx.widget_action(uid, &scope.path, GProgressEvent::AfterMove(self.value));
            }
            Hit::FingerMove(fe) => {
                if !self.read_only {
                    match self.progress_type {
                        GProgressType::Horizontal => {
                            let rel = fe.abs - fe.abs_start;
                            if let Some(start_pos) = self.dragging {
                                self.value = (start_pos + rel.x / fe.rect.size.x).max(0.0).min(1.0);
                                self.set_internal(self.to_external());
                                self.draw_progress.redraw(cx);
                                cx.widget_action(
                                    uid,
                                    &scope.path,
                                    GProgressEvent::Moving(self.to_external()),
                                );
                            }
                        }
                        GProgressType::Vertical => {
                            let rel = fe.abs - fe.abs_start;
                            if let Some(start_pos) = self.dragging {
                                // here we need to rev the y
                                self.value = (start_pos - rel.y / fe.rect.size.y).max(0.0).min(1.0);
                                self.set_internal(self.to_external());
                                self.draw_progress.redraw(cx);
                                cx.widget_action(
                                    uid,
                                    &scope.path,
                                    GProgressEvent::Moving(self.to_external()),
                                );
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }
}

impl GProgressRef {
    ref_event_option! {
        before_move => f64,
        moving => f64,
        after_move => f64
    }
    animatie_fn! {
        animate_hover_on,
        animate_hover_off
    }
    widget_origin_fn!(GProgress);
}

impl GProgressSet {
    set_event! {
        before_move => f64,
        moving => f64,
        after_move => f64
    }
}