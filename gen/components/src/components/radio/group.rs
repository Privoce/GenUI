use makepad_widgets::*;

use crate::components::card::GCard;

live_design! {
    GRadioGroupBase = {{GRadioGroup}} {
        border_radius: 0.0,
        border_width: 0.0,
        spread_radius: 0.0,
        background_visible: false,
        height: Fit,
        width: Fit,
        animation_open: false,
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct GRadioGroup {
    #[deref]
    pub deref_widget: GCard,
}

impl Widget for GRadioGroup {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        DrawStep::done()
    }
}
