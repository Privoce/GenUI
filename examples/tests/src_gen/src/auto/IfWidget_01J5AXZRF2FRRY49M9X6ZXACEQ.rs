use makepad_widgets::*;
live_design! { import makepad_widgets :: base :: * ; import makepad_widgets :: theme_desktop_dark :: * ; import makepad_draw :: shader :: std :: * ; IfWidget01J5AXZRF2FRRY49M9X6ZXACEQ = { { IfWidget01J5AXZRF2FRRY49M9X6ZXACEQ } } { height : Fit , width : Fit , if_button : < Button > { text : "True Btn" , } , else_button : < Button > { text : "False Btn" , } } }
#[derive(Live, Widget, LiveHook)]
pub struct IfWidget01J5AXZRF2FRRY49M9X6ZXACEQ {
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
impl Widget for IfWidget01J5AXZRF2FRRY49M9X6ZXACEQ {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);
        if self.if_signal {
            let _ = self.if_button.draw_walk(cx, scope, walk);
        } else {
            let _ = self.else_button.draw_walk(cx, scope, walk);
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
impl IfWidget01J5AXZRF2FRRY49M9X6ZXACEQRef {
    pub fn set_if_signal(&mut self, if_signal: bool) {
        if let Some(mut instance) = self.borrow_mut() {
            instance.if_signal = if_signal;
        }
    }
}