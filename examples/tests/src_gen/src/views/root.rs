use crate::auto::IfWidget_01J5MX1ANB7PPRH8MD05TR08RG::*;
use makepad_widgets::*;
live_design! { import makepad_widgets :: base :: * ; import makepad_widgets :: theme_desktop_dark :: * ; import makepad_draw :: shader :: std :: * ; import crate :: auto :: IfWidget_01J5MX1ANB7PPRH8MD05TR08RG ::*; RootComponent = {{ RootComponent }}{ main_window = < Window >{ window : { } flow : Down , width : Fill , height : Fill , main_view = < View >{ flow : Down , height : All , if_widget1 = < IfWidget01J5MX1ANB7PPRH8MD05TR08RG >{ } toggle_btn = < Button >{ text : "click here to change if signal" , } } } } }
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
        if let Event::Actions(actions) = event {
            let mut toggle = || {
                self.flag1 = false;
            };
            let _ = toggle();
            println!("{}", self.flag1);
            self.if_widget01_j5_mx1_anb7_pprh8_md05_tr08_rg(id!(if_widget1))
                .set_if_signal(self.flag1);
        }
        self.deref_widget.handle_event(cx, event, scope);
    }
}
impl LiveHook for RootComponent {
    fn before_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.flag1 = true;
        self.if_widget01_j5_mx1_anb7_pprh8_md05_tr08_rg(id!(if_widget1))
            .set_if_signal(self.flag1);
    }
}
