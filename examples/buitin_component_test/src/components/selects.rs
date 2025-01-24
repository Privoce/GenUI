use gen_components::components::{
    view::GView,
    select::{event::GSelectEvent, GSelectWidgetExt},
};
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;

    GSelectExample = {{GSelectExample}}{
        height: 400.0,
        width: Fill,
        flow: Down,
        spacing: 10.0,
        background_visible: false,
        easy = <GSelect>{
            background_visible: true
        }
        // easy2 = <GSelect>{}
    }
}

#[derive(Live, Widget)]
pub struct GSelectExample {
    #[deref]
    pub view: GView,
}

impl LiveHook for GSelectExample {
    fn after_apply(
        &mut self,
        _cx: &mut Cx,
        _apply: &mut Apply,
        _index: usize,
        _nodes: &[LiveNode],
    ) {
        self.gselect(id!(easy)).borrow_mut().map(|mut x| {
            x.options = vec![
                ("Rust", "rust").into(),
                ("C++", "cpp").into(),
                ("Python", "python").into(),
                ("JavaScript", "js").into(),
                ("TypeScript", "ts").into(),
                ("Go", "go").into(),
            ];
        });
        self.gselect(id!(easy2)).borrow_mut().map(|mut x| {
            x.options = vec![
                ("Rust", "rust").into(),
                ("C++", "cpp").into(),
                ("Python", "python").into(),
                ("JavaScript", "js").into(),
                
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
            let actions = actions.filter_widget_actions(x.widget_uid());
            actions.for_each(|action| {
                if let GSelectEvent::Changed(e) = action.cast() {
                    dbg!(e);
                }
            });
        });
    }
}
