pub mod event;
mod register;

// use event::*;
use makepad_widgets::*;
pub use register::register;
use std::rc::Rc;

use crate::{shader::manual::{PopupMode, Position}, widget_area};
use icon_atlas::RefCell;

use super::{card::GCard, popup::GPopup};

live_design! {
    GDropDownBase = {{GDropDown}} {}
}

#[derive(Live, Widget)]
pub struct GDropDown {
    #[live]
    pub mode: PopupMode,
    #[deref]
    #[live]
    pub card: GCard,
    #[live]
    pub popup: Option<LivePtr>,
    #[live]
    pub position: Position,
    #[rust]
    pub opened: bool,
    #[live(6.0)]
    pub offset: f32,
    // visible -------------------
    #[live(true)]
    pub visible: bool,
}

#[derive(Default, Clone)]
struct PopupMenuGlobal {
    map: Rc<RefCell<ComponentMap<LivePtr, GPopup>>>,
}

// #[derive(Clone, Debug, DefaultNone)]
// pub enum GDropDownEvent {
//     Clicked,
//     None
// }

impl LiveHook for GDropDown {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        if !self.visible {
            return;
        }
        self.card.after_apply(cx, apply, index, nodes);
        if self.popup.is_none() || !apply.from.is_from_doc() {
            return;
        }
        let global = cx.global::<PopupMenuGlobal>().clone();
        let mut global_map = global.map.borrow_mut();
        global_map.retain(|k, _| cx.live_registry.borrow().generation_valid(*k));
        let popup = self.popup.unwrap();
        global_map.get_or_insert(cx, popup, |cx| GPopup::new_from_ptr(cx, Some(popup)));
    }
}

impl GDropDown {
    widget_area! {
        area, draw_card
    }
    pub fn open(&mut self, cx: &mut Cx) {
        self.opened = true;
        self.draw_card.redraw(cx);
        cx.sweep_lock(self.draw_card.area());
    }
    pub fn close(&mut self, cx: &mut Cx) {
        self.opened = false;
        self.draw_card.redraw(cx);
        cx.sweep_unlock(self.draw_card.area());
    }
}

impl Widget for GDropDown {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        let _ = self.card.draw_walk(cx, scope, walk);

        cx.add_nav_stop(self.draw_card.area(), NavRole::DropDown, Margin::default());

        if self.opened && self.popup.is_some() {
            let global = cx.global::<PopupMenuGlobal>().clone();
            let mut map = global.map.borrow_mut();
            let popup_menu = map.get_mut(&self.popup.unwrap()).unwrap();
            popup_menu.begin(cx);

            if let PopupMode::Dialog = self.mode {
                let area = self.draw_card.area();
                let mut rect = area.rect(cx);
                rect.pos = DVec2::default();
                area.set_rect(cx, &rect);
                popup_menu.draw_container(cx, scope, None);
                popup_menu.end(cx, scope, area, DVec2::default());
            } else {
                let area = self.draw_card.area().rect(cx);
                popup_menu.draw_container(cx, scope, Some(self.position.clone()));
                let container = popup_menu.container_area().rect(cx);

                let shift = match self.position {
                    Position::Bottom => DVec2 {
                        x: -container.size.x / 2.0 + area.size.x / 2.0,
                        y: area.size.y + self.offset as f64,
                    },
                    Position::BottomLeft => DVec2 {
                        x: 0.0,
                        y: area.size.y + self.offset as f64,
                    },
                    Position::BottomRight => DVec2 {
                        x: area.size.x - container.size.x,
                        y: area.size.y + self.offset as f64,
                    },
                    Position::Top => DVec2 {
                        x: 0.0 - area.size.x / 2.0,
                        y: -self.offset as f64 - container.size.y,
                    },
                    Position::TopLeft => DVec2 {
                        x: 0.0,
                        y: -self.offset as f64 - container.size.y,
                    },
                    Position::TopRight => DVec2 {
                        x: area.size.x - container.size.x,
                        y: -self.offset as f64 - container.size.y,
                    },
                    Position::Left => DVec2 {
                        x: -self.offset as f64 - container.size.x,
                        y: area.size.y / 2.0 - container.size.y / 2.0,
                    },
                    Position::LeftTop => DVec2 {
                        x: -self.offset as f64 - container.size.x,
                        y: 0.0,
                    },
                    Position::LeftBottom => DVec2 {
                        x: -self.offset as f64 - container.size.x,
                        y: 0.0 - container.size.y + area.size.y,
                    },
                    Position::Right => DVec2 {
                        x: area.size.x + self.offset as f64,
                        y: area.size.y / 2.0 - container.size.y / 2.0,
                    },
                    Position::RightTop => DVec2 {
                        x: area.size.x + self.offset as f64,
                        y: 0.0,
                    },
                    Position::RightBottom => DVec2 {
                        x: area.size.x + self.offset as f64,
                        y: 0.0 - container.size.y + area.size.y,
                    },
                };

                popup_menu.end(cx, scope, self.draw_card.area(), shift);
            }
        }

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.opened && self.popup.is_some() {
            let global = cx.global::<PopupMenuGlobal>().clone();
            let mut map = global.map.borrow_mut();
            let popup_menu = map.get_mut(&self.popup.unwrap()).unwrap();
            popup_menu.handle_event_with(cx, event, scope, self.draw_card.area());
            if let Event::MouseDown(e) = event {
                // if !popup_menu.menu_contains_pos(cx, e.abs) {
                //     self.close(cx);
                //     self.animator_play(cx, id!(hover.off));
                //     return;
                // }
                if let PopupMode::Dialog = self.mode {
                    if !popup_menu.container_contains_pos(cx, e.abs) {
                        self.close(cx);
                        self.animator_play(cx, id!(hover.off));
                        return;
                    }
                } else {
                    if !popup_menu.menu_contains_pos(cx, e.abs) {
                        self.close(cx);
                        self.animator_play(cx, id!(hover.off));
                        return;
                    }
                }
            }
        }

        match event.hits_with_sweep_area(cx, self.draw_card.area(), self.draw_card.area()) {
            Hit::KeyFocus(_) => {
                // self.animator_play(cx, id!(focus.on));
            }
            Hit::KeyFocusLost(_) => {
                self.close(cx);
                self.animator_play(cx, id!(hover.off));
                self.draw_card.redraw(cx);
            }
            Hit::FingerDown(_) => {
                cx.set_key_focus(self.draw_card.area());
                self.open(cx);
            }
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
                self.animator_play(cx, id!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                cx.set_cursor(MouseCursor::Default);
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerUp(f) => {
                if f.is_over && f.device.has_hovers() {
                    self.animator_play(cx, id!(hover.on));
                }
                if !f.is_over {
                    self.animator_play(cx, id!(hover.off));
                }
            }
            _ => {}
        }
    }
}
