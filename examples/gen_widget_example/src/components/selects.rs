use gen_components::components::{
    card::GCard,
    select::{event::GSelectEvent, types::SelectOption, GSelectWidgetExt},
};
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;

    GSelectExample = {{GSelectExample}}{
        height: 400.0,
        width: Fill,
        flow: Down,
        background_visible: false,
        easy = <GSelect>{}
    }
}

#[derive(Live, Widget)]
pub struct GSelectExample {
    #[deref]
    pub view: GCard,
}

impl LiveHook for GSelectExample {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, _nodes: &[LiveNode]) {
        self.gselect(id!(easy)).borrow_mut().map(|mut x| {
            x.options = vec![
                ("Rust", "rust").into(),
                ("C++", "cpp").into(),
                ("Python", "python").into(),
                ("JavaScript", "js").into(),
                ("TypeScript", "ts").into(),
            ];
        });
    }
}

impl Widget for GSelectExample {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.view.handle_event(cx, event, scope));

        self.gselect(id!(easy)).borrow_mut().map(|x| {
            let mut actions = actions.filter_widget_actions(x.widget_uid());
            actions.for_each(|action| {
                if let GSelectEvent::Changed(e) = action.cast() {
                    dbg!(e);
                }
            });
        });
    }
}
