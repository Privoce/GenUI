pub mod event;
pub mod item;
pub mod options;
mod register;
pub mod types;

use std::{cell::RefCell, rc::Rc};

use event::*;
use options::GSelectOptions;
pub use register::register;

use makepad_widgets::*;
use types::SelectOption;

use crate::{
    shader::{draw_view::DrawGView, draw_text::DrawGText},
    themes::Themes,
    utils::{get_font_family, set_cursor, BoolToF32, ThemeColor},
    widget_area,
};

live_design! {
    GSelectBase = {{GSelect}}{
        height: 36.0,
        width: 180.0,
        border_width: 1.0,
        spread_radius: 2.2,
        shadow_offset: vec2(0.0, 2.0),
        blur_radius: 5.0,
        clip_x: false,
        clip_y: false,
        background_visible: false,
        cursor: Hand,
        align: {x: 0.0, y: 0.5},
        padding: 8.0,
        draw_text: {
            fn get_color(self) -> vec4{
                return self.color;
            }
        }
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_select: {focus: 0.0, hover: 0.0}
                        draw_text: {focus: 0.0, hover: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: 0.1}
                    }
                    apply: {
                        draw_select: {focus: 0.0, hover: 1.0}
                        draw_text: {focus: 0.0, hover: 1.0}
                    }
                }
            }
            // focus = {
            //     default: off
            //     off = {
            //         from: {all: Forward {duration: 0.2}}
            //         apply: {
            //             draw_select: {focus: 0.0},
            //             // draw_text: {focus: 0.0}
            //         }
            //     }
            //     on = {
            //         from: {all: Snap}
            //         apply: {
            //             draw_select: {focus: 1.0},
            //             // draw_text: {focus: 1.0}
            //         }
            //     }
            // }
        }
    }
}

#[derive(Live, Widget)]
pub struct GSelect {
    #[live]
    pub theme: Themes,
    #[live]
    pub color: Option<Vec4>,
    #[live]
    pub background_color: Option<Vec4>,
    #[live(true)]
    pub background_visible: bool,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub focus_color: Option<Vec4>,
    #[live]
    pub shadow_color: Option<Vec4>,
    #[live(4.8)]
    pub spread_radius: f32,
    #[live(4.8)]
    pub blur_radius: f32,
    #[live]
    pub shadow_offset: Vec2,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(0.0)]
    pub border_width: f32,
    #[live(2.0)]
    pub border_radius: f32,
    #[live]
    pub cursor: Option<MouseCursor>,
    #[redraw]
    #[live]
    pub draw_select: DrawGView,
    #[redraw]
    #[live]
    pub draw_text: DrawGText,
    #[live]
    pub select_options: Option<LivePtr>,
    #[live]
    pub select_item: Option<LivePtr>,
    #[live(10.0)]
    pub font_size: f64,
    // visible -------------------
    #[live(true)]
    pub visible: bool,
    #[live]
    pub opened: bool,
    #[live(6.0)]
    pub offset: f32,
    #[rust]
    pub options: Vec<SelectOption>,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[live]
    pub font_family: LiveDependency,
    #[live]
    pub selected: usize,
    #[animator]
    animator: Animator,
    #[live(true)]
    pub event_key: bool,
}

#[derive(Default, Clone)]
pub struct SelectOptionsGlobal {
    pub map: Rc<RefCell<ComponentMap<LivePtr, GSelectOptions>>>,
}

impl Widget for GSelect {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let _ = self.draw_select.begin(cx, walk, self.layout);

        if (self.options.len() >= self.selected) && self.options.len() != 0 {
            let font = get_font_family(&self.font_family, cx);
            self.draw_text.text_style.font = font;
            let text = self.options[self.selected].text.to_string();
            self.draw_text
                .draw_walk(cx, Walk::fit(), Align { x: 0.0, y: 0.5 }, &text);
        }
        self.draw_select.end(cx);

        cx.add_nav_stop(self.area(), NavRole::DropDown, Margin::default());

        if self.opened && self.select_options.is_some() {
            let global = cx.global::<SelectOptionsGlobal>().clone();
            let mut map = global.map.borrow_mut();
            let options_menu = map.get_mut(&self.select_options.unwrap()).unwrap();
            // begin draw options
            options_menu.begin(cx, self.theme);
            // set item live ptr and draw
            options_menu.item = self.select_item.clone();

            for (index, option) in self.options.iter().enumerate() {
                options_menu.draw_option(cx, LiveId(index as u64), &option.text, &option.value);
            }
            
            let _ = options_menu.end_container(cx);
            let area = self.area().rect(cx);
            let container_size = options_menu.area().rect(cx).size;

            let shift = DVec2 {
                x: area.size.x / 2.0 - container_size.x / 2.0,
                y: area.size.y + self.offset as f64,
            };
            options_menu.end(cx, scope, self.area(), shift);
        }
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.animator_handle_event(cx, event);
        let uid = self.widget_uid();
        if self.opened && self.select_options.is_some() {
            let global = cx.global::<SelectOptionsGlobal>().clone();
            let mut map = global.map.borrow_mut();
            let menu = map.get_mut(&self.select_options.unwrap()).unwrap();
            let mut close = false;

            menu.handle_event_with(cx, event, self.area(), &mut |cx, action| match action {
                GSelectOptionsEvent::Changed(e) => {
                    self.selected = e.selected_id;
                    cx.widget_action(uid, &scope.path, GSelectEvent::Changed(e));
                    self.draw_select.redraw(cx);
                    close = true;
                }
                _ => (),
            });
            if close {
                self.close(cx);
            }
            if let Event::MouseDown(e) = event {
                if !menu.menu_contains_pos(cx, e.abs) {
                    self.close(cx);
                    // self.animator_play(cx, id!(hover.off));
                }
            }
        }

        match event.hits_with_sweep_area(cx, self.area(), self.area()) {
            Hit::FingerHoverIn(_) => {
                set_cursor(cx, self.cursor.as_ref());
                self.animator_play(cx, id!(hover.on));
            }
            Hit::FingerUp(f) => {
                if f.is_over && f.device.has_hovers() {
                    set_cursor(cx, self.cursor.as_ref());
                    self.open(cx);
                    self.animator_play(cx, id!(hover.on));
                }
                if !f.is_over {
                    self.animator_play(cx, id!(hover.off));
                }
            }
            _ => (),
        }
    }
}

impl LiveHook for GSelect {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.visible {
            return;
        }
        let global = cx.global::<SelectOptionsGlobal>().clone();
        let mut map = global.map.borrow_mut();
        map.retain(|k, _| cx.live_registry.borrow().generation_valid(*k));
        let menu = self.select_options.unwrap();
        map.get_or_insert(cx, menu, |cx| GSelectOptions::new_from_ptr(cx, Some(menu)));
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.use_or("#ffffff");
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.use_or("#ffffff");
        // ------------------ focus color ---------------------------------------------
        let focus_color = self.focus_color.use_or("#ffffff");
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 600);
        let shadow_color = self.shadow_color.get(self.theme, 700);
        let background_visible = self.background_visible.to_f32();
        let color = self.color.use_or("#ADBAC7");
        self.draw_select.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                background_visible: (background_visible),
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
        self.draw_text.apply_over(
            cx,
            live! {
                color: (color),
                text_style: {
                    font_size: (self.font_size),
                }
            },
        );
        self.draw_text.redraw(cx);
        self.draw_select.redraw(cx);
    }
}

impl GSelect {
    widget_area! {
        area, draw_select
    }
    pub fn open(&mut self, cx: &mut Cx) {
        self.opened = true;
        self.draw_select.apply_over(cx, live! {focus: 1.0});
        self.draw_select.redraw(cx);
        // let global = cx.global::<PopupMenuGlobal>().clone();
        // let mut map = global.map.borrow_mut();
        // let lb = map.get_mut(&self.popup_menu.unwrap()).unwrap();
        // let node_id = LiveId(self.selected_item as u64).into();
        // lb.init_select_item(node_id);
        cx.sweep_lock(self.draw_select.area());
    }

    pub fn close(&mut self, cx: &mut Cx) {
        self.opened = false;
        self.draw_select.apply_over(cx, live! {focus: 0.0});
        self.draw_select.redraw(cx);
        cx.sweep_unlock(self.draw_select.area());
    }
}
