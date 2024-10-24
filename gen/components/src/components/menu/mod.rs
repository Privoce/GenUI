pub mod event;
pub mod menu_item;
mod register;
pub mod sub_menu;

use super::view::GView;
use crate::{
    event_option, ref_event_option, set_event, shader::{draw_view::DrawGView, manual::MenuItemMode}, themes::Themes, utils::{BoolToF32, ThemeColor}
};
use event::{GMenuChangedParam, GMenuEvent};
use makepad_widgets::*;
use menu_item::GMenuItemWidgetRefExt;
pub use register::register;
use sub_menu::GSubMenuWidgetRefExt;

live_design! {
    GMenuBase = {{GMenu}}{

    }
}

#[derive(Live, Widget)]
pub struct GMenu {
    #[live(Themes::Dark)]
    pub theme: Themes,
    #[live]
    pub background_color: Option<Vec4>,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub focus_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(0.0)]
    pub border_width: f32,
    #[live(0.0)]
    pub border_radius: f32,
    #[live(true)]
    pub background_visible: bool,
    #[live]
    pub shadow_color: Option<Vec4>,
    #[live(4.8)]
    pub spread_radius: f32,
    #[live(4.8)]
    pub blur_radius: f32,
    #[live]
    pub shadow_offset: Vec2,
    #[live]
    pub cursor: Option<MouseCursor>,
    #[live(true)]
    pub visible: bool,
    #[redraw]
    #[live]
    pub draw_menu: DrawGView,
    #[find]
    #[live]
    #[redraw]
    pub header: GView,
    #[find]
    #[live]
    #[redraw]
    pub footer: GView,
    #[find]
    #[live]
    #[redraw]
    pub body: GView,
    #[layout]
    pub layout: Layout,
    #[walk]
    pub walk: Walk,
    #[rust]
    pub selected: Option<Vec<usize>>,
    #[rust]
    pub item_modes: Vec<MenuItemMode>,
    #[live(true)]
    pub event_key: bool,
}

impl Widget for GMenu {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.is_visible() {
            return DrawStep::done();
        }

        self.draw_menu.begin(cx, walk, self.layout);
        if self.header.is_visible() {
            let header_walk = self.header.walk(cx);
            let _ = self.header.draw_walk(cx, scope, header_walk);
        }
        if self.body.is_visible() {
            let body_walk = self.body.walk(cx);
            let _ = self.body.draw_walk(cx, scope, body_walk);
        }
        if self.footer.is_visible() {
            let footer_walk = self.footer.walk(cx);
            let _ = self.footer.draw_walk(cx, scope, footer_walk);
        }
        self.draw_menu.end(cx);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.is_visible() {
            return;
        }
        if self.header.is_visible() {
            self.header.handle_event(cx, event, scope);
        }
        if self.body.is_visible() {
            let actions = cx.capture_actions(|cx| self.body.handle_event(cx, event, scope));
            let mut fresh = None;
            for (index, ((id, child), item_mode)) in self
                .body
                .children
                .iter()
                .zip(self.item_modes.iter())
                .enumerate()
            {
                match item_mode {
                    MenuItemMode::SubMenu(_) => {
                        child.as_gsub_menu().borrow().map(|sub_menu| {
                            if let Some(e) = sub_menu.changed(&actions) {
                                fresh.replace((e.selected_id, e.e));
                                let mut selected = vec![index];
                                selected.extend(e.selected.unwrap());
                                self.selected.replace(selected);
                            }
                        });
                    }
                    MenuItemMode::MenuItem(_) => {
                        child.as_gmenu_item().borrow().map(|item| {
                            if let Some(e) = item.clicked(&actions) {
                                if e.selected {
                                    // means need to change self.selected
                                    self.selected.replace(vec![index]);
                                    // do fresh selected
                                    fresh.replace((id.clone(), e.e));
                                }
                            }
                        });
                    }
                }
                if fresh.is_some() {
                    break;
                }
            }
            if let Some((id, e)) = fresh {
                self.fresh_selected(cx);

                cx.widget_action(
                    self.widget_uid(),
                    &scope.path,
                    GMenuEvent::Changed(GMenuChangedParam {
                        selected: self.selected.clone(),
                        selected_id: id,
                        e,
                    }),
                );
            }
        }
        if self.footer.is_visible() {
            self.footer.handle_event(cx, event, scope);
        }
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GMenu {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        if !self.is_visible() {
            return;
        }
        if self.header.is_visible() {
            self.header.after_apply(cx, apply, index, nodes);
        }
        if self.body.is_visible() {
            self.body.after_apply(cx, apply, index, nodes);
            let _ = self.find_selected();
        }
        if self.footer.is_visible() {
            self.footer.after_apply(cx, apply, index, nodes);
        }
        self.render(cx);
        self.redraw(cx);
    }
}

impl GMenu {
    event_option! {
        changed: GMenuEvent::Changed => GMenuChangedParam
    }
    pub fn fresh_selected(&mut self, cx: &mut Cx) {
        // let all children unselected
        for (_index, ((_, child), item_mode)) in self
            .body
            .children
            .iter()
            .zip(self.item_modes.iter())
            .enumerate()
        {
            match item_mode {
                MenuItemMode::SubMenu(_) => {
                    child.as_gsub_menu().borrow_mut().map(|mut sub_menu| {
                        sub_menu.clear_selected(cx);
                    });
                }
                MenuItemMode::MenuItem(_) => {
                    child.as_gmenu_item().borrow_mut().map(|mut item| {
                        item.clear_selected(cx);
                    });
                }
            }
        }
        // then if selected is not None, set the selected item
        if let Some(selected) = self.selected.as_ref() {
            MenuItemMode::find_node(&mut self.body.children, selected, &mut |item| {
                item.as_gmenu_item().borrow_mut().map(|mut item| {
                    item.selected = true;
                    item.render(cx);
                });
            });
        }
    }
    pub fn find_selected(&mut self) {
        for (_, child) in self.body.children.iter() {
            if let Some(child) = child.as_gmenu_item().borrow() {
                self.item_modes.push(MenuItemMode::MenuItem(child.selected));
            } else if let Some(child) = child.as_gsub_menu().borrow() {
                self.item_modes
                    .push(MenuItemMode::SubMenu(child.item_modes.clone()));
            } else {
                panic!("GMenu only allows GMenuItem or GSubMenu as child!");
            }
        }
        self.selected = MenuItemMode::selected(&self.item_modes);
    }

    pub fn redraw(&mut self, cx: &mut Cx) {
        if !self.is_visible() {
            return;
        }

        self.draw_menu.redraw(cx);
        if self.header.is_visible() {
            self.header.redraw(cx);
        }
        if self.body.is_visible() {
            self.body.redraw(cx);
        }
        if self.footer.is_visible() {
            self.footer.redraw(cx);
        }
    }
    pub fn render(&mut self, cx: &mut Cx) {
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.get(self.theme, 500);
        let shadow_color = self.shadow_color.get(self.theme, 700);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.get(self.theme, 400);
        // ------------------ focus color ---------------------------------------------
        let focus_color = self.focus_color.get(self.theme, 600);
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 600);
        // ------------------ is background_visible --------------------------------------------
        let background_visible = self.background_visible.to_f32();
        self.draw_menu.apply_over(
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
                blur_radius: (self.blur_radius),
            },
        );
    }
}


impl GMenuRef {
    ref_event_option! {
        changed => GMenuChangedParam
    }
}

impl GMenuSet {
    set_event! {
        changed => GMenuChangedParam
    }
}