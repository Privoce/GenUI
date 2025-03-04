use crate::components::hello::*;
use gen_components::*;
use makepad_widgets::*;
live_design! { use link :: widgets :: * ; use link :: gen_components :: * ; use link :: shaders :: * ; use crate :: components :: hello ::*; pub Home = { { Home } } { flow : Down , spacing : 16.0 , height : Fit , align : { x : 0.5 , y : 0.5 , } , padding : 12.0 , header = < Hello > { } my_btn = < GButton > { theme : Error , slot : < GLabel > { font_size : 12.0 , text : "Click Me!" , } } } }
#[derive(Live, Widget)]
pub struct Home {
    #[deref]
    pub deref_widget: GView,
    #[live]
    num: u32,
}
impl Home {
    #[allow(unused_variables)]
    fn click_btn(&mut self, cx: &mut Cx) {
        let mut header = self.hello(id!(header));
        self.num += 1;
        header.set_my_text(cx, format!("Clicked: {}", self.num));
    }
}
#[allow(unused)]
impl HomeRef {}
impl Widget for Home {
    #[allow(unused_variables)]
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    #[allow(unused_variables)]
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        let my_btn = self.gbutton(id!(my_btn));
        if let Some(_) = my_btn.clicked(&actions) {
            self.click_btn(cx);
        }
    }
    #[allow(unused_variables)]
    fn is_visible(&self) -> bool {
        self.visible
    }
}
impl LiveHook for Home {
    #[allow(unused_variables)]
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        let deref_prop = HomeDeref::default();
        self.set_num(cx, deref_prop.num);
    }
}
impl Default for HomeDeref {
    fn default() -> Self {
        Self { num: 0 }
    }
}
pub struct HomeDeref {
    num: u32,
}
