use crate::auto::IfWidget_01J56V83G1RC4X9R1YB70C2AJ9::*;
use makepad_widgets::*;
live_design! { import makepad_widgets :: base :: * ; import makepad_widgets :: theme_desktop_dark :: * ; import makepad_draw :: shader :: std :: * ; import crate :: auto :: IfWidget_01J56V83G1RC4X9R1YB70C2AJ9 ::*; ui = {{ RootComponent }}{ main_window = < Window >{ window : { } flow : Down , width : Fill , height : Fill , main_view = < View >{ flow : Down , height : All , if_widget1 = < IfWidget01J56V83G1RC4X9R1YB70C2AJ9 >{ } } } } }
#[derive(Live, Widget)]
pub struct RootComponent {
    #[live]
    #[deref]
    pub deref_widget: Root,
}
impl Widget for RootComponent {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        if let Event::Actions(actions) = event {}
        self.deref_widget.handle_event(cx, event, scope);
    }
}
impl LiveHook for RootComponent {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        let flag1 = true;
        self.if_widget01_j56_v83_g1_rc4_x9_r1_yb70_c2_aj9(id!(if_widget1))
            .set_if_signal(flag1);
    }
}
