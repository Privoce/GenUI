use makepad_widgets::*;
live_design! { import makepad_widgets :: base :: * ; import makepad_widgets :: theme_desktop_dark :: * ; import makepad_draw :: shader :: std :: * ; IfWidget01J544D879YKB7AS7T8B4YG20M = { { IfWidget01J544D879YKB7AS7T8B4YG20M } } { height : Fit , width : Fit , if_button : < Button > { text : "True Btn" , } , else_button : < Button > { text : "False Btn" , } } }
#[derive(Live, Widget, LiveHook)]
pub struct IfWidget01J544D879YKB7AS7T8B4YG20M {
    #[rust]
    #[redraw]
    area: Area,
    #[layout]
    layout: Layout,
    #[walk]
    walk: Walk,
    #[live]
    if_button: Button,
    #[live]
    else_button: Button,
    #[rust]
    else_signal: bool,
}
impl Widget for IfWidget01J544D879YKB7AS7T8B4YG20M {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);
        cx.end_turtle();
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {}
}
