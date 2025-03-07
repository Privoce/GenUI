use gen_components::*;
use makepad_widgets::*;
live_design! { use link :: widgets :: * ; use link :: gen_components :: * ; use link :: shaders :: * ; pub Home = { { Home } } { item_ptr0 : < GView > { background_visible : false , align : { x : 0.0 , y : 0.5 } , height : Fit , box_left = < GView > { background_visible : false , spacing : 4.0 , height : Fit , flow : Down , title_lb = < GLabel > { color : vec4 (0.0 , 0.0 , 0.0 , 1.0) , } msg_lb = < GLabel > { color : vec4 (0.4 , 0.4 , 0.4 , 1.0) , } } < GCheckbox > { theme : Info , checkbox_type : Tick , selected : false , } } , height : Fill , flow : Down , align : { x : 0.5 , y : 0.0 , } , background_color : # FFF , spacing : 12.0 , border_radius : 0.0 , padding : 12.0 , < GView > { height : Fit , background_visible : false , < GLabel > { color : vec4 (0.0 , 0.0 , 0.0 , 1.0) , font_size : 14.0 , text : "User Profile" , } } header = < GView > { background_color : vec4 (1.0 , 1.0 , 1.0 , 1.0) , height : Fit , spacing : 16.0 , flow : Down , border_radius : 6.0 , border_width : 1.0 , border_color : vec4 (0.8666667 , 0.8666667 , 0.8666667 , 1.0) , padding : { left : 12.0 , top : 12.0 , right : 12.0 , bottom : 12.0 } , < GView > { width : Fill , background_visible : false , height : Fit , < GLabel > { font_size : 12.0 , color : vec4 (0.0 , 0.0 , 0.0 , 1.0) , text : "Profile information" , } } < GView > { background_visible : false , height : Fit , align : { x : 0.0 , y : 0.5 } , spacing : 24.0 , < GImage > { height : 60.0 , width : 60.0 , src : dep ("crate://self/resources/avatar.jpg") , } < GView > { flow : Down , background_visible : false , height : Fit , < GLabel > { color : vec4 (0.0 , 0.0 , 0.0 , 1.0) , text : "Profile Picture" , } < GLabel > { text : "Recommended size: 256x256px" , color : vec4 (0.4 , 0.4 , 0.4 , 1.0) , } } } < GView > { background_visible : false , height : Fit , flow : Down , spacing : 12.0 , < GView > { spacing : 16.0 , height : Fit , background_visible : false , < GView > { flow : Down , background_visible : false , height : 52.0 , spacing : 6.0 , < GLabel > { color : vec4 (0.0 , 0.0 , 0.0 , 1.0) , text : "Display Name" , } < GInput > { width : Fill , text : "Alexander Mitchell" , } } < GView > { background_visible : false , height : 52.0 , flow : Down , spacing : 6.0 , < GLabel > { color : vec4 (0.0 , 0.0 , 0.0 , 1.0) , text : "Username" , } < GInput > { width : Fill , text : "alex.mitchell" , } } } < GView > { flow : Down , height : 52.0 , spacing : 6.0 , background_visible : false , < GLabel > { color : vec4 (0.0 , 0.0 , 0.0 , 1.0) , text : "Email" , } < GInput > { width : Fill , text : "alexander.mitchell@example.com" , } } < GView > { background_visible : false , height : 92.0 , spacing : 6.0 , flow : Down , < GLabel > { text : "Bio" , color : vec4 (0.0 , 0.0 , 0.0 , 1.0) , } < GInput > { text : "Software engineer passionate about building great user experiences. Always learning and exploring new technologies." , width : Fill , } } } } setting_wrapper = < GView > { background_color : vec4 (1.0 , 1.0 , 1.0 , 1.0) , padding : { left : 12.0 , top : 12.0 , right : 12.0 , bottom : 12.0 } , border_radius : 6.0 , height : Fit , spacing : 16.0 , border_width : 1.0 , border_color : vec4 (0.8666667 , 0.8666667 , 0.8666667 , 1.0) , flow : Down , < GView > { height : Fit , background_visible : false , < GLabel > { color : vec4 (0.0 , 0.0 , 0.0 , 1.0) , font_size : 12.0 , text : "Account Settings" , } } } < GView > { background_color : vec4 (1.0 , 1.0 , 1.0 , 1.0) , border_color : vec4 (0.8666667 , 0.8666667 , 0.8666667 , 1.0) , border_radius : 6.0 , height : Fit , spacing : 16.0 , flow : Down , padding : { left : 12.0 , top : 12.0 , right : 12.0 , bottom : 12.0 } , border_width : 1.0 , < GView > { height : Fit , background_visible : false , < GLabel > { font_size : 12.0 , color : vec4 (0.0 , 0.0 , 0.0 , 1.0) , text : "Connected Devices" , } } < GView > { align : { x : 0.0 , y : 0.5 } , background_color : vec4 (0.9764706 , 0.98039216 , 0.9843137 , 1.0) , height : 60.0 , < GView > { margin : { left : 24.0 , top : 24.0 , right : 24.0 , bottom : 24.0 } , width : Fit , height : Fit , background_visible : false , < GImage > { height : 16.0 , src : dep ("crate://self/resources/computer.png") , width : 16.0 , } } < GView > { background_visible : false , height : Fit , spacing : 6.0 , flow : Down , con_title = < GLabel > { text : "MacBook Pro" , color : vec4 (0.0 , 0.0 , 0.0 , 1.0) , } con_msg = < GLabel > { color : vec4 (0.4 , 0.4 , 0.4 , 1.0) , text : "Last active: 2 minutes ago" , } } < GLabel > { color : vec4 (1.0 , 0.0 , 0.0 , 1.0) , margin : { left : 12.0 , top : 12.0 , right : 12.0 , bottom : 12.0 } , text : "Remove" , } } < GView > { align : { x : 0.0 , y : 0.5 } , background_color : vec4 (0.9764706 , 0.98039216 , 0.9843137 , 1.0) , height : 60.0 , < GView > { margin : { left : 24.0 , top : 24.0 , right : 24.0 , bottom : 24.0 } , height : Fit , background_visible : false , width : Fit , < GImage > { src : dep ("crate://self/resources/phone.png") , width : 16.0 , height : 16.0 , } } < GView > { background_visible : false , flow : Down , height : Fit , spacing : 6.0 , con_title = < GLabel > { text : "iPhone 15 Pro" , color : vec4 (0.0 , 0.0 , 0.0 , 1.0) , } con_msg = < GLabel > { color : vec4 (0.4 , 0.4 , 0.4 , 1.0) , text : "Last active: 5 minutes ago" , } } < GLabel > { color : vec4 (1.0 , 0.0 , 0.0 , 1.0) , margin : { left : 12.0 , top : 12.0 , right : 12.0 , bottom : 12.0 } , text : "Remove" , } } < GView > { background_color : vec4 (0.9764706 , 0.98039216 , 0.9843137 , 1.0) , align : { x : 0.0 , y : 0.5 } , height : 60.0 , < GView > { width : Fit , background_visible : false , margin : { left : 24.0 , top : 24.0 , right : 24.0 , bottom : 24.0 } , height : Fit , < GImage > { width : 16.0 , height : 16.0 , src : dep ("crate://self/resources/browser.png") , } } < GView > { height : Fit , flow : Down , spacing : 6.0 , background_visible : false , con_title = < GLabel > { color : vec4 (0.0 , 0.0 , 0.0 , 1.0) , text : "MacBook Pro" , } con_msg = < GLabel > { text : "Last active: 21 hours ago" , color : vec4 (0.4 , 0.4 , 0.4 , 1.0) , } } < GLabel > { margin : { left : 12.0 , top : 12.0 , right : 12.0 , bottom : 12.0 } , color : vec4 (1.0 , 0.0 , 0.0 , 1.0) , text : "Remove" , } } } < GView > { spacing : 16.0 , align : { x : 1.0 , y : 0.0 } , background_visible : false , < GButton > { theme : Info , slot : < GLabel > { text : "Cancel" , } } < GButton > { theme : Warning , slot : < GLabel > { text : "Save Change" , } } } } }
#[derive(Live, Widget)]
pub struct Home {
    #[deref]
    pub deref_widget: GView,
    #[live]
    num: u32,
    #[rust]
    accout_settings: Vec<AccountSetting>,
    #[live]
    item_ptr0: Option<LivePtr>,
    #[rust]
    twb_poll: TwoWayBindingPoll,
}
impl Home {
    fn get_num(&self) -> u32 {
        self.num.clone()
    }
    fn set_num(&mut self, cx: &mut Cx, value: u32) -> () {
        self.num = value.clone();
    }
    fn get_accout_settings(&self) -> Vec<AccountSetting> {
        self.accout_settings.clone()
    }
    fn set_accout_settings(&mut self, cx: &mut Cx, value: Vec<AccountSetting>) -> () {
        self.sugar_for_accout_settings(cx, &value);
        self.accout_settings = value.clone();
    }
    fn sugar_for_accout_settings(&mut self, cx: &mut Cx, value: &Vec<AccountSetting>) -> () {
        let len_accout_settings = self.accout_settings.len();
        if let Some(mut father) = self.gview(id!(setting_wrapper)).borrow_mut() {
            if len_accout_settings > 0 && father.children.len() > 1usize {
                for _ in 1usize..(1usize + len_accout_settings) {
                    father.children.remove(1usize);
                }
            }
            for (index, item) in value.iter().enumerate() {
                let item = item.clone();
                let widget_ref = WidgetRef::new_from_ptr(cx, self.item_ptr0);
                let widget_target = widget_ref.as_gview();
                let box_left = widget_target.gview(id!(box_left));
                box_left.glabel(id!(title_lb)).set_text(cx, item.title);
                let box_left = widget_target.gview(id!(box_left));
                box_left.glabel(id!(msg_lb)).set_text(cx, item.msg);
                father
                    .children
                    .insert(1usize + index, (LiveId(index as u64), widget_ref));
            }
            father.redraw(cx);
        }
    }
    fn clicked_my_btn(&self) {
        print!("clicked!");
    }
}
#[allow(unused)]
impl HomeRef {
    pub fn get_num(&self) -> u32 {
        self.getter(|c_ref| c_ref.num.clone())
    }
    pub fn set_num(&self, cx: &mut Cx, value: u32) -> () {
        self.setter(cx, |c_ref, cx| {
            c_ref.set_num(cx, value);
        });
    }
    pub fn get_accout_settings(&self) -> Vec<AccountSetting> {
        self.getter(|c_ref| c_ref.accout_settings.clone())
    }
    pub fn set_accout_settings(&self, cx: &mut Cx, value: Vec<AccountSetting>) -> () {
        self.setter(cx, |c_ref, cx| {
            c_ref.set_accout_settings(cx, value);
        });
    }
    fn setter<F>(&self, cx: &mut Cx, f: F) -> ()
    where
        F: FnOnce(&mut std::cell::RefMut<'_, Home>, &mut Cx),
    {
        if let Some(mut c_ref) = self.borrow_mut() {
            f(&mut c_ref, cx);
        }
    }
    fn getter<T, F>(&self, f: F) -> T
    where
        F: Fn(&std::cell::Ref<'_, Home>) -> T,
        T: Default,
    {
        if let Some(c_ref) = self.borrow() {
            f(&c_ref)
        } else {
            T::default()
        }
    }
}
impl Widget for Home {
    #[allow(unused_variables)]
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    #[allow(unused_variables)]
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        let setting_box = self.gview(id!(setting_box));
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
        self.set_accout_settings(cx, deref_prop.accout_settings);
    }
    #[allow(unused_variables)]
    fn after_apply_from_doc(&mut self, cx: &mut Cx) {
        if !self.visible {
            return;
        }
        let c_ptr = self as *mut Home;
        self.twb_poll.on_accout_settings_change = Some(Box::new(move |cx, new_state| unsafe {
            (*c_ptr).accout_settings = new_state;
        }));
    }
}
#[derive(Default, Debug, Clone)]
pub struct AccountSetting {
    title: String,
    msg: String,
    open: bool,
}
impl AccountSetting {
    pub fn new(title: &str, msg: &str, open: bool) -> Self {
        Self {
            title: title.to_string(),
            msg: msg.to_string(),
            open,
        }
    }
}
#[derive(Default)]
struct TwoWayBindingPoll {
    pub on_accout_settings_change: Option<Box<dyn Fn(&mut Cx, Vec<AccountSetting>)>>,
}
impl Default for HomeDeref {
    fn default() -> Self {
        Self {
            num: 0,
            accout_settings: vec![
                AccountSetting::new(
                    "Two-Factor Authentication",
                    "Add an extra layer of security to your account",
                    true,
                ),
                AccountSetting::new(
                    "Sync Across Devices",
                    "Keep your settings synchronized on all devices",
                    true,
                ),
                AccountSetting::new(
                    "Activity Status",
                    "Show when you're active to other users",
                    false,
                ),
            ],
        }
    }
}
pub struct HomeDeref {
    num: u32,
    accout_settings: Vec<AccountSetting>,
}
