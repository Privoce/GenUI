mod event;
pub mod types;
pub mod page;
mod register;
use makepad_widgets::*;
pub use register::register;
use super::view::GView;

live_design! {
    GRouterBase = {{GRouter}}{}
}

#[derive(Live, Widget)]
pub struct GRouter {
    #[deref]
    pub deref_widget: GView,
    #[rust]
    screen_width: f64,
    #[rust(id!(app_page)[0])]
    active_router: LiveId,
    // #[rust]
    // register_routers: Option<Vec<LiveId>>
}

impl LiveHook for GRouter {}

impl Widget for GRouter {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.widget(&[self.active_router]).draw_all(cx, scope);
        DrawStep::done()
    }
}

impl GRouter {
    // pub fn
}
