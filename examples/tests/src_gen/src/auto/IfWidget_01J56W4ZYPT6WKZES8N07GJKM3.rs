use makepad_widgets::*;
live_design! { import makepad_widgets :: base :: * ; import makepad_widgets :: theme_desktop_dark :: * ; import makepad_draw :: shader :: std :: * ; IfWidget01J56W4ZYPT6WKZES8N07GJKM3 = { { IfWidget01J56W4ZYPT6WKZES8N07GJKM3 } } { height : Fit , width : Fit , if_button : < Button > { text : "True Btn" , } , else_button : < Button > { text : "False Btn" , } } }
#[derive(Live, Widget, LiveHook)]
pub struct IfWidget01J56W4ZYPT6WKZES8N07GJKM3 {
    #[rust]
    #[redraw]
    area: Area,
    #[layout]
    layout: Layout,
    #[walk]
    walk: Walk,
    #[live]
    if_button: Button,
    #[rust]
    if_signal: bool,
    #[live]
    else_button: Button,
}
impl Widget for IfWidget01J56W4ZYPT6WKZES8N07GJKM3 {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);
        if self.if_signal {
            let _ = self.if_button.draw_walk(cx, scope, walk);
        }
        cx.end_turtle();
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.if_signal {
            self.if_button.handle_event(cx, event, scope)
        }
    }
}
impl IfWidget01J56W4ZYPT6WKZES8N07GJKM3Ref {
    pub fn set_if_signal(&mut self, if_signal: bool) {
        if let Some(mut instance) = self.borrow_mut() {
            instance.if_signal = if_signal;
        }
    }
}
