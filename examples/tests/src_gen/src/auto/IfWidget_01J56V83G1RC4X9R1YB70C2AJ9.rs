use makepad_widgets::*;
live_design! { import makepad_widgets :: base :: * ; import makepad_widgets :: theme_desktop_dark :: * ; import makepad_draw :: shader :: std :: * ; IfWidget01J56V83G1RC4X9R1YB70C2AJ9 = { { IfWidget01J56V83G1RC4X9R1YB70C2AJ9 } } { height : Fit , width : Fit , if_button : < Button > { text : "True Btn" , } , else_button : < Button > { text : "False Btn" , } } }
#[derive(Live, Widget, LiveHook)]
pub struct IfWidget01J56V83G1RC4X9R1YB70C2AJ9 {
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
impl Widget for IfWidget01J56V83G1RC4X9R1YB70C2AJ9 {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);
        cx.end_turtle();
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {}
}
impl IfWidget01J56V83G1RC4X9R1YB70C2AJ9Ref {
    pub fn set_if_signal(&mut self, prop: bool) {
        if let Some(mut instance) = self.borrow_mut() {
            instance.if_signal = prop;
        }
    }
}
