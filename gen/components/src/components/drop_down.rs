use std::rc::Rc;

use icon_atlas::RefCell;

use makepad_widgets::*;

use crate::shader::manual::Position;

use super::{card::Card, popup::GPopup};

live_design! {
    GDropDownBase = {{GDropDown}} {}
}

#[derive(Live, Widget)]
pub struct GDropDown {
    #[deref]
    #[live]
    card: Card,
    #[live]
    popup: Option<LivePtr>,
    #[live]
    position: Position,
    #[rust]
    opened: bool,
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
        let _ = self.card.draw_walk(cx, scope, walk);

        cx.add_nav_stop(self.draw_card.area(), NavRole::DropDown, Margin::default());

        if self.opened && self.popup.is_some() {
            let global = cx.global::<PopupMenuGlobal>().clone();
            let mut map = global.map.borrow_mut();
            let popup_menu = map.get_mut(&self.popup.unwrap()).unwrap();
            popup_menu.begin(cx);

            match self.position {
                Position::Bottom => {
                    let area = self.draw_card.area().rect(cx);
                    let shift = DVec2 {
                        x: 0.0,
                        y: area.size.y,
                    };
                    popup_menu.draw_container(cx, scope);
                    popup_menu.end(cx, scope, self.draw_card.area(), shift);
                }
                _ => {}
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
                if !popup_menu.menu_contains_pos(cx, e.abs) {
                    self.close(cx);
                    self.animator_play(cx, id!(hover.off));
                    return;
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
