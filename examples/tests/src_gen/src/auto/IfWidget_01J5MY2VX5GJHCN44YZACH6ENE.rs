use makepad_widgets::*;
live_design! { import makepad_widgets :: base :: * ; import makepad_widgets :: theme_desktop_dark :: * ; import makepad_draw :: shader :: std :: * ; IfWidget01J5MY2VX5GJHCN44YZACH6ENE = { { IfWidget01J5MY2VX5GJHCN44YZACH6ENE } } { height : Fit , width : Fit , if_button : < Button > { text : "True Btn" , } , else_button : < Button > { text : "False Btn" , } } }
#[derive(Live, Widget)]
pub struct IfWidget01J5MY2VX5GJHCN44YZACH6ENE {
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
impl Widget for IfWidget01J5MY2VX5GJHCN44YZACH6ENE {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);
        if self.if_signal {
            dbg!("yes");
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
        }else{
            self.else_button.handle_event(cx, event, scope)
        }
    }
}

impl LiveHook for IfWidget01J5MY2VX5GJHCN44YZACH6ENE {
    fn before_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        
    }
}

impl IfWidget01J5MY2VX5GJHCN44YZACH6ENERef {
    pub fn set_if_signal(&mut self, if_signal: bool) {
        if let Some(mut instance) = self.borrow_mut() {
            instance.if_signal = if_signal;
        }
    }
}
