use makepad_widgets::*;
live_design! { import makepad_widgets :: base :: * ; import makepad_widgets :: theme_desktop_dark :: * ; import makepad_draw :: shader :: std :: * ; ButtonView = {{ ButtonView }}{ first_lb = < Label >{ draw_text : { text_style : { font_size : 32 , brightness : 1.1 , } , wrap : Word , color : vec4 (1.0 , 1.0 , 1.0 , 1.0) , } , } second_lb = < Label >{ draw_text : { text_style : { brightness : 1.1 , } , wrap : Word , color : vec4 (1.0 , 1.0 , 1.0 , 1.0) , } , text : "label 2" , } bb = < Button >{ text : "text btn" , } } }
#[derive(Live, Widget)]
pub struct ButtonView {
    #[live]
    pub label1: String,
    #[deref]
    pub deref_widget: View,
}
#[derive(DefaultNone, Clone, Debug)]
pub enum Events {
    Clicked(String),
    None,
}
impl Widget for ButtonView {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        if let Event::Actions(actions) = event {
            if self.button(id!(bb)).clicked(actions) {
                let mut btn_click = || {
                    self.label1 = String::from("I have been clicked");
                    cx.widget_action(uid, &scope.path, Events::Clicked("Hello".to_string()));
                };
                let _ = btn_click();
                self.label(id!(first_lb))
                    .apply_over_and_redraw(cx, live! { text : (self . label1) });
            }
        }
        self.redraw(cx);
        self.deref_widget.handle_event(cx, event, scope);
    }
}
impl LiveHook for ButtonView {
    fn before_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.label1 = "Click The Button".to_string();
    }
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.label1 = String::from("sss");
        let fs: f64 = 18.0;
        self.label(id!(second_lb)).apply_over_and_redraw(
            cx,
            live! { draw_text : { text_style : { font_size : (fs) , } , } , },
        );
        self.label(id!(first_lb))
            .apply_over_and_redraw(cx, live! { text : (self . label1) });
    }
}
