use gen_components::*;
use makepad_widgets::*;
live_design! { use link :: widgets :: * ; use link :: gen_components :: * ; use link :: shaders :: * ; pub Hello = { { Hello } } { my_lb = < GLabel > { font_size : 16.0 , } } }
#[derive(Live, Widget)]
pub struct Hello {
    #[deref]
    pub deref_widget: GView,
    #[live]
    my_text: String,
    #[rust]
    twb_poll: TwoWayBindingPoll,
}
impl Hello {
    fn get_my_text(&self) -> String {
        self.my_text.clone()
    }
    fn set_my_text(&mut self, cx: &mut Cx, value: String) -> () {
        let widget = self.glabel(id!(my_lb));
        widget.set_text(cx, value.clone());
        self.my_text = value.clone();
    }
}
#[allow(unused)]
impl HelloRef {
    pub fn get_my_text(&self) -> String {
        self.getter(|c_ref| c_ref.my_text.clone())
    }
    pub fn set_my_text(&self, cx: &mut Cx, value: String) -> () {
        self.setter(cx, |c_ref, cx| {
            c_ref.set_my_text(cx, value);
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
        let my_lb = self.glabel(id!(my_lb));
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
        self.my_text = deref_prop.my_text;
    }
    #[allow(unused_variables)]
    fn after_apply_from_doc(&mut self, cx: &mut Cx) {
        if !self.visible {
            return;
        }
        let c_ptr = self as *mut Hello;
        self.twb_poll.on_my_text_change = Some(Box::new(move |cx, new_state| unsafe {
            (*c_ptr).my_text = new_state;
        }));
    }
}
#[derive(Default)]
struct TwoWayBindingPoll {
    pub on_my_text_change: Option<Box<dyn Fn(&mut Cx, String)>>,
}
impl Default for HelloDeref {
    fn default() -> Self {
        Self {
            my_text: "Hello world".to_string(),
        }
    }
}
pub struct HelloDeref {
    my_text: String,
}
