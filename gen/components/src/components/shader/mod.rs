pub mod event;
mod register;

use event::*;
use makepad_widgets::*;
pub use register::register;

use crate::{
    event_bool, ref_area, ref_event_bool, ref_redraw, set_event_bool, set_scope_path, shader::draw_shader::DrawGShader, utils::BoolToF32, widget_area
};

live_design! {
    GShaderBase = {{GShader}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Live, Widget)]
pub struct GShader {
    #[redraw]
    #[live]
    pub draw_shader: DrawGShader,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[live]
    pub time: f32,
    #[rust]
    next_frame: NextFrame,
    // store previous state(animation_key)
    #[rust]
    pub pre_state: bool,
    #[live(true)]
    pub visible: bool,
    #[live(true)]
    pub animation_key: bool,
    #[live(true)]
    pub event_key: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>
}

impl LiveHook for GShader {
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        // starts the animation cycle on startup
        if self.animation_key {
            self.next_frame = cx.new_next_frame();
        }
    }
    fn after_apply(
        &mut self,
        _cx: &mut Cx,
        _apply: &mut Apply,
        _index: usize,
        _nodes: &[LiveNode],
    ) {
        self.pre_state = self.animation_key;
        if !self.visible {
            return;
        }
        self.draw_shader.opened = self.animation_key.to_f32();
    }
    fn after_update_from_doc(&mut self, cx: &mut Cx) {
        if self.pre_state != self.animation_key {
            let uid = self.widget_uid();
            if self.pre_state {
                cx.widget_action(uid, &Scope::empty().path, GShaderEvent::Closed);
            } else {
                cx.widget_action(uid, &Scope::empty().path, GShaderEvent::Opened);
            }
        }
    }
}

impl Widget for GShader {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if !self.animation_key || !self.visible {
            return;
        }
        if let Some(ne) = self.next_frame.is_event(event) {
            // update time to use for animation
            self.time = (ne.time * 0.001).fract() as f32;

            // force updates, so that we can animate in the absence of user-generated events
            self.redraw(cx);
            self.next_frame = cx.new_next_frame();
        }
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        self.set_scope_path(&scope.path);
        self.draw_shader.begin(cx, walk, self.layout);
        self.draw_shader.end(cx);
        DrawStep::done()
    }
}

impl GShader {
    set_scope_path!();
    widget_area! {
        area, draw_shader
    }
    event_bool! {
        opened: GShaderEvent::Opened,
        closed: GShaderEvent::Closed
    }
    pub fn open(&mut self, cx: &mut Cx) -> () {
        self.animation_key = true;
        self.draw_shader.opened = 1.0;
        self.redraw(cx);
    }
    pub fn close(&mut self, cx: &mut Cx) -> () {
        self.animation_key = false;
        self.draw_shader.opened = 0.0;
        self.redraw(cx);
    }
    pub fn redraw(&self, cx: &mut Cx) -> () {
        self.draw_shader.redraw(cx);
    }
}

impl GShaderRef {
    ref_redraw!();
    ref_area!();
    ref_event_bool! {
        opened,
        closed
    }
    pub fn open(&mut self, cx: &mut Cx) -> () {
        if let Some(mut c_ref) = self.borrow_mut() {
            c_ref.open(cx);
        }
    }
    pub fn close(&mut self, cx: &mut Cx) -> () {
        if let Some(mut c_ref) = self.borrow_mut() {
            c_ref.close(cx);
        }
    }
}

impl GShaderSet {
    set_event_bool! {
        opened,
        closed
    }
}
