mod event;
mod register;

pub use event::*;
// use event::*;
use makepad_widgets::*;
pub use register::register;
use std::rc::Rc;

use crate::{
    ref_area, ref_redraw_mut,
    shader::manual::{CloseMode, PopupMode, Position, TriggerMode},
};
use icon_atlas::RefCell;

use super::{
    popup::{GPopup, GPopupContainer},
    view::GView,
};

live_design! {
    GDropDownBase = {{GDropDown}} {}
}

#[derive(Live, Widget)]
pub struct GDropDown {
    #[rust]
    pub mode: PopupMode,
    #[deref]
    pub deref_widget: GView,
    #[live]
    pub popup: Option<LivePtr>,
    #[live]
    pub position: Position,
    #[live]
    pub trigger_mode: TriggerMode,
    #[live]
    pub opened: bool,
    #[live(6.0)]
    pub offset: f32,
    #[live]
    pub offset_x: f32,
    #[live]
    pub offset_y: f32,
    // visible -------------------
    #[live(true)]
    pub visible: bool,
    /// if proportion > 1.0, we think that is height/width (dep on position)(TODO: fix this)
    #[live(0.4)]
    pub proportion: f32,
    #[live(true)]
    pub event_key: bool,
    #[rust]
    pub close_mode: CloseMode,
    #[rust(true)]
    pub redraw_flag: bool,
}

#[derive(Default, Clone)]
pub struct PopupMenuGlobal {
    pub map: Rc<RefCell<ComponentMap<LivePtr, GPopup>>>,
}

impl LiveHook for GDropDown {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
        if self.popup.is_none() || !apply.from.is_from_doc() {
            return;
        }
        let global = cx.global::<PopupMenuGlobal>().clone();
        let mut global_map = global.map.borrow_mut();
        global_map.retain(|k, _| cx.live_registry.borrow().generation_valid(*k));
        let popup = self.popup.unwrap();
        let popup = global_map.get_or_insert(cx, popup, |cx| GPopup::new_from_ptr(cx, Some(popup)));
        self.close_mode = popup.close_mode;
        self.mode = popup.mode;
    }
}

impl GDropDown {
    fn area(&self) -> Area {
        self.deref_widget.area
    }
    pub fn changed(&mut self, actions: &Actions) -> Option<GDropDownChangedParam> {
        if let GDropDownEvent::Changed(e) = actions.find_widget_action(self.widget_uid()).cast() {
            Some(e)
        } else {
            None
        }
    }
    pub fn redraw(&mut self, cx: &mut Cx) -> () {
        self.deref_widget.redraw(cx);
    }
    pub fn open(&mut self, cx: &mut Cx) {
        self.open_inner(cx, GDropDownToggleKind::Other);
    }
    pub fn close(&mut self, cx: &mut Cx) {
        // this close is virtual close
        if !self.opened {
            return;
        }
        // we don't need to care close mode here
        self.opened = false;
        self.redraw(cx);
        cx.sweep_unlock(self.area());
        self.active_toggled(cx, GDropDownToggleKind::Other);
        self.redraw_flag = true;
    }
    pub fn toggle(&mut self, cx: &mut Cx) {
        if self.opened {
            self.close(cx);
        } else {
            self.open(cx);
        }
    }
    /// open the popup only inner control
    fn open_inner(&mut self, cx: &mut Cx, e_kind: GDropDownToggleKind) {
        if self.opened {
            return;
        }
        self.opened = true;
        self.redraw(cx);
        cx.sweep_lock(self.area());
        self.active_toggled(cx, e_kind);
    }
    /// close the popup only inner control
    fn close_inner(&mut self, cx: &mut Cx, e_kind: GDropDownToggleKind, is_in: bool) {
        // here is a quick return to optimize
        if !self.opened {
            return;
        }
        let mut flag = false;
        match self.close_mode {
            CloseMode::Out => {
                if !is_in {
                    flag = true;
                }
            }
            CloseMode::Virtual => {
                flag = false;
            }
        }
        if flag {
            self.opened = false;
            self.redraw(cx);
            cx.sweep_unlock(self.area());
            self.active_toggled(cx, e_kind);
        }
        self.redraw_flag = true;
    }
    fn active_toggled(&mut self, cx: &mut Cx, e_kind: GDropDownToggleKind) {
        cx.widget_action(
            self.widget_uid(),
            self.scope_path.as_ref().unwrap(),
            GDropDownEvent::Changed(GDropDownChangedParam {
                e: e_kind,
                opened: self.opened,
            }),
        );
    }
    #[allow(dead_code)]
    fn toggle_inner(&mut self, cx: &mut Cx, e_kind: GDropDownToggleKind, is_in: bool) {
        // we should check the close mode to make sure the close is correct (but only when opened)
        if self.opened {
            self.close_inner(cx, e_kind, is_in);
        } else {
            // if not opened, we should open it
            self.open_inner(cx, e_kind);
        }
    }

    pub fn get<F>(&mut self, cx: &mut Cx, mut f: F) -> ()
    where
        F: FnMut(&mut Cx, &Self, &GPopupContainer),
    {
        let global = cx.global::<PopupMenuGlobal>().clone();
        let map = global.map.borrow_mut();
        let popup_menu = map.get(&self.popup.unwrap()).unwrap();
        let _ = f(cx, self, popup_menu.get());
    }
    pub fn get_mut<F>(&mut self, cx: &mut Cx, mut f: F) -> ()
    where
        F: FnMut(&mut Cx, &mut Self, &mut GPopupContainer),
    {
        let global = cx.global::<PopupMenuGlobal>().clone();
        let mut map = global.map.borrow_mut();
        let popup_menu = map.get_mut(&self.popup.unwrap()).unwrap();
        let _ = f(cx, self, popup_menu.get_mut());
    }
}

impl Widget for GDropDown {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        cx.add_nav_stop(self.area(), NavRole::DropDown, Margin::default());

        if self.opened && self.popup.is_some() {
            let global = cx.global::<PopupMenuGlobal>().clone();
            let mut map = global.map.borrow_mut();
            let popup_menu = map.get_mut(&self.popup.unwrap()).unwrap();
            popup_menu.begin(cx);

            match self.mode {
                PopupMode::Popup | PopupMode::ToolTip => {
                    let area = self.area().rect(cx);
                    let angle_offset = self.position.angle_offset(area.size);
                    popup_menu.draw_container(cx, scope, Some(self.position.clone()), angle_offset, &mut self.redraw_flag);
                    let container = popup_menu.container_area().rect(cx);
                    let mut shift = match self.position {
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

                    shift.x += self.offset_x as f64;
                    shift.y += self.offset_y as f64;

                    popup_menu.end(cx, scope, self.area(), shift);
                }

                PopupMode::Dialog => {
                    popup_menu.draw_container(cx, scope, None, 0.0, &mut false);
                    popup_menu.end(cx, scope, Area::Empty, DVec2::default());
                }
                PopupMode::Drawer => {
                    let _ =
                        popup_menu.draw_container_drawer(cx, scope, self.position, self.proportion, &mut self.redraw_flag);
                    popup_menu.end(cx, scope, Area::Empty, DVec2::default());
                }
            }
        }

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.opened && self.popup.is_some() {
            let global = cx.global::<PopupMenuGlobal>().clone();
            let mut map = global.map.borrow_mut();
            let popup_menu = map.get_mut(&self.popup.unwrap()).unwrap();
            popup_menu.handle_event_with(cx, event, scope, self.area());
            if let Event::MouseDown(e) = event {
                let is_in = popup_menu.menu_contains_pos(cx, e.abs);
                self.close_inner(cx, GDropDownToggleKind::Other, is_in);
                return;
            }
        }

        match event.hits_with_sweep_area(cx, self.area(), self.area()) {
            // template remove -------------------------------------------------------------------
            // Hit::KeyFocus(_) => {
            //     // self.animator_play(cx, id!(focus.on));
            // }
            // Hit::KeyFocusLost(k_e) => {
            //     // self.toggle_inner(cx, GDropDownToggleKind::KetFocusLost(k_e.clone()), false);
            //     // self.animator_play(cx, id!(hover.off));
            //     // self.draw_view.redraw(cx);
            // }
            // template remove -------------------------------------------------------------------
            Hit::FingerDown(e) => {
                cx.set_key_focus(self.area());
                if self.trigger_mode.is_press() {
                    self.open_inner(cx, GDropDownToggleKind::Press(e));
                }
            }
            Hit::FingerHoverIn(e) => {
                cx.set_cursor(MouseCursor::Hand);
                if self.trigger_mode.is_hover() {
                    self.open_inner(cx, GDropDownToggleKind::Hover(e));
                }
            }
            Hit::FingerHoverOut(f) => {
                cx.set_cursor(MouseCursor::Default);
                if self.trigger_mode.is_hover() {
                    self.close_inner(cx, GDropDownToggleKind::Hover(f), false);
                }
            }
            Hit::FingerUp(e) => {
                if e.is_over && self.trigger_mode.is_click() {
                    self.open_inner(cx, GDropDownToggleKind::Click(e));
                } else {
                    // focus lost
                    self.close_inner(cx, GDropDownToggleKind::Other, false);
                }
            }
            _ => {}
        }
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl GDropDownRef {
    ref_redraw_mut!();
    ref_area!();
    /// open the popup
    pub fn open(&mut self, cx: &mut Cx) {
        if let Some(mut c_ref) = self.borrow_mut() {
            c_ref.open(cx);
        }
    }
    /// close the popup
    pub fn close(&mut self, cx: &mut Cx) {
        if let Some(mut c_ref) = self.borrow_mut() {
            c_ref.close(cx);
        }
    }
    /// get the popup and inner container, the container is the real popup(you need to control)
    pub fn get<F>(&self, cx: &mut Cx, f: F) -> ()
    where
        F: FnMut(&mut Cx, &GDropDown, &GPopupContainer),
    {
        if let Some(mut c_ref) = self.borrow_mut() {
            c_ref.get(cx, f);
        }
    }
    /// ## get as mut ref
    /// ```rust
    /// let mut pop = self.gdrop_down(id!(pop));
    /// pop.get_mut(cx, |cx, pop, container| {
    ///     let close = container.gbutton(id!(close));
    ///
    ///     if close.clicked(&actions).is_some() {
    ///         pop.close(cx);
    ///     }
    /// });
    /// ```
    pub fn get_mut<F>(&mut self, cx: &mut Cx, f: F) -> ()
    where
        F: FnMut(&mut Cx, &mut GDropDown, &mut GPopupContainer),
    {
        if let Some(mut c_ref) = self.borrow_mut() {
            c_ref.get_mut(cx, f);
        }
    }
    /// ## toggle the popup
    /// If you don't know the state of the popup, you can use this method to toggle the popup
    ///
    /// This is a easy way to control the popup, and do not worry, open or close fn has been optimized
    pub fn toggle(&mut self, cx: &mut Cx) {
        if let Some(mut c_ref) = self.borrow_mut() {
            c_ref.toggle(cx);
        }
    }
    pub fn changed(&mut self, actions: &Actions) -> Option<GDropDownChangedParam> {
        if let Some(mut c_ref) = self.borrow_mut() {
            c_ref.changed(actions)
        } else {
            None
        }
    }
}
