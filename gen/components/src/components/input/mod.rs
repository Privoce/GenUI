mod register;
pub mod types;

use makepad_widgets::*;
pub use register::register;
use shader::draw_text::TextWrap;
use types::{Edit, EditKind, History};
use unicode_segmentation::{GraphemeCursor, UnicodeSegmentation};

use crate::{
    animatie_fn, event_bool, event_option, ref_event_bool, ref_event_option, set_event, set_event_bool, shader::{draw_view::DrawGView, draw_text::DrawGText}, themes::Themes, utils::{get_font_family, BoolToF32, ThemeColor}, widget_area
};

live_design! {
    import makepad_draw::shader::std::*;
    GInputBase = {{GInput}}{
        background_color: vec4(1.0, 1.0, 1.0, 1.0),
        hover_color: vec4(0.9, 0.9, 0.9, 1.0),
        focus_color: vec4(0.9, 0.9, 0.9, 1.0),
        text_hover_color: vec4(0.2, 0.2, 0.2, 1.0),
        text_focus_color: vec4(0.2, 0.2, 0.2, 1.0),
        shadow_offset: vec2(0.0, 0.0),
        color: #667085,
        height: Fill,
        width: 180.0,
        // align: {x: 0.0, y: 0.0},
        padding: 8.6,
        clip_x: false,
        clip_y: false,
        placeholder: "Please Input",
        text_align: {y: 0.},
        read_only: false,
        numeric_only: false,
        animator: {
            hover = {
                default: off
                off = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_selection: {hover: 0.0},
                        draw_text: {hover: 0.0},
                        draw_input: {hover: 0.0},
                    }
                }
                on = {
                    from: {all: Snap}
                    apply: {
                        draw_selection: {hover: 1.0},
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
                        draw_selection: {focus: 0.0}
                        draw_text: {focus: 0.0}
                    }
                }
                on = {
                    from: {all: Snap}
                    apply: {
                        draw_cursor: {focus: 1.0},
                        draw_input: {focus: 1.0},
                        draw_selection: {focus: 1.0}
                        draw_text: {focus: 1.0}
                    }
                }
            }
        },

        draw_text: {
            // instance focus: 0.0;
            instance placeholder_color: vec4;
            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        self.color,
                        mix(self.stroke_hover_color, self.stroke_focus_color, self.focus),
                        self.hover
                    ),
                    self.placeholder_color,
                    self.empty
                )
            }
        }

        draw_cursor: {
            // instance focus: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    0.,
                    0.,
                    self.rect_size.x,
                    self.rect_size.y,
                    self.border_radius
                )
                sdf.fill(self.get_background_color());
                return sdf.result;
            }
        }

        // draw_selection: {
        //     instance hover: 0.0
        //     instance focus: 0.0

        //     fn pixel(self) -> vec4 {
        //         //return mix(#f00,#0f0,self.pos.y)
        //         let sdf = Sdf2d::viewport(self.pos * self.rect_size);
        //         sdf.box(
        //             0.,
        //             0.,
        //             self.rect_size.x,
        //             self.rect_size.y,
        //             0.5
        //         )
        //         sdf.fill(
        //             self.get_color()
        //         );
        //         return sdf.result
        //     }
        // }

    }
}

#[derive(Live, Widget)]
pub struct GInput {
    #[live]
    pub theme: Themes,
    #[live]
    pub shadow_color: Option<Vec4>,
    #[live(0.0)]
    pub spread_radius: f32,
    #[live(4.8)]
    pub blur_radius: f32,
    #[live]
    pub shadow_offset: Vec2,
    #[live]
    pub placeholder_color: Option<Vec4>,
    #[live]
    pub color: Option<Vec4>,
    #[live]
    pub cursor_color: Option<Vec4>,
    #[live]
    pub select_color: Option<Vec4>,
    #[live]
    pub background_color: Option<Vec4>,
    #[live(true)]
    pub background_visible: bool,
    #[live(true)]
    pub visible: bool,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub text_hover_color: Option<Vec4>,
    #[live]
    pub text_focus_color: Option<Vec4>,
    #[live]
    pub cursor_hover_color: Option<Vec4>,
    #[live]
    pub cursor_focus_color: Option<Vec4>,
    #[live]
    pub select_hover_color: Option<Vec4>,
    #[live]
    pub select_focus_color: Option<Vec4>,
    #[live]
    pub focus_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(1.0)]
    pub border_width: f32,
    #[live(2.0)]
    pub border_radius: f32,
    // text --------------------
    #[live]
    pub text_align: Align,
    #[live(9.0)]
    pub font_size: f64,
    #[live(1.0)]
    pub brightness: f32,
    #[live(0.5)]
    pub curve: f32,
    // #[live(1.2)]
    // pub top_drop: f64,
    #[live(1.3)]
    pub height_factor: f64,
    #[live(TextWrap::Word)]
    pub wrap: TextWrap,
    #[live]
    pub font_family: LiveDependency,
    #[live(1.0)]
    cursor_border_radius: f64,
    // deref --------------
    #[animator]
    animator: Animator,
    #[redraw]
    #[live]
    draw_input: DrawGView,
    #[live]
    draw_text: DrawGText,
    #[live]
    draw_selection: DrawGView,
    #[live]
    draw_cursor: DrawGView,
    #[layout]
    layout: Layout,
    #[walk]
    walk: Walk,
    #[live(2.0)]
    cursor_width: f64,
    #[live]
    pub read_only: bool,
    #[live]
    pub numeric_only: bool,
    #[live]
    pub placeholder: String,
    #[live]
    pub text: String,
    #[rust]
    cursor: Cursor,
    #[rust]
    history: History,
    #[live]
    scroll_bars: ScrollBars,
    #[live(true)]
    pub event_key: bool,
}

impl Widget for GInput {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        // self.draw_text.wrap = self.wrap.clone();
        self.draw_text.text_style.font = get_font_family(&self.font_family, cx);
        self.draw_input.begin(cx, walk, self.layout);

        self.draw_selection.append_to_draw_call(cx);

        // Draw text
        if self.text.is_empty() {
            // self.draw_text.empty = 1.0;
            self.draw_text
                .draw_walk(cx, Walk::fill(), self.text_align, &self.placeholder);
        } else {
            // self.draw_text.empty = 0.0;
            self.draw_text
                .draw_walk(cx, Walk::fill(), self.text_align, &self.text);
        }

        let padded_rect = cx.turtle().padded_rect();

        // Draw selection
        let rects = self.draw_text.selected_rects(
            cx,
            Walk::fill(),
            self.text_align,
            padded_rect.size.x,
            &self.text,
            self.cursor.head.min(self.cursor.tail),
            self.cursor.head.max(self.cursor.tail),
        );
        for rect in rects {
            self.draw_selection.draw_abs(
                cx,
                Rect {
                    pos: padded_rect.pos + rect.pos,
                    size: rect.size,
                },
            );
        }

        // Draw cursor
        let cursor_position = self.cursor_position(cx, padded_rect.size.x);
        let cursor_height = self.draw_text.line_height(cx);
        self.draw_cursor.draw_abs(
            cx,
            Rect {
                pos: padded_rect.pos
                    + dvec2(
                        cursor_position.x - 0.5 * self.cursor_width,
                        cursor_position.y,
                    ),
                size: dvec2(self.cursor_width, cursor_height),
            },
        );

        self.draw_input.end(cx);

        if cx.has_key_focus(self.draw_input.area()) {
            let padding = dvec2(self.layout.padding.left, self.layout.padding.top);
            cx.show_text_ime(
                self.draw_input.area(),
                padding + cursor_position - self.cursor_width * 0.5,
            );
        }

        cx.add_nav_stop(
            self.draw_input.area(),
            NavRole::TextInput,
            Margin::default(),
        );

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let rect = self.draw_input.area().rect(cx);
        let padded_rect = Rect {
            pos: rect.pos + self.layout.padding.left_top(),
            size: rect.size - self.layout.padding.size(),
        };

        let uid = self.widget_uid();

        if self.animator_handle_event(cx, event).must_redraw() {
            self.draw_input.redraw(cx);
        }

        match event.hits(cx, self.draw_input.area()) {
            Hit::KeyFocus(_) => {
                self.animator_play(cx, id!(focus.on));
                self.force_new_edit_group();
                // TODO: Select all if necessary
                cx.widget_action(uid, &scope.path, TextInputAction::KeyFocus);
            }
            Hit::KeyFocusLost(_) => {
                self.animator_play(cx, id!(focus.off));
                cx.hide_text_ime();
                cx.widget_action(uid, &scope.path, TextInputAction::KeyFocusLost);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::ArrowLeft,
                modifiers: KeyModifiers {
                    shift: is_select, ..
                },
                ..
            }) => {
                self.move_cursor_left(is_select);
                self.draw_input.redraw(cx);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::ArrowRight,
                modifiers: KeyModifiers {
                    shift: is_select, ..
                },
                ..
            }) => {
                self.move_cursor_right(is_select);
                self.draw_input.redraw(cx);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::ArrowUp,
                modifiers: KeyModifiers {
                    shift: is_select, ..
                },
                ..
            }) => {
                let event = DrawEvent::default();
                let mut cx = Cx2d::new(cx, &event);
                self.move_cursor_up(&mut cx, padded_rect.size.x, is_select);
                self.draw_input.redraw(&mut cx);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::ArrowDown,
                modifiers: KeyModifiers {
                    shift: is_select, ..
                },
                ..
            }) => {
                let event = DrawEvent::default();
                let mut cx = Cx2d::new(cx, &event);
                self.move_cursor_down(&mut cx, padded_rect.size.x, is_select);
                self.draw_input.redraw(&mut cx);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::Home,
                ..
            }) => {
                self.move_cursor_to(
                    IndexAffinity {
                        index: 0,
                        affinity: Affinity::Before,
                    },
                    false,
                );
                self.history.force_new_edit_group();
                self.draw_input.redraw(cx);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::End,
                ..
            }) => {
                self.move_cursor_to(
                    IndexAffinity {
                        index: self.text.len(),
                        affinity: Affinity::After,
                    },
                    false,
                );
                self.history.force_new_edit_group();
                self.draw_input.redraw(cx);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::ReturnKey,
                modifiers: KeyModifiers { shift: false, .. },
                ..
            }) => {
                cx.hide_text_ime();
                cx.widget_action(uid, &scope.path, TextInputAction::Return(self.text.clone()));
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::ReturnKey,
                modifiers: KeyModifiers { shift: true, .. },
                ..
            }) if !self.read_only => {
                self.history
                    .create_or_extend_edit_group(EditKind::Other, self.cursor);
                self.apply_edit(Edit {
                    start: self.cursor.start().index,
                    end: self.cursor.end().index,
                    replace_with: "\n".to_string(),
                });
                self.draw_input.redraw(cx);
                cx.widget_action(uid, &scope.path, TextInputAction::Change(self.text.clone()));
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::Escape,
                ..
            }) => {
                cx.widget_action(uid, &scope.path, TextInputAction::Escape);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::Backspace,
                ..
            }) if !self.read_only => {
                let mut start = self.cursor.start().index;
                let end = self.cursor.end().index;
                if start == end {
                    start = prev_grapheme_boundary(&self.text, start).unwrap_or(0);
                }
                self.history
                    .create_or_extend_edit_group(EditKind::Backspace, self.cursor);
                self.apply_edit(Edit {
                    start,
                    end,
                    replace_with: String::new(),
                });
                self.draw_input.redraw(cx);
                cx.widget_action(uid, &scope.path, TextInputAction::Change(self.text.clone()));
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::Delete,
                ..
            }) if !self.read_only => {
                let start = self.cursor.start().index;
                let mut end = self.cursor.end().index;
                if start == end {
                    end = next_grapheme_boundary(&self.text, end).unwrap_or(self.text.len());
                }
                self.history
                    .create_or_extend_edit_group(EditKind::Delete, self.cursor);
                self.apply_edit(Edit {
                    start,
                    end,
                    replace_with: String::new(),
                });
                self.draw_input.redraw(cx);
                cx.widget_action(uid, &scope.path, TextInputAction::Change(self.text.clone()));
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::KeyA,
                modifiers: KeyModifiers { control: true, .. },
                ..
            })
            | Hit::KeyDown(KeyEvent {
                key_code: KeyCode::KeyA,
                modifiers: KeyModifiers { logo: true, .. },
                ..
            }) => {
                self.select_all();
                self.draw_input.redraw(cx);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::KeyZ,
                modifiers:
                    KeyModifiers {
                        logo: true,
                        shift: false,
                        ..
                    },
                ..
            }) if !self.read_only => {
                self.undo();
                self.draw_input.redraw(cx);
                cx.widget_action(uid, &scope.path, TextInputAction::Change(self.text.clone()));
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::KeyZ,
                modifiers:
                    KeyModifiers {
                        logo: true,
                        shift: true,
                        ..
                    },
                ..
            }) if !self.read_only => {
                self.redo();
                self.draw_input.redraw(cx);
                cx.widget_action(uid, &scope.path, TextInputAction::Change(self.text.clone()));
            }
            Hit::TextInput(TextInputEvent {
                input,
                replace_last,
                was_paste,
                ..
            }) if !self.read_only => {
                let input = self.filter_input(input);
                if !input.is_empty() {
                    let mut start = self.cursor.start().index;
                    let end = self.cursor.end().index;
                    if replace_last {
                        start -= self
                            .history
                            .last_inserted_text(&self.text)
                            .map_or(0, |text| text.len());
                    }
                    self.history.create_or_extend_edit_group(
                        if replace_last || was_paste {
                            EditKind::Other
                        } else {
                            EditKind::Insert
                        },
                        self.cursor,
                    );
                    self.apply_edit(Edit {
                        start,
                        end,
                        replace_with: input,
                    });
                    self.draw_input.redraw(cx);
                    cx.widget_action(uid, &scope.path, TextInputAction::Change(self.text.clone()));
                }
            }
            Hit::TextCopy(event) => {
                let selection = &self.text[self.cursor.start().index..self.cursor.end().index];
                *event.response.borrow_mut() = Some(selection.to_string());
            }
            Hit::TextCut(event) => {
                let selection = &self.text[self.cursor.start().index..self.cursor.end().index];
                *event.response.borrow_mut() = Some(selection.to_string());
                if !selection.is_empty() {
                    self.history
                        .create_or_extend_edit_group(EditKind::Other, self.cursor);
                    self.apply_edit(Edit {
                        start: self.cursor.start().index,
                        end: self.cursor.end().index,
                        replace_with: String::new(),
                    });
                    self.draw_input.redraw(cx);
                    cx.widget_action(uid, &scope.path, TextInputAction::Change(self.text.clone()));
                }
            }
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Text);
                self.animator_play(cx, id!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerDown(FingerDownEvent { abs, tap_count, .. }) => {
                let event = DrawEvent::default();
                let mut cx = Cx2d::new(cx, &event);
                let index_affinity = self.position_to_index_affinity(
                    &mut cx,
                    padded_rect.size.x,
                    abs - padded_rect.pos,
                );
                self.move_cursor_to(index_affinity, false);
                if tap_count == 2 {
                    self.select_word();
                } else if tap_count == 3 {
                    self.select_all();
                }
                self.set_key_focus(&mut *cx);
                self.draw_input.redraw(&mut *cx);
            }
            Hit::FingerMove(FingerMoveEvent { abs, tap_count, .. }) => {
                let event: DrawEvent = DrawEvent::default();
                let mut cx = Cx2d::new(cx, &event);
                let index_affinity = self.position_to_index_affinity(
                    &mut cx,
                    padded_rect.size.x,
                    abs - padded_rect.pos,
                );
                self.move_cursor_to(index_affinity, true);
                if tap_count == 2 {
                    self.select_word();
                } else if tap_count == 3 {
                    self.select_all();
                }
                self.draw_input.redraw(&mut *cx);
            }
            _ => {}
        }
    }
    fn text(&self) -> String {
        self.text.to_string()
    }

    fn set_text(&mut self, text: &str) {
        if self.text == text {
            return;
        }
        self.text = self.filter_input(text.to_string());
        self.cursor.head.index = self.cursor.head.index.min(text.len());
        self.cursor.tail.index = self.cursor.tail.index.min(text.len());
        self.history.clear();
    }

    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GInput {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.visible {
            return;
        }
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.get(self.theme, 25);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.get(self.theme, 25);
        let shadow_color = self.shadow_color.get(self.theme, 700);
        let text_hover_color = self.text_hover_color.get(self.theme, 600);
        let text_focus_color = self.text_focus_color.get(self.theme, 800);
        let cursor_color = self.cursor_color.get(self.theme, 800);
        let cursor_hover_color = self.cursor_hover_color.get(self.theme, 800);
        let cursor_focus_color = self.cursor_focus_color.get(self.theme, 800);
        let select_color = self.select_color.get(self.theme, 400);
        let select_hover_color = self.select_hover_color.get(self.theme, 300);
        let select_focus_color = self.select_focus_color.get(self.theme, 500);
        let placeholder_color = self.placeholder_color.use_or("#98A2B3");
        // ------------------ focus color ---------------------------------------------
        let focus_color = self.focus_color.get(self.theme, 25);
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 400);
        // ------------------ font ------------------------------------------------------
        let font_color = self.color.get(self.theme, 800);
        // ---------------------- is empty ------------------------------------------------
        let empty = self.text.len().eq(&0).to_f32();
        // draw input --------------------------------------------------------------
        self.draw_input.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                background_visible: (self.background_visible.to_f32()),
                border_color: (border_color),
                border_width: (self.border_width),
                border_radius: (self.border_radius),
                focus_color: (focus_color),
                hover_color: (hover_color),
                shadow_color: (shadow_color),
                shadow_offset: (self.shadow_offset),
                spread_radius: (self.spread_radius),
                blur_radius: (self.blur_radius)
            },
        );
        // draw text ---------------------------------------------------------------
        self.draw_text.apply_over(
            cx,
            live! {
                color: (font_color),
                stroke_hover_color: (text_hover_color),
                stroke_focus_color: (text_focus_color),
                placeholder_color:(placeholder_color),
                empty: (empty),
                text_style: {
                    // brightness: (self.brightness),
                    // curve: (self.curve),
                    // line_spacing: (self.layout.line_spacing),
                    // top_drop: (self.top_drop),
                    font_size: (self.font_size),
                    // height_factor: (self.height_factor),
                }
            },
        );
        self.draw_text.wrap = self.wrap.clone();
        // draw cursor -------------------------------------------------------------
        self.draw_cursor.apply_over(
            cx,
            live! {
                background_color: (cursor_color),
                // border_color: (border_color),
                // border_width: (self.border_width),
                border_radius: (self.cursor_border_radius),
                focus_color: (cursor_focus_color),
                hover_color: (cursor_hover_color),
            },
        );
        // draw select -------------------------------------------------------------
        self.draw_selection.apply_over(
            cx,
            live! {
                background_color: (select_color),
                background_visible: 1.0,
                focus_color: (select_focus_color),
                hover_color: (select_hover_color),
                border_radius: 0.0
            },
        );
        // self.draw_text.redraw(cx);
        // self.draw_input.redraw(cx);
        // self.draw_cursor.redraw(cx);
        // self.draw_selection.redraw(cx);
    }
}

impl GInput {
    widget_area! {
        area, draw_input,
        area_selection, draw_selection
    }
    event_option! {
        change: TextInputAction::Change => String,
        r#return: TextInputAction::Return => String
    }
    event_bool! {
        key_focus: TextInputAction::KeyFocus,
        key_focus_lost: TextInputAction::KeyFocusLost,
        escape: TextInputAction::Escape
    }
    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
        self.draw_input.apply_over(
            cx,
            live! {
                hover: 1.0,
                focus: 0.0
            },
        );
        self.draw_cursor.apply_over(
            cx,
            live! {
                hover: 1.0,
                focus: 0.0
            },
        );
        self.draw_selection.apply_over(
            cx,
            live! {
                hover: 1.0,
                focus: 0.0
            },
        );
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 1.0,
                focus: 0.0
            },
        );
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) -> () {
        self.draw_input.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
        self.draw_cursor.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
        self.draw_selection.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
    }
    pub fn animate_focus_on(&mut self, cx: &mut Cx) -> () {
        self.draw_input.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 1.0
            },
        );
        self.draw_cursor.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 1.0
            },
        );
        self.draw_selection.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 1.0
            },
        );
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 1.0
            },
        );
    }

    pub fn animate_focus_off(&mut self, cx: &mut Cx) -> () {
        self.draw_input.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
        self.draw_cursor.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
        self.draw_selection.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
    }

    pub fn set_key_focus(&self, cx: &mut Cx) {
        cx.set_key_focus(self.draw_input.area());
    }

    pub fn set_cursor(&mut self, cursor: Cursor) {
        self.cursor = cursor;
    }

    pub fn select_all(&mut self) {
        self.set_cursor(Cursor {
            head: IndexAffinity {
                index: self.text.len(),
                affinity: Affinity::After,
            },
            tail: IndexAffinity {
                index: 0,
                affinity: Affinity::Before,
            },
        });
    }

    pub fn filter_input(&mut self, input: String) -> String {
        if self.numeric_only {
            input
                .chars()
                .filter_map(|char| match char {
                    '.' | ',' => Some('.'),
                    char if char.is_ascii_digit() => Some(char),
                    _ => None,
                })
                .collect()
        } else {
            input
        }
    }

    pub fn force_new_edit_group(&mut self) {
        self.history.force_new_edit_group();
    }

    fn position_to_index_affinity(
        &self,
        cx: &mut Cx2d,
        width: f64,
        position: DVec2,
    ) -> IndexAffinity {
        self.draw_text.position_to_index_affinity(
            cx,
            Walk::fill(),
            self.text_align,
            width,
            &self.text,
            position,
        )
    }

    fn cursor_position(&self, cx: &mut Cx2d, width: f64) -> DVec2 {
        self.draw_text.index_affinity_to_position(
            cx,
            Walk::fill(),
            self.text_align,
            width,
            &self.text,
            self.cursor.head,
        )
    }

    fn move_cursor_left(&mut self, is_select: bool) {
        let Some(index) = prev_grapheme_boundary(&self.text, self.cursor.head.index) else {
            return;
        };
        self.move_cursor_to(
            IndexAffinity {
                index,
                affinity: Affinity::After,
            },
            is_select,
        );
    }

    fn move_cursor_right(&mut self, is_select: bool) {
        let Some(index) = next_grapheme_boundary(&self.text, self.cursor.head.index) else {
            return;
        };
        self.move_cursor_to(
            IndexAffinity {
                index,
                affinity: Affinity::Before,
            },
            is_select,
        );
    }

    fn move_cursor_up(&mut self, cx: &mut Cx2d, width: f64, is_select: bool) {
        let position = self.cursor_position(cx, width);
        let line_spacing = self.draw_text.line_spacing(cx);
        let index_affinity = self.position_to_index_affinity(
            cx,
            width,
            DVec2 {
                x: position.x,
                y: position.y - 0.5 * line_spacing,
            },
        );
        self.move_cursor_to(index_affinity, is_select)
    }

    fn move_cursor_down(&mut self, cx: &mut Cx2d, width: f64, is_select: bool) {
        let position = self.cursor_position(cx, width);
        let line_spacing = self.draw_text.line_spacing(cx);
        let index_affinity = self.position_to_index_affinity(
            cx,
            width,
            DVec2 {
                x: position.x,
                y: position.y + 1.5 * line_spacing,
            },
        );
        self.move_cursor_to(index_affinity, is_select);
    }

    fn move_cursor_to(&mut self, index_affinity: IndexAffinity, is_select: bool) {
        self.cursor.head = index_affinity;
        if !is_select {
            self.cursor.tail = self.cursor.head;
        }
        self.history.force_new_edit_group();
    }

    fn select_word(&mut self) {
        if self.cursor.head.index < self.cursor.tail.index {
            self.cursor.head = IndexAffinity {
                index: self.ceil_word_boundary(self.cursor.head.index),
                affinity: Affinity::After,
            };
        } else if self.cursor.head.index > self.cursor.tail.index {
            self.cursor.head = IndexAffinity {
                index: self.floor_word_boundary(self.cursor.head.index),
                affinity: Affinity::Before,
            };
        } else {
            self.cursor.tail = IndexAffinity {
                index: self.ceil_word_boundary(self.cursor.head.index),
                affinity: Affinity::After,
            };
            self.cursor.head = IndexAffinity {
                index: self.floor_word_boundary(self.cursor.head.index),
                affinity: Affinity::Before,
            };
        }
    }

    fn ceil_word_boundary(&self, index: usize) -> usize {
        let mut prev_word_boundary_index = 0;
        for (word_boundary_index, _) in self.text.split_word_bound_indices() {
            if word_boundary_index > index {
                return prev_word_boundary_index;
            }
            prev_word_boundary_index = word_boundary_index;
        }
        prev_word_boundary_index
    }

    fn floor_word_boundary(&self, index: usize) -> usize {
        let mut prev_word_boundary_index = self.text.len();
        for (word_boundary_index, _) in self.text.split_word_bound_indices().rev() {
            if word_boundary_index < index {
                return prev_word_boundary_index;
            }
            prev_word_boundary_index = word_boundary_index;
        }
        prev_word_boundary_index
    }

    fn apply_edit(&mut self, edit: Edit) {
        self.cursor.head.index = edit.start + edit.replace_with.len();
        self.cursor.tail = self.cursor.head;
        self.history.apply_edit(edit, &mut self.text);
    }

    fn undo(&mut self) {
        if let Some(cursor) = self.history.undo(self.cursor, &mut self.text) {
            self.cursor = cursor;
        }
    }

    fn redo(&mut self) {
        if let Some(cursor) = self.history.redo(self.cursor, &mut self.text) {
            self.cursor = cursor;
        }
    }
}

impl GInputRef {
    ref_event_bool! {
        key_focus,
        key_focus_lost,
        escape
    }
    ref_event_option! {
        change => String,
        r#return => String
    }
    animatie_fn! {
        animate_hover_on,
        animate_hover_off,
        animate_focus_on,
        animate_focus_off
    }
    pub fn changed(&self, actions: &Actions) -> Option<String> {
        if let TextInputAction::Change(val) = actions.find_widget_action_cast(self.widget_uid()) {
            return Some(val);
        }
        None
    }

    pub fn returned(&self, actions: &Actions) -> Option<String> {
        if let TextInputAction::Return(val) = actions.find_widget_action_cast(self.widget_uid()) {
            return Some(val);
        }
        None
    }

    pub fn set_cursor(&self, head: usize, tail: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_cursor(Cursor {
                head: IndexAffinity {
                    index: head,
                    affinity: Affinity::After,
                },
                tail: IndexAffinity {
                    index: tail,
                    affinity: Affinity::Before,
                },
            });
        }
    }

    pub fn set_key_focus(&self, cx: &mut Cx) {
        if let Some(inner) = self.borrow() {
            inner.set_key_focus(cx);
        }
    }
}

impl GInputSet {
    set_event_bool! {
        key_focus,
        key_focus_lost,
        escape
    }
    set_event! {
        change => String,
        r#return => String
    }
}

fn next_grapheme_boundary(string: &str, index: usize) -> Option<usize> {
    let mut cursor = GraphemeCursor::new(index, string.len(), true);
    cursor.next_boundary(string, 0).unwrap()
}

fn prev_grapheme_boundary(string: &str, index: usize) -> Option<usize> {
    let mut cursor = GraphemeCursor::new(index, string.len(), true);
    cursor.prev_boundary(string, 0).unwrap()
}
