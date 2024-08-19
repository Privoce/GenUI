use crate::auto::IfWidget_01J5MY2VX5GJHCN44YZACH6ENE::*;
use makepad_widgets::*;
live_design! { import makepad_widgets :: base :: * ; import makepad_widgets :: theme_desktop_dark :: * ; import makepad_draw :: shader :: std :: * ; import crate :: auto :: IfWidget_01J5MY2VX5GJHCN44YZACH6ENE ::*; RootComponent = {{ RootComponent }}{ main_window = < Window >{ window : { } flow : Down , width : Fill , height : Fill , main_view = < View >{ flow : Down , height : All , if_widget1 = < IfWidget01J5MY2VX5GJHCN44YZACH6ENE >{ } toggle_btn = < Button >{ text : "click here to change if signal" , } } } } }
#[derive(Live, Widget)]
pub struct RootComponent {
    #[live]
    pub flag1: bool,
    #[deref]
    #[redraw]
    #[live]
    pub deref_widget: Root,
}
impl Widget for RootComponent {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        if let Event::Actions(actions) = event {
            if self.button(id!(toggle_btn)).clicked(actions) {
                let mut toggle = || {
                    self.flag1 = false;
                };
                
                let _ = toggle();
                self.if_widget01_j5_my2_vx5_gjhcn44_yzach6_ene(id!(if_widget1))
                    .set_if_signal(self.flag1);

                    self.redraw(cx);
            }
        }
        self.deref_widget.handle_event(cx, event, scope);
    }
}
impl LiveHook for RootComponent {
    fn before_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        dbg!("1");
        self.flag1 = true;
        self.if_widget01_j5_my2_vx5_gjhcn44_yzach6_ene(id!(if_widget1))
            .set_if_signal(self.flag1);
        self.redraw(cx);
       
    }
}
