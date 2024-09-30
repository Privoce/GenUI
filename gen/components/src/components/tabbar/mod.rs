mod register;
pub mod event;
pub mod item;

use event::*;
pub use register::register;

use makepad_widgets::*;

use crate::shader::draw_view::DrawGView;

live_design!{
    GTabbarBase = {{GTabbar}}{

    }
}

#[derive(Live, Widget)]
pub struct GTabbar{
    #[redraw]
    #[live]
    pub draw_tabbar: DrawGView,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout
}


impl Widget for GTabbar {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        DrawStep::done()
    }
}

impl LiveHook for GTabbar {
    
}