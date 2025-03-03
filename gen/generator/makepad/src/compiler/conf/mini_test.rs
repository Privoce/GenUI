use crate::components::header::*;
use gen_components::*;
use makepad_widgets::*;
live_design! { use link :: widgets :: * ; use link :: gen_components :: * ; use link :: shaders :: * ; use crate :: components :: header ::*; pub Hello = { { Hello } } { flow : Down , spacing : 12.0 , my_header = < Header > { height : 40.0 , } h_lb = < GLabel > { } h_btn = < GButton > { } } }
use crate::components::header::*;
#[derive(Live, Widget)]
pub struct Hello {
    #[deref]
    pub deref_widget: GView,
    #[live]
    txt: String,
    #[rust]
    twb_poll: TwoWayBindingPoll,
}
impl Hello {
    fn get_txt(&self) -> String {
        self.txt.clone()
    }
    fn set_txt(&mut self, cx: &mut Cx, value: String) -> () {
        let widget = self.glabel(id!(h_lb));
        widget.set_text(cx, value.clone());
        self.txt = value.clone();
    }
    #[allow(unused_variables)]
    fn click_btn(&mut self, cx: &mut Cx) {
        let lb = self.glabel(id!(h_lb));
        self.txt = "world".to_string().clone();
        lb.set_text(cx, "world".to_string());
        let txt = self.get_txt();
        dbg!(txt);
        let header = self.header(id!(my_header));
        header.set_btn_name(cx, "clicked".to_string());
        let header_btn_name = header.get_btn_name();
        dbg!(header_btn_name);
    }
}
#[allow(unused)]
impl HelloRef {
    pub fn get_txt(&self) -> String {
        self.getter(|c_ref| c_ref.txt.clone())
    }
    pub fn set_txt(&self, cx: &mut Cx, value: String) -> () {
        self.setter(cx, |c_ref, cx| {
            c_ref.set_txt(cx, value);
        });
    }
    fn setter<F>(&self, cx: &mut Cx, f: F) -> ()
    where
        F: FnOnce(&mut std::cell::RefMut<'_, Hello>, &mut Cx),
    {
        if let Some(mut c_ref) = self.borrow_mut() {
            f(&mut c_ref, cx);
        }
    }
    fn getter<T, F>(&self, f: F) -> T
    where
        F: Fn(&std::cell::Ref<'_, Hello>) -> T,
        T: Default,
    {
        if let Some(c_ref) = self.borrow() {
            f(&c_ref)
        } else {
            T::default()
        }
    }
}
impl Widget for Hello {
    #[allow(unused_variables)]
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    #[allow(unused_variables)]
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        let h_btn = self.gbutton(id!(h_btn));
        let h_lb = self.glabel(id!(h_lb));
        if let Some(_) = h_btn.clicked(&actions) {
            self.click_btn(cx);
        }
    }
    #[allow(unused_variables)]
    fn is_visible(&self) -> bool {
        self.visible
    }
}
impl LiveHook for Hello {
    #[allow(unused_variables)]
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        let deref_prop = HelloDeref::default();
        self.set_txt(cx, deref_prop.txt);
    }
    #[allow(unused_variables)]
    fn after_apply_from_doc(&mut self, cx: &mut Cx) {
        if !self.visible {
            return;
        }
        let c_ptr = self as *mut Hello;
        self.twb_poll.on_txt_change = Some(Box::new(move |cx, new_state| unsafe {
            (*c_ptr).txt = new_state;
        }));
    }
}
#[derive(Default)]
struct TwoWayBindingPoll {
    pub on_txt_change: Option<Box<dyn Fn(&mut Cx, String)>>,
}
impl Default for HelloDeref {
    fn default() -> Self {
        Self {
            txt: "hello!!!".to_string(),
        }
    }
}
pub struct HelloDeref {
    txt: String,
}
