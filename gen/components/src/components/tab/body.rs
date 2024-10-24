use makepad_widgets::*;

use crate::components::view::GView;

live_design! {
    GTabBodyBase = {{GTabBody}}{
        background_visible: true,
        border_width: 0.0,
        border_radius: 0.0,
        padding: 0.0,
        margin: 0.0,
    }
}

#[derive(Live, Widget)]
pub struct GTabBody {
    #[live]
    pub text: ArcStringMut,
    #[live]
    #[deref]
    pub deref_widget: GView,
}

impl Widget for GTabBody {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.deref_widget.handle_event(cx, event, scope)
    }
    fn text(&self) -> String {
        self.text.as_ref().to_string()
    }
    fn set_text(&mut self, v: &str) {
        self.text.as_mut_empty().push_str(v);
    }
    fn set_text_and_redraw(&mut self, cx: &mut Cx, v: &str) {
        self.text.as_mut_empty().push_str(v);
        self.redraw(cx)
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GTabBody {}
