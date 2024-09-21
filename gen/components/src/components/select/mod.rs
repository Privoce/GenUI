pub mod event;
pub mod item;
mod register;

use event::*;
pub use register::register;

use makepad_widgets::*;

use super::{card::GCard, popup::GPopup};

live_design!{
    GSelectBase = {{GSelect}}{
        height: 36.0,
        width: 180.0,
        border_width: 1.0,
        spread_radius: 2.2,
        shadow_offset: vec2(0.0, 2.0),
        blur_radius: 5.0,
        clip_x: false,
        clip_y: false,
        background_visible: false
    }
}

#[derive(Live, Widget)]
pub struct GSelect {
    #[deref]
    #[live]
    pub deref_widget: GCard,
    #[live]
    pub select_options:  Option<LivePtr>,
}


impl Widget for GSelect {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let _ = self.deref_widget.draw_walk(cx, scope, walk);
        DrawStep::done()
    }
}

impl LiveHook for GSelect {
    
}