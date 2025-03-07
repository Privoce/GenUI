use gen_components::*;
use makepad_widgets::*;
live_design! { use link :: widgets :: * ; use link :: gen_components :: * ; use link :: shaders :: * ; pub NestedFor = { { NestedFor } } { item_ptr0 : < GView > { wrapper = < GView > { inner_label = < GLabel > { } } } , } }
#[derive(Live, Widget)]
pub struct NestedFor {
    #[deref]
    pub deref_widget: GView,
    #[live]
    alist: Vec<String>,
    #[live]
    item_ptr0: Option<LivePtr>,
    #[rust]
    twb_poll: TwoWayBindingPoll,
}
impl NestedFor {
    fn get_alist(&self) -> Vec<String> {
        self.alist.clone()
    }
    fn set_alist(&mut self, cx: &mut Cx, value: Vec<String>) -> () {
        self.sugar_for_alist(cx, &value);
        self.alist = value.clone();
    }
    fn sugar_for_alist(&mut self, cx: &mut Cx, value: &Vec<String>) -> () {
        let len_alist = self.alist.len();
        if len_alist > 0 && self.children.len() > 0usize {
            for _ in 0usize..(0usize + len_alist) {
                self.children.remove(0usize);
            }
        }
        for (index, item) in value.iter().enumerate() {
            let item = item.clone();
            let widget_ref = WidgetRef::new_from_ptr(cx, self.item_ptr0);
            let widget_target = widget_ref.as_gview();
            let wrapper = widget_target.gview(id!(wrapper));
            wrapper.glabel(id!(inner_label)).set_text(cx, item);
            self.children
                .insert(0usize + index, (LiveId(index as u64), widget_ref));
        }
        self.redraw(cx);
    }
}
#[allow(unused)]
impl NestedForRef {
    pub fn get_alist(&self) -> Vec<String> {
        self.getter(|c_ref| c_ref.alist.clone())
    }
    pub fn set_alist(&self, cx: &mut Cx, value: Vec<String>) -> () {
        self.setter(cx, |c_ref, cx| {
            c_ref.set_alist(cx, value);
        });
    }
    fn setter<F>(&self, cx: &mut Cx, f: F) -> ()
    where
        F: FnOnce(&mut std::cell::RefMut<'_, NestedFor>, &mut Cx),
    {
        if let Some(mut c_ref) = self.borrow_mut() {
            f(&mut c_ref, cx);
        }
    }
    fn getter<T, F>(&self, f: F) -> T
    where
        F: Fn(&std::cell::Ref<'_, NestedFor>) -> T,
        T: Default,
    {
        if let Some(c_ref) = self.borrow() {
            f(&c_ref)
        } else {
            T::default()
        }
    }
}
impl Widget for NestedFor {
    #[allow(unused_variables)]
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    #[allow(unused_variables)]
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        let for_wrapper = self.gview(id!(for_wrapper));
    }
    #[allow(unused_variables)]
    fn is_visible(&self) -> bool {
        self.visible
    }
}
impl LiveHook for NestedFor {
    #[allow(unused_variables)]
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        let deref_prop = NestedForDeref::default();
        self.set_alist(cx, deref_prop.alist);
    }
    #[allow(unused_variables)]
    fn after_apply_from_doc(&mut self, cx: &mut Cx) {
        if !self.visible {
            return;
        }
        let c_ptr = self as *mut NestedFor;
        self.twb_poll.on_alist_change = Some(Box::new(move |cx, new_state| unsafe {
            (*c_ptr).alist = new_state;
        }));
    }
}
#[derive(Default)]
struct TwoWayBindingPoll {
    pub on_alist_change: Option<Box<dyn Fn(&mut Cx, Vec<String>)>>,
}
impl Default for NestedForDeref {
    fn default() -> Self {
        Self {
            alist: vec!["Hello".to_string(), "World".to_string()],
        }
    }
}
pub struct NestedForDeref {
    alist: Vec<String>,
}
