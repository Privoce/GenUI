use makepad_widgets::*;
use shader::draw_text::TextWrap;

use crate::{
    shader::{draw_card::DrawCard, draw_text::DrawGText},
    themes::{get_color, Themes}, utils::get_font_family,
};

live_design! {
    import makepad_draw::shader::std::*;
    GInputBase = {{GInput}}{
        height: Fit,
        width: 180.0,
        align: {x: 0.0, y: 0.5},
        padding: 8.6,
        clip_x: false,
        clip_y: false,
        placeholder: "Please Input",
        text_align: {x: 0.0, y: 0.0},
        animator: {
            hover = {
                default: off
                off = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_select: {hover: 0.0},
                        draw_text: {hover: 0.0},
                        draw_input: {hover: 0.0},
                    }
                }
                on = {
                    from: {all: Snap}
                    apply: {
                        draw_select: {hover: 1.0},
                        draw_text: {hover: 1.0},
                        draw_input: {hover: 1.0},
                    }
                }
            }
            focus = {
                default: off
                off = {
                    from: {all: Forward {duration: .25}}
                    apply: {
                        draw_cursor: {focus: 0.0},
                        draw_input: {focus: 0.0},
                        draw_select: {focus: 0.0}
                        draw_text: {focus: 0.0}
                    }
                }
                on = {
                    from: {all: Snap}
                    apply: {
                        draw_cursor: {focus: 1.0},
                        draw_input: {focus: 1.0},
                        draw_select: {focus: 1.0}
                        draw_text: {focus: 1.0}
                    }
                }
            }
        },
        draw_input: {
            instance hover: 0.0
            instance pressed: 0.0
            instance focus: 0.0

            fn get_color(self) -> vec4 {
                return self.background_color
            }

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                sdf.box(
                    1.,
                    1.,
                    self.rect_size.x - 2.0,
                    self.rect_size.y - 2.0,
                    self.border_radius
                )

                sdf.fill_keep(self.get_color());

                sdf.stroke(
                    self.get_border_color(),
                    self.border_width
                );

                return sdf.result
            }
        }
        draw_text: {
            instance hover: 0.0
            instance focus: 0.0
            wrap: Word,
            
            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        self.color,
                        mix(self.hover_color, self.pressed_color, self.pressed),
                        self.hover
                    ),
                    self.hover_color,
                    self.empty
                )
            }
        }

        draw_cursor: {
            instance focus: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    0.,
                    0.,
                    self.rect_size.x,
                    self.rect_size.y,
                    self.border_radius
                )
                sdf.fill(self.get_color());
                return sdf.result;
            }
        }

        draw_select: {
            instance hover: 0.0
            instance focus: 0.0
            
            fn pixel(self) -> vec4 {
                //return mix(#f00,#0f0,self.pos.y)
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    0.,
                    0.,
                    self.rect_size.x,
                    self.rect_size.y,
                    0.5
                )
                sdf.fill(
                    self.get_color()
                ); 
                return sdf.result
            }
        }
        
    }
}

#[derive(Live, Widget)]
pub struct GInput {
    #[live]
    pub theme: Themes,
    #[live]
    pub color: Option<Vec4>,
    #[live]
    pub background_color: Option<Vec4>,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub pressed_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(0.0)]
    pub border_width: f32,
    #[live(2.0)]
    pub border_radius: f32,
    #[live(false)]
    pub round: bool,
    #[live]
    pub value: String,
    #[live(String::from("Please Input"))]
    pub placeholder: String,
    #[live]
    pub input_type: GInputType,
    #[live(false)]
    pub disabled: bool,
    // text --------------------
    #[live]
    pub text_align: Align,
    #[live(9.0)]
    pub font_size: f64,
    #[live(1.0)]
    pub brightness: f32,
    #[live(0.5)]
    pub curve: f32,
    #[live(1.2)]
    pub top_drop: f64,
    #[live(1.3)]
    pub height_factor: f64,
    #[live(TextWrap::Word)]
    pub wrap: TextWrap,
    #[live]
    pub font_family: LiveDependency,
    // cursor styles ------------------
    #[live(2.0)]
    cursor_width: f64,
    #[live(1.0)]
    cursor_border_radius: f64,
    #[live(0.0)]
    cursor_margin_bottom: f64,
    #[live(0.0)]
    cursor_margin_top: f64,
    #[live(false)]
    on_focus_select_all: bool,
    #[rust]
    cursor_tail: usize,
    #[rust]
    cursor_head: usize,
    // deref --------------------
    #[redraw]
    #[live]
    draw_input: DrawCard,
    #[live]
    // 光标
    draw_select: DrawCard,
    #[live]
    draw_cursor: DrawCard,
    #[live]
    draw_text: DrawGText,
    /// 撤销id
    #[rust]
    undo_id: u64,
    /// 上一次的撤销
    #[rust]
    last_undo: Option<UndoItem>,
    /// 撤销栈
    #[rust]
    undo_stack: Vec<UndoItem>,
    /// 重做栈
    #[rust]
    redo_stack: Vec<UndoItem>,
    #[rust]
    double_tap_start: Option<(usize, usize)>,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[animator]
    animator: Animator,
}

#[derive(Clone)]
struct UndoItem {
    text: String,
    undo_group: UndoGroup,
    cursor_head: usize,
    cursor_tail: usize,
}

#[derive(Live, LiveHook, Clone)]
#[live_ignore]
#[repr(u32)]
pub enum GInputType {
    #[pick]
    /// text
    Text = shader_enum(1),
    /// password
    Pwd = shader_enum(2),
}
#[derive(Clone, Debug, PartialEq, DefaultNone)]
pub enum GTextInputEvent {
    Changed(String),
    Return(String),
    Escape,
    KeyFocus,
    KeyFocusLost,
    None,
}

impl Widget for GInput {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        if self.animator_handle_event(cx, event).must_redraw() {
            self.draw_input.redraw(cx);
        }
        match event.hits(cx, self.draw_input.area()) {
            Hit::KeyFocusLost(_) => {
                // 焦点丢失事件
                self.animator_play(cx, id!(focus.off));
                cx.hide_text_ime();
                cx.widget_action(uid, &scope.path, GTextInputEvent::KeyFocusLost);
            },
            Hit::KeyFocus(_) => {
                // 获取焦点事件
                self.undo_id += 1;
                self.animator_play(cx, id!(focus.on));
                if self.on_focus_select_all {
                    self.select_all();
                }
                self.draw_input.redraw(cx);
                cx.widget_action(uid, &scope.path, GTextInputEvent::KeyFocus);
            }
            Hit::KeyDown(ke) => {
                // 键盘事件
                match ke.key_code {
                    KeyCode::Tab => {}
                    KeyCode::ReturnKey if ke.modifiers.shift => {
                        if self.change(cx, "\n") {
                            self.push_change_action(uid, scope, cx)
                        }
                    }
                    KeyCode::ReturnKey => {
                        cx.hide_text_ime();
                        cx.widget_action(
                            uid,
                            &scope.path,
                            TextInputAction::Return(self.value.clone()),
                        );
                    }
                    KeyCode::Escape => {
                        cx.widget_action(uid, &scope.path, TextInputAction::Escape);
                    }
                    KeyCode::KeyZ if ke.modifiers.logo || ke.modifiers.shift => {
                        if self.disabled {
                            return;
                        }
                        self.undo_id += 1;
                        if ke.modifiers.shift {
                            self.redo();
                        } else {
                            self.undo();
                        }
                        self.push_change_action(uid, scope, cx);
                        self.draw_input.redraw(cx);
                    }
                    KeyCode::KeyA if ke.modifiers.logo || ke.modifiers.control => {
                        self.undo_id += 1;
                        self.cursor_tail = 0;
                        self.cursor_head = self.value.chars().count();
                        self.draw_input.redraw(cx);
                    }
                    KeyCode::ArrowLeft => {
                        if !ke.modifiers.logo {
                            self.undo_id += 1;
                            if self.cursor_head > 0 {
                                self.cursor_head -= 1;
                            }
                            if !ke.modifiers.shift {
                                self.cursor_tail = self.cursor_head;
                            }
                            self.draw_input.redraw(cx);
                        }
                    }
                    KeyCode::ArrowRight => {
                        if !ke.modifiers.logo {
                            self.undo_id += 1;
                            if self.cursor_head < self.value.chars().count() {
                                self.cursor_head += 1;
                            }
                            if !ke.modifiers.shift {
                                self.cursor_tail = self.cursor_head;
                            }
                            self.draw_input.redraw(cx);
                        }
                    }
                    KeyCode::ArrowDown => {
                        if !ke.modifiers.logo {
                            self.undo_id += 1;
                            // we need to figure out what is below our current cursor
                            if let Some(pos) = self.draw_text.get_cursor_pos(
                                cx,
                                self.newline_indexes(),
                                0.0,
                                self.cursor_head,
                            ) {
                                if let Some(pos) = self.draw_text.closest_offset(
                                    cx,
                                    self.newline_indexes(),
                                    dvec2(pos.x, pos.y + self.draw_text.get_line_spacing() * 1.5),
                                ) {
                                    self.cursor_head = pos;
                                    if !ke.modifiers.shift {
                                        self.cursor_tail = self.cursor_head;
                                    }
                                    self.draw_input.redraw(cx);
                                }
                            }
                        }
                    }
                    KeyCode::ArrowUp => {
                        if !ke.modifiers.logo {
                            self.undo_id += 1;
                            // we need to figure out what is below our current cursor
                            if let Some(pos) = self.draw_text.get_cursor_pos(
                                cx,
                                self.newline_indexes(),
                                0.0,
                                self.cursor_head,
                            ) {
                                if let Some(pos) = self.draw_text.closest_offset(
                                    cx,
                                    self.newline_indexes(),
                                    dvec2(pos.x, pos.y - self.draw_text.get_line_spacing() * 0.5),
                                ) {
                                    self.cursor_head = pos;
                                    if !ke.modifiers.shift {
                                        self.cursor_tail = self.cursor_head;
                                    }
                                    self.draw_input.redraw(cx);
                                }
                            }
                        }
                    }
                    KeyCode::Home => {
                        if !ke.modifiers.logo {
                            self.undo_id += 1;
                            self.cursor_head = 0;
                            if !ke.modifiers.shift {
                                self.cursor_tail = self.cursor_head;
                            }
                            self.draw_input.redraw(cx);
                        }
                    }
                    KeyCode::End => {
                        if !ke.modifiers.logo {
                            self.undo_id += 1;
                            self.cursor_head = self.value.chars().count();

                            if !ke.modifiers.shift {
                                self.cursor_tail = self.cursor_head;
                            }
                            self.draw_input.redraw(cx);
                        }
                    }
                    KeyCode::Backspace => {
                        self.create_undo(UndoGroup::Backspace(self.undo_id));
                        if self.cursor_head == self.cursor_tail {
                            if self.cursor_tail > 0 {
                                self.cursor_tail -= 1;
                            }
                        }
                        if self.change(cx, "") {
                            self.push_change_action(uid, scope, cx)
                        }
                    }
                    KeyCode::Delete => {
                        self.create_undo(UndoGroup::Delete(self.undo_id));
                        if self.cursor_head == self.cursor_tail {
                            if self.cursor_head < self.value.chars().count() {
                                self.cursor_head += 1;
                            }
                        }
                        if self.change(cx, "") {
                            self.push_change_action(uid, scope, cx)
                        }
                    }
                    _ => {}
                }
            }

            Hit::TextInput(e) => {
                let mut input = String::new();

                self.filter_input(&e.input, Some(&mut input));
                if input.len() == 0 {
                    return;
                }
                let last_undo = self.last_undo.take();

                if e.replace_last {
                    self.undo_id += 1;
                    self.create_undo(UndoGroup::TextInput(self.undo_id));
                    if let Some(item) = last_undo {
                        self.consume_undo_item(item);
                    }
                } else {
                    // if input.is_empty(){
                    //     self.undo_id += 1;
                    // }
                    // here need to check todo!()!
                    if input == " " {
                        self.undo_id += 1;
                    }
                    // if this one follows a space, it still needs to eat it
                    self.create_undo(UndoGroup::TextInput(self.undo_id));
                }
                if self.change(cx, &input) {
                    cx.widget_action(
                        uid,
                        &scope.path,
                        GTextInputEvent::Changed(self.value.to_string()),
                    );
                }
            }
            Hit::TextCopy(e) => {
                self.undo_id += 1;
                *e.response.borrow_mut() = Some(self.selected_text());
            }
            Hit::TextCut(e) => {
                self.undo_id += 1;
                if self.cursor_head != self.cursor_tail {
                    *e.response.borrow_mut() = Some(self.selected_text());
                    self.create_undo(UndoGroup::Cut(self.undo_id));
                    if self.change(cx, "") {
                        cx.widget_action(
                            uid,
                            &scope.path,
                            GTextInputEvent::Changed(self.value.to_string()),
                        );
                    }
                }
            }
            Hit::FingerDown(e) => {
                cx.set_cursor(MouseCursor::Text);
                cx.set_key_focus(self.draw_input.area());
                if let Some(pos) = self
                    .draw_text
                    .closest_offset(cx, self.newline_indexes(), e.abs)
                {
                    let pos = pos.min(self.value.chars().count());
                    if e.tap_count == 1 {
                        if pos != self.cursor_head {
                            self.cursor_head = pos;
                            if !e.modifiers.shift {
                                self.cursor_tail = pos;
                            }
                        }
                        self.draw_input.redraw(cx);
                    }
                    if e.tap_count == 2 {
                        self.select_ranged_text(pos);
                        self.double_tap_start = Some((self.cursor_head, self.cursor_tail));
                    }
                    if e.tap_count == 3 {
                        self.select_all();
                    }
                    self.draw_input.redraw(cx);
                }
            }
            Hit::FingerMove(e) => {
                if let Some(pos) = self
                    .draw_text
                    .closest_offset(cx, self.newline_indexes(), e.abs)
                {
                    let pos = pos.min(self.value.chars().count());
                    if e.tap_count == 1 {
                        if let Some(pos_start) =
                            self.draw_text
                                .closest_offset(cx, self.newline_indexes(), e.abs_start)
                        {
                            let pos_start = pos_start.min(self.value.chars().count());
                            self.cursor_head = pos_start;
                            self.cursor_tail = self.cursor_head;
                        }
                        if pos != self.cursor_head {
                            self.cursor_head = pos;
                        }
                        self.draw_input.redraw(cx);
                    }

                    if e.tap_count == 2 {
                        let (head, tail) = self.double_tap_start.unwrap();
                        self.select_ranged_text(pos);
                        if head > self.cursor_head {
                            self.cursor_head = head;
                        }
                        if tail < self.cursor_tail {
                            self.cursor_tail = tail;
                        }
                        self.draw_input.redraw(cx);
                    }
                }
            }
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Text);
                self.animator_play(cx, id!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerUp(e) => {
                self.double_tap_start = None;
                if let Some(pos) = self
                    .draw_text
                    .closest_offset(cx, self.newline_indexes(), e.abs)
                {
                    let pos = pos.min(self.value.chars().count());
                    if !e.modifiers.shift && e.tap_count == 1 && e.was_tap() {
                        self.cursor_head = pos;
                        self.cursor_tail = pos;
                        self.draw_input.redraw(cx);
                    }
                }
                if e.was_long_press() {
                    cx.show_clipboard_actions(self.selected_text());
                }
                if e.is_over && e.device.has_hovers() {
                    self.animator_play(cx, id!(hover.on));
                } else {
                    self.animator_play(cx, id!(hover.off));
                }
            }
            _ => {}
        }
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        // 开始绘制
        self.draw_input.begin(cx, walk, self.layout);
        let font = get_font_family(&self.font_family, cx);

        self.draw_text.text_style.font = font;
        let turtle_rect = cx.turtle().rect();
        // 设置光标永远在文字后
        self.draw_select.append_to_draw_call(cx);

        // 设置文字
        // 文字长度为0
        if self.value.len() == 0 {
            
            self.draw_text.empty = 1.0;
            // 设置文字的高度和宽度
            // 设置文字的对齐方式 和 无输入内容是的默认填充文字
            self.draw_text.draw_walk(
                cx,
                Walk::size(self.walk.width, self.walk.height),
                self.text_align,
                &self.placeholder,
            );
        } else {
            
            self.draw_text.empty = 0.0;
            // 匹配输入类型
            match self.input_type {
                GInputType::Text => {
                    // 设置文字输入
                    self.draw_text.draw_walk(
                        cx,
                        Walk::size(self.walk.width, self.walk.height),
                        self.text_align,
                        &self.value,
                    );
                }
                GInputType::Pwd => {
                    // 设置密码输入，就用*代替
                    self.draw_text.draw_walk(
                        cx,
                        Walk::size(self.walk.width, self.walk.height),
                        self.text_align,
                        &"*".repeat(self.value.len()),
                    );
                }
            }
        }

        let mut turtle = cx.turtle().padded_rect_used();
        turtle.pos.y -= self.cursor_margin_top;
        turtle.size.y += self.cursor_margin_top + self.cursor_margin_bottom;
        let line_spacing = self.draw_text.get_line_spacing();
        let top_drop = self.draw_text.get_font_size() * 0.2;
        let head = self
            .draw_text
            .get_cursor_pos(cx, self.newline_indexes(), 0.0, self.cursor_head)
            .unwrap_or(dvec2(turtle.pos.x, 0.0));

        //
        if !self.disabled && self.cursor_head == self.cursor_tail {
            self.draw_cursor.draw_abs(
                cx,
                Rect {
                    pos: dvec2(head.x - 0.5 * self.cursor_width, head.y - top_drop),
                    size: dvec2(self.cursor_width, line_spacing),
                },
            );
        }
        if self.cursor_head != self.cursor_tail {
            let top_drop = self.draw_text.get_font_size() * 0.3;
            let bottom_drop = self.draw_text.get_font_size() * 0.1;

            let (start, end) = self.sorted_cursor();
            let rects = self.draw_text.get_selection_rects(
                cx,
                self.newline_indexes(),
                start,
                end,
                dvec2(0.0, -top_drop),
                dvec2(0.0, bottom_drop),
            );
            for rect in rects {
                self.draw_select.draw_abs(cx, rect);
            }
        }
        self.draw_input.end(cx);
        // 检查是否有键盘焦点：cx.has_key_focus(self.draw_bg.area())
        // 获取光标位置：let ime_x = self.draw_text.get_cursor_pos(...)
        // 如果输入框只接受数字输入，隐藏 IME：cx.hide_text_ime()
        // 否则，显示 IME 并设置其位置：cx.show_text_ime(...)
        if cx.has_key_focus(self.draw_input.area()) {
            // ok so. if we have the IME we should inject a tracking point
            let ime_x = self
                .draw_text
                .get_cursor_pos(cx, self.newline_indexes(), 0.5, self.cursor_head)
                .unwrap_or(dvec2(turtle.pos.x, 0.0))
                .x;

            // if self.numeric_only {
            //     cx.hide_text_ime();
            // }
            // else {
            //     let ime_abs = dvec2(ime_x, turtle.pos.y);
            //     cx.show_text_ime(self.draw_bg.area(), ime_abs - turtle_rect.pos);
            // }
            let ime_abs = dvec2(ime_x, turtle.pos.y);
            cx.show_text_ime(self.draw_input.area(), ime_abs - turtle_rect.pos);
        }

        cx.add_nav_stop(
            self.draw_input.area(),
            NavRole::TextInput,
            Margin::default(),
        );
        DrawStep::done()
    }
}

impl LiveHook for GInput {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        // ----------------- background color -------------------------------------------
        let bg_color = get_color(self.theme, self.background_color, 25);
        // ------------------ hover color -----------------------------------------------
        let hover_color = get_color(self.theme, self.hover_color, 400);
        // ------------------ pressed color ---------------------------------------------
        let pressed_color = get_color(self.theme, self.pressed_color, 600);
        // ------------------ border color ----------------------------------------------
        let border_color = get_color(self.theme, self.border_color, 800);
        // ------------------ font ------------------------------------------------------
        let font_color = get_color(self.theme, self.color, 800);
        // ---------------------- is empty ------------------------------------------------
        let empty = self.value.len().eq(&0) as u8 as f32;
        // ---------------------- select color ------------------------------------------
        let mut select_color = font_color.clone();
        select_color.w = 0.5;
        // ------------------ round -----------------------------------------------------
        if self.round {
            self.border_radius = match self.walk.height {
                Size::Fixed(h) => (h * 0.25) as f32,
                Size::Fit => {
                    ((self.draw_text.text_style.font_size
                        + self.layout.padding.top
                        + self.layout.padding.bottom)
                        * 0.25) as f32
                }
                _ => panic!("round only support fixed and fit"),
            };
        }
        // draw input --------------------------------------------------------------
        self.draw_input.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                border_color: (border_color),
                border_width: (self.border_width),
                border_radius: (self.border_radius),
                pressed_color: (pressed_color),
                hover_color: (hover_color),
            },
        );
        // draw text ---------------------------------------------------------------
        self.draw_text.apply_over(
            cx,
            live! {
                color: (font_color),
                hover_color: (hover_color),
                pressed_color: (pressed_color),
                empty: (empty),
                text_style: {
                    brightness: (self.brightness),
                    curve: (self.curve),
                    line_spacing: (self.layout.line_spacing),
                    top_drop: (self.top_drop),
                    font_size: (self.font_size),
                    height_factor: (self.height_factor),
                }
            },
        );
        // draw cursor -------------------------------------------------------------
        self.draw_cursor.apply_over(cx, live!{
            background_color: (font_color),
            border_color: (border_color),
            // border_width: (self.border_width),
            border_radius: (self.cursor_border_radius),
            pressed_color: (pressed_color),
            hover_color: (hover_color),
        });
        // draw select -------------------------------------------------------------
        self.draw_select.apply_over(cx, live!{
            background_color: (select_color),
            pressed_color: (pressed_color),
            hover_color: (hover_color),
        });
        self.draw_text.redraw(cx);
        self.draw_input.redraw(cx);
        self.draw_cursor.redraw(cx);
        self.draw_select.redraw(cx);
    }
}

impl GInput {
    fn newline_indexes(&self) -> Vec<usize> {
        let mut ret = Vec::new();
        for (i, c) in self.value.chars().enumerate() {
            if c == '\n' {
                ret.push(i);
            }
        }
        ret
    }
    pub fn sorted_cursor(&self) -> (usize, usize) {
        if self.cursor_head < self.cursor_tail {
            (self.cursor_head, self.cursor_tail)
        } else {
            (self.cursor_tail, self.cursor_head)
        }
    }
    /// 选择所有
    pub fn select_all(&mut self) {
        self.cursor_tail = 0;
        self.cursor_head = self.value.chars().count();
    }
    /// 过滤输入
    pub fn filter_input(&mut self, input: &str, output: Option<&mut String>) {
        let output = if let Some(output) = output {
            output
        } else {
            &mut self.value
        };
        output.clear();
        // if self.ascii_only {
        //     for c in input.as_bytes() {
        //         if *c>31 && *c<127 {
        //             output.push(*c as char);
        //         }
        //     }
        // }
        // else if self.numeric_only {
        //     let mut output = String::new();
        //     for c in input.chars() {
        //         if c.is_ascii_digit() || c == '.' {
        //             output.push(c);
        //         }
        //         else if c == ',' {
        //             // some day someone is going to search for this for days
        //             output.push('.');
        //         }
        //     }
        // }
        // else {
        //     output.push_str(input);
        // }
        output.push_str(input);
    }
    /// 创建撤销
    pub fn create_undo(&mut self, undo_group: UndoGroup) {
        // 如果禁用，直接返回
        if self.disabled {
            return;
        }
        // 清空重做栈
        self.redo_stack.clear();
        let new_item = self.create_undo_item(undo_group);
        if let Some(item) = self.undo_stack.last_mut() {
            if item.undo_group != undo_group {
                self.last_undo = Some(new_item.clone());
                self.undo_stack.push(new_item);
            } else {
                self.last_undo = Some(new_item);
            }
        } else {
            self.last_undo = Some(new_item.clone());
            self.undo_stack.push(new_item);
        }
    }
    /// 创建撤销项，文字，光标位置
    fn create_undo_item(&mut self, undo_group: UndoGroup) -> UndoItem {
        UndoItem {
            undo_group: undo_group,
            text: self.value.clone(),
            cursor_head: self.cursor_head,
            cursor_tail: self.cursor_tail,
        }
    }

    fn consume_undo_item(&mut self, item: UndoItem) {
        self.value = item.text;
        self.cursor_head = item.cursor_head;
        self.cursor_tail = item.cursor_tail;
    }

    pub fn change(&mut self, cx: &mut Cx, s: &str) -> bool {
        if self.disabled {
            return false;
        }
        self.replace_text(s);
        self.draw_input.redraw(cx);
        true
    }
    pub fn replace_text(&mut self, input: &str) {
        let mut new = String::new();
        let (left, right) = self.sorted_cursor();
        let mut chars_inserted = 0;
        let mut inserted = false;
        for (i, c) in self.value.chars().enumerate() {
            // cursor insertion point
            if i == left {
                inserted = true;
                for c in input.chars() {
                    chars_inserted += 1;
                    new.push(c);
                }
            }
            // outside of the selection so copy
            if i < left || i >= right {
                new.push(c);
            }
        }
        if !inserted {
            // end of string or empty string
            for c in input.chars() {
                chars_inserted += 1;
                new.push(c);
            }
        }
        self.cursor_head = left + chars_inserted;
        self.cursor_tail = self.cursor_head;
        self.value = new;
    }
    pub fn selected_text(&mut self) -> String {
        let mut ret = String::new();
        let (left, right) = self.sorted_cursor();
        for (i, c) in self.value.chars().enumerate() {
            if i >= left && i < right {
                ret.push(c);
            }
            if i >= right {
                break;
            }
        }
        ret
    }
    /// 选择文字按照范围
    pub fn select_ranged_text(&mut self, around: usize) {
        let mut first_ws = Some(0);
        let mut last_ws = None;
        let mut after_center = false;
        for (i, c) in self.value.chars().enumerate() {
            last_ws = Some(i + 1);
            if i >= around {
                after_center = true;
            }
            if c.is_whitespace() {
                last_ws = Some(i);
                if after_center {
                    break;
                }
                first_ws = Some(i + 1);
            }
        }
        if let Some(first_ws) = first_ws {
            if let Some(last_ws) = last_ws {
                self.cursor_tail = first_ws;
                self.cursor_head = last_ws;
            }
        }
    }
    pub fn push_change_action(&self, uid: WidgetUid, scope: &Scope, cx: &mut Cx) {
        cx.widget_action(
            uid,
            &scope.path,
            GTextInputEvent::Changed(self.value.clone()),
        );
    }
    pub fn undo(&mut self) {
        if let Some(item) = self.undo_stack.pop() {
            let redo_item = self.create_undo_item(item.undo_group);
            self.consume_undo_item(item.clone());
            self.redo_stack.push(redo_item);
        }
    }

    pub fn redo(&mut self) {
        if let Some(item) = self.redo_stack.pop() {
            let undo_item = self.create_undo_item(item.undo_group);
            self.consume_undo_item(item.clone());
            self.undo_stack.push(undo_item);
        }
    }
}
