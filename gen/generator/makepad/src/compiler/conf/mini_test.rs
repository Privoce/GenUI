use gen_components::*;
use makepad_widgets::*;
live_design! { use link :: widgets :: * ; use link :: gen_components :: * ; use link :: shaders :: * ; pub Header = { { Header } } { header_btn = < GButton > { padding : { left : 12.0 , top : 12.0 , right : 12.0 , bottom : 12.0 } , slot : < GLabel > { } } } }
#[derive(Live, Widget)]
pub struct Header {
    #[deref]
    pub deref_widget: GView,
    #[live]
    btn_name: String,
    #[rust]
    twb_poll: TwoWayBindingPoll,
}
impl Header {
    fn get_btn_name(&self) -> String {
        self.btn_name.clone()
    }
    fn set_btn_name(&mut self, cx: &mut Cx, value: String) -> () {
        if let Some(mut c_ref) = self.button(id!(header_btn)).borrow_mut() {
            let slot_widget = c_ref.slot.as_glabel();
            slot_widget.set_text(cx, value.clone());
        }
        self.btn_name = value.clone();
    }
    fn click_header(&self, actions: &Actions) -> Option<()> {
        if !self.event_key {
            return None;
        }
        if let HeaderEvent::ClickHeader = actions.find_widget_action(self.widget_uid()).cast() {
            Some(())
        } else {
            None
        }
    }
    fn active_event<F>(&mut self, cx: &mut Cx, f: F) -> ()
    where
        F: FnOnce(&mut Cx, WidgetUid, &HeapLiveIdPath) -> (),
    {
        if self.event_key {
            self.scope_path.as_ref().map(|path| {
                f(cx, self.widget_uid(), path);
            });
        }
    }
    #[allow(unused_variables)]
    fn click_h_btn(&self, cx: &mut Cx) {
        dbg!("clicked header");
    }
}
#[allow(unused)]
impl HeaderRef {
    pub fn get_btn_name(&self) -> String {
        self.getter(|c_ref| c_ref.btn_name.clone())
    }
    pub fn set_btn_name(&self, cx: &mut Cx, value: String) -> () {
        self.setter(cx, |c_ref, cx| {
            c_ref.set_btn_name(cx, value);
        });
    }
    fn setter<F>(&self, cx: &mut Cx, f: F) -> ()
    where
        F: FnOnce(&mut std::cell::RefMut<'_, Header>, &mut Cx),
    {
        if let Some(mut c_ref) = self.borrow_mut() {
            f(&mut c_ref, cx);
        }
    }
    fn getter<T, F>(&self, f: F) -> T
    where
        F: Fn(&std::cell::Ref<'_, Header>) -> T,
        T: Default,
    {
        if let Some(c_ref) = self.borrow() {
            f(&c_ref)
        } else {
            T::default()
        }
    }
    pub fn click_header(&self, actions: &Actions) -> Option<()> {
        if let Some(c_ref) = self.borrow() {
            return c_ref.click_header(actions);
        }
        None
    }
}
impl Widget for Header {
    #[allow(unused_variables)]
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    #[allow(unused_variables)]
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        let slot = self.glabel(id!(slot));
        let header_btn = self.gbutton(id!(header_btn));
        if let Some(_) = header_btn.clicked(&actions) {
            self.click_h_btn(cx);
        }
    }
    #[allow(unused_variables)]
    fn is_visible(&self) -> bool {
        self.visible
    }
}
impl LiveHook for Header {
    #[allow(unused_variables)]
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        let deref_prop = HeaderDeref::default();
        self.set_btn_name(cx, deref_prop.btn_name);
    }
    #[allow(unused_variables)]
    fn after_apply_from_doc(&mut self, cx: &mut Cx) {
        if !self.visible {
            return;
        }
        let c_ptr = self as *mut Header;
        self.twb_poll.on_btn_name_change = Some(Box::new(move |cx, new_state| unsafe {
            (*c_ptr).btn_name = new_state;
        }));
    }
}
#[derive(Default)]
struct TwoWayBindingPoll {
    pub on_btn_name_change: Option<Box<dyn Fn(&mut Cx, String)>>,
}
impl Default for HeaderDeref {
    fn default() -> Self {
        Self {
            btn_name: "Header Btn!".to_string(),
        }
    }
}
pub struct HeaderDeref {
    btn_name: String,
}
#[derive(Debug, Clone, DefaultNone)]
pub enum HeaderEvent {
    ClickHeader,
    None,
}
