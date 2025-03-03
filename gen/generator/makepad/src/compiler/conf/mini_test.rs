use gen_components::*;
use makepad_widgets::*;
live_design! { use link :: widgets :: * ; use link :: gen_components :: * ; use link :: shaders :: * ; pub UiRoot = { { UiRoot } } { < GView > { flow : Down , spacing : 12.0 , align : { x : 0.5 , y : 0.5 } , height : Fill , theme : Dark , width : Fill , < GLabel > { text : "This is an easy GenUI template" , font_size : 36.0 , } < EasyLabel > { } < Home > { } } } }
#[derive(Live, Widget)]
pub struct UiRoot {
    #[deref]
    pub deref_widget: GView,
}
impl UiRoot {}
#[allow(unused)]
impl UiRootRef {}
impl Widget for UiRoot {
    #[allow(unused_variables)]
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    #[allow(unused_variables)]
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
    }
    #[allow(unused_variables)]
    fn is_visible(&self) -> bool {
        self.visible
    }
}
impl LiveHook for UiRoot {
    #[allow(unused_variables)]
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}
