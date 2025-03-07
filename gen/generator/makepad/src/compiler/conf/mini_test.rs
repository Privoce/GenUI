use crate::views::header::*;
use gen_components::*;
use makepad_widgets::*;
live_design! { use link :: widgets :: * ; use link :: gen_components :: * ; use link :: shaders :: * ; use crate :: views :: header ::*; pub TodoList = { { TodoList } } { item_ptr0 : < GView > { flow : Down , height : Fit , item_check = < GCheckbox > { height : 36.0 , font_size : 14.0 , theme : Info , } < GDivider > { theme : Info , } } , padding : 0.0 , spacing : 12.0 , background_visible : false , flow : Down , header = < Header > { align : { x : 1.0 , y : 0.5 , } , padding : 4.0 , height : Fit , } < GView > { height : Fit , padding : { left : 6.0 , top : 6.0 , right : 6.0 , bottom : 6.0 } , < GLabel > { font_size : 24.0 , text : "提醒" , color : vec4 (0.99607843 , 0.58431375 , 0.0 , 1.0) , } < GView > { align : { x : 1.0 , y : 0.5 } , todo_num = < GLabel > { color : vec4 (0.99607843 , 0.58431375 , 0.0 , 1.0) , font_size : 24.0 , } } } my_input = < GInput > { placeholder : "输入一条todo~~~" , width : Fill , height : 32.0 , } info_lb = < GLabel > { } check_list = < GView > { spacing : 16.0 , padding : { left : 4.0 , top : 4.0 , right : 4.0 , bottom : 4.0 } , flow : Down , < GLabel > { text : "这是个模仿mac todo的简单例子, 用作GenUI早期测试:" , } < GLabel > { text : "1. 基础组件 2. for循环控制 3. 网络请求 4. 绑定属性和事件" , } < GLabel > { text : "发送todo可能会导致400 Bad Request但这不是GenUI的问题，而是Makepad和GenUI进行的序列化选择不同" , } } } }
#[derive(Live, Widget)]
pub struct TodoList {
    #[deref]
    pub deref_widget: GView,
    #[live]
    todo_list: Vec<TodoItem>,
    #[live]
    num: String,
    #[live]
    input_txt: String,
    #[live]
    info: String,
    #[live]
    item_ptr0: Option<LivePtr>,
    #[rust]
    twb_poll: TwoWayBindingPoll,
}
impl TodoList {
    fn get_todo_list(&self) -> Vec<TodoItem> {
        self.todo_list.clone()
    }
    fn set_todo_list(&mut self, cx: &mut Cx, value: Vec<TodoItem>) -> () {
        self.sugar_for_todo_list(cx, &value);
        self.todo_list = value.clone();
    }
    fn get_num(&self) -> String {
        self.num.clone()
    }
    fn set_num(&mut self, cx: &mut Cx, value: String) -> () {
        let widget = self.glabel(id!(todo_num));
        widget.set_text(cx, value.clone());
        self.num = value.clone();
    }
    fn get_input_txt(&self) -> String {
        self.input_txt.clone()
    }
    fn set_input_txt(&mut self, cx: &mut Cx, value: String) -> () {
        let widget = self.ginput(id!(my_input));
        widget.set_text(cx, value.clone());
        self.input_txt = value.clone();
    }
    fn get_info(&self) -> String {
        self.info.clone()
    }
    fn set_info(&mut self, cx: &mut Cx, value: String) -> () {
        let widget = self.glabel(id!(info_lb));
        widget.set_text(cx, value.clone());
        self.info = value.clone();
    }
    fn sugar_for_todo_list(&mut self, cx: &mut Cx, value: &Vec<TodoItem>) -> () {
        let len_todo_list = self.todo_list.len();
        if let Some(mut father) = self.gview(id!(check_list)).borrow_mut() {
            if len_todo_list > 0 && father.children.len() > 3usize {
                for _ in 3usize..(3usize + len_todo_list) {
                    father.children.remove(3usize);
                }
            }
            for (index, item) in value.iter().enumerate() {
                let item = item.clone();
                let widget_ref = WidgetRef::new_from_ptr(cx, self.item_ptr0);
                let widget_target = widget_ref.as_gview();
                widget_target
                    .gcheckbox(id!(item_check))
                    .set_text(cx, item.text);
                widget_target
                    .gcheckbox(id!(item_check))
                    .set_selected(cx, item.done);
                father
                    .children
                    .insert(3usize + index, (LiveId(index as u64), widget_ref));
            }
            father.redraw(cx);
        }
    }
    #[allow(unused_variables)]
    fn change_input(&mut self, param: GInputChangedParam, cx: &mut Cx) {
        if let InputEventType::KeyDown(code) = param.ty {
            if let KeyCode::ReturnKey = code {
                let txt = self.get_input_txt();
                dbg!(txt);
            }
        }
    }
    #[allow(unused_variables)]
    fn call_add(&mut self, cx: &mut Cx) {
        dbg!("call add");
        if self.get_input_txt().is_empty() {
            let lb = self.glabel(id!(info_lb));
            lb.set_color(cx, "#FF0000".to_string());
            self.set_info(cx, "请输入todo，当前为空".to_string());
        } else {
            let txt = self.get_input_txt();
            let body = format!(
                r#"{{
                "todo": {{
                    "text": "{}",
                    "done": false
                }}
            }}"#,
                txt
            );
            dbg!("send post!");
            let _ = http_post!(save_todo, "", body);
        }
    }
    fn http_response1(&mut self, cx: &mut Cx, response: &HttpResponse) {
        dbg!("response");
        if response.status_code == 200 {
            let todos = response.get_string_body().unwrap();
            let todos: Value = serde_json::from_str(&todos).unwrap();
            let todo_items: Vec<TodoItem> = todos["data"]
                .as_array()
                .unwrap()
                .iter()
                .take(5)
                .map({
                    |todo| TodoItem {
                        id: todo["id"].as_u64().unwrap() as u64,
                        text: todo["text"].as_str().unwrap().to_string(),
                        done: todo["done"].as_bool().unwrap(),
                    }
                })
                .collect();
            self.set_todo_list(cx, todo_items);
        }
    }
    fn save_todo(&mut self, cx: &mut Cx, response: &HttpResponse) {
        if response.status_code == 201 {
            dbg!("save");
            let todo_response = response.get_string_body().unwrap();
            let todo: Value = serde_json::from_str(&todo_response).unwrap();
            let new_todo = TodoItem {
                id: todo["data"]["id"].as_u64().unwrap() as u64,
                text: todo["data"]["text"].as_str().unwrap().to_string(),
                done: todo["data"]["done"].as_bool().unwrap(),
            };
            let mut todos = self.get_todo_list();
            todos.push(new_todo);
            self.set_todo_list(cx, todos);
            self.set_info(cx, "添加todo成功!".to_string());
        } else {
            println!("{:?}", response);
        }
    }
}
#[allow(unused)]
impl TodoListRef {
    pub fn get_todo_list(&self) -> Vec<TodoItem> {
        self.getter(|c_ref| c_ref.todo_list.clone())
    }
    pub fn set_todo_list(&self, cx: &mut Cx, value: Vec<TodoItem>) -> () {
        self.setter(cx, |c_ref, cx| {
            c_ref.set_todo_list(cx, value);
        });
    }
    pub fn get_num(&self) -> String {
        self.getter(|c_ref| c_ref.num.clone())
    }
    pub fn set_num(&self, cx: &mut Cx, value: String) -> () {
        self.setter(cx, |c_ref, cx| {
            c_ref.set_num(cx, value);
        });
    }
    pub fn get_input_txt(&self) -> String {
        self.getter(|c_ref| c_ref.input_txt.clone())
    }
    pub fn set_input_txt(&self, cx: &mut Cx, value: String) -> () {
        self.setter(cx, |c_ref, cx| {
            c_ref.set_input_txt(cx, value);
        });
    }
    pub fn get_info(&self) -> String {
        self.getter(|c_ref| c_ref.info.clone())
    }
    pub fn set_info(&self, cx: &mut Cx, value: String) -> () {
        self.setter(cx, |c_ref, cx| {
            c_ref.set_info(cx, value);
        });
    }
    fn setter<F>(&self, cx: &mut Cx, f: F) -> ()
    where
        F: FnOnce(&mut std::cell::RefMut<'_, TodoList>, &mut Cx),
    {
        if let Some(mut c_ref) = self.borrow_mut() {
            f(&mut c_ref, cx);
        }
    }
    fn getter<T, F>(&self, f: F) -> T
    where
        F: Fn(&std::cell::Ref<'_, TodoList>) -> T,
        T: Default,
    {
        if let Some(c_ref) = self.borrow() {
            f(&c_ref)
        } else {
            T::default()
        }
    }
}
impl Widget for TodoList {
    #[allow(unused_variables)]
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    #[allow(unused_variables)]
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        self.widget_match_event(cx, event, scope);
        let check_item = self.gview(id!(check_item));
        let header = self.header(id!(header));
        let info_lb = self.glabel(id!(info_lb));
        let todo_num = self.glabel(id!(todo_num));
        let my_input = self.ginput(id!(my_input));
        if let Some(param) = my_input.changed(&actions) {
            let new_state = my_input.get_text();
            self.input_txt = new_state.clone();
            if let Some(on_change_callback) = self.twb_poll.on_input_txt_change.as_ref() {
                on_change_callback(cx, new_state);
            }
            self.change_input(param, cx);
        }
        if let Some(_) = header.add(&actions) {
            self.call_add(cx);
        }
    }
    #[allow(unused_variables)]
    fn is_visible(&self) -> bool {
        self.visible
    }
}
impl LiveHook for TodoList {
    #[allow(unused_variables)]
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        let deref_prop = TodoListDeref::default();
        self.set_todo_list(cx, deref_prop.todo_list);
        self.set_num(cx, deref_prop.num);
        self.set_input_txt(cx, deref_prop.input_txt);
        self.set_info(cx, deref_prop.info);
        {
            self.set_num(cx, "2".to_string());
            let _ = http_get!(http_response1);
        }
    }
    #[allow(unused_variables)]
    fn after_apply_from_doc(&mut self, cx: &mut Cx) {
        if !self.visible {
            return;
        }
        let c_ptr = self as *mut TodoList;
        self.twb_poll.on_info_change = Some(Box::new(move |cx, new_state| unsafe {
            (*c_ptr).info = new_state;
        }));
        self.twb_poll.on_num_change = Some(Box::new(move |cx, new_state| unsafe {
            (*c_ptr).num = new_state;
        }));
        self.twb_poll.on_todo_list_change = Some(Box::new(move |cx, new_state| unsafe {
            (*c_ptr).todo_list = new_state;
        }));
        self.twb_poll.on_input_txt_change = Some(Box::new(move |cx, new_state| unsafe {
            (*c_ptr).input_txt = new_state;
        }));
    }
}
impl WidgetMatchEvent for TodoList {
    fn handle_http_response(
        &mut self,
        cx: &mut Cx,
        request_id: LiveId,
        response: &HttpResponse,
        scope: &mut Scope,
    ) {
        match request_id {
            live_id!(http_response1) => self.http_response1(cx, response),
            live_id!(save_todo) => self.save_todo(cx, response),
            _ => {}
        }
    }
}
use serde_json::Value;
#[derive(Clone, Debug, Default, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct TodoItem {
    #[live]
    pub id: u64,
    #[live]
    pub text: String,
    #[live]
    pub done: bool,
}
#[derive(Default)]
struct TwoWayBindingPoll {
    pub on_info_change: Option<Box<dyn Fn(&mut Cx, String)>>,
    pub on_num_change: Option<Box<dyn Fn(&mut Cx, String)>>,
    pub on_todo_list_change: Option<Box<dyn Fn(&mut Cx, Vec<TodoItem>)>>,
    pub on_input_txt_change: Option<Box<dyn Fn(&mut Cx, String)>>,
}
impl Default for TodoListDeref {
    fn default() -> Self {
        Self {
            todo_list: vec![],
            info: "在输入框中添加todo后请点击右上角添加按钮".to_string(),
            num: Default::default(),
            input_txt: Default::default(),
        }
    }
}
pub struct TodoListDeref {
    todo_list: Vec<TodoItem>,
    num: String,
    input_txt: String,
    info: String,
}
