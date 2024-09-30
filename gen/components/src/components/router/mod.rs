mod event;
use makepad_widgets::*;

use super::view::GView;

live_design! {
    GRouter = {{GRouter}}{}
}

#[derive(Live, Widget)]
pub struct GRouter {
    #[deref]
    view: GView,
    #[rust]
    screen_width: f64,
    #[rust]
    active_stack_view: ActiveStackView,
}

#[derive(Default)]
enum ActiveStackView {
    #[default]
    None,
    Active(LiveId),
}

impl LiveHook for GRouter {}

impl Widget for GRouter {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        DrawStep::done()
    }
}
