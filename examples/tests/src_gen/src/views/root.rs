use crate::auto::IfWidget_01J57Z7DKG50EJY7WQJXR5QX8M::*;
use makepad_widgets::*;
live_design! { import makepad_widgets :: base :: * ; import makepad_widgets :: theme_desktop_dark :: * ; import makepad_draw :: shader :: std :: * ; import crate :: auto :: IfWidget_01J57Z7DKG50EJY7WQJXR5QX8M ::*; RootComponent = {{ RootComponent }}{ main_window = < Window >{ window : { } flow : Down , width : Fill , height : Fill , main_view = < View >{ flow : Down , height : All , if_widget1 = < IfWidget01J57Z7DKG50EJY7WQJXR5QX8M >{ } } } } }
#[derive(Live, Widget)]
pub struct RootComponent {
    #[live]
    pub flag1: bool,
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
        self.flag1 = true;
        // let mut props = RootComponent::default();
        self.if_widget01_j57_z7_dkg50_ejy7_wqjxr5_qx8_m(id!(if_widget1))
            .set_if_signal(self.flag1);
    }
}
