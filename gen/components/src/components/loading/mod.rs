pub mod event;
mod register;

use event::*;
use makepad_widgets::*;
pub use register::register;

use crate::{
    event_bool, ref_event_bool, set_event_bool, shader::draw_loading::{DrawGLoading, GLoadingType}, themes::Themes, utils::ThemeColor, widget_area
};

live_design! {
    GLoadingBase = {{GLoading}}{}
}

#[derive(Live, Widget)]
pub struct GLoading {
    #[live]
    pub theme: Themes,
    #[live]
    pub stroke_color: Option<Vec4>,
    // deref -------------------
    #[live]
    #[redraw]
    pub draw_loading: DrawGLoading,
    #[live]
    pub loading_type: GLoadingType,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    // frame -------------------
    #[live(true)]
    pub visible: bool,
    #[live(true)]
    pub animation_open: bool,
    #[live]
    pub time: f32,
    #[rust]
    next_frame: NextFrame,
    // store previous state(animation_open)
    #[rust]
    pub pre_state: bool,
}

impl Widget for GLoading {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        self.draw_loading.draw_walk(cx, walk);
        // redraw is important when changing visible or animation open state
        self.redraw(cx);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if !self.animation_open || !self.visible {
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
}

impl LiveHook for GLoading {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        self.pre_state = self.animation_open;
        if !self.visible {
            return;
        }
        // ------------------ hover color -----------------------------------------------
        let loading_color = self.stroke_color.get(self.theme, 600);
        // ------------------ apply to draw_loading_wrap ----------------------------------------
        self.draw_loading.apply_over(
            cx,
            live! {
                stroke_color: (loading_color),
            },
        );
        self.draw_loading.apply_type(self.loading_type.clone());
        self.draw_loading.redraw(cx);
    }
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        // starts the animation cycle on startup
        if self.animation_open {
            self.next_frame = cx.new_next_frame();
        }
    }

    fn after_update_from_doc(&mut self, cx: &mut Cx) {
        if self.pre_state != self.animation_open {
            let uid = self.widget_uid();
            if self.pre_state {
                cx.widget_action(uid, &Scope::empty().path, GLoadingEvent::Close);
            } else {
                cx.widget_action(uid, &Scope::empty().path, GLoadingEvent::Open);
            }
        }
    }
}

impl GLoading {
    widget_area! {
        area, draw_loading
    }
    event_bool!{
        open: GLoadingEvent::Open,
        close: GLoadingEvent::Close
    }
    pub fn redraw(&mut self, cx: &mut Cx){
        self.draw_loading.redraw(cx);
    }
}

impl GLoadingRef{
    ref_event_bool! {
        open,
        close
    }
}

impl GLoadingSet {
    set_event_bool!{
        open, 
        close
    }
}