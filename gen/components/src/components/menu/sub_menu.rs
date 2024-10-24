use makepad_widgets::*;

use crate::{
    components::view::GView,
    event_option, ref_event_option, set_event,
    shader::{draw_view::DrawGView, manual::MenuItemMode},
    themes::Themes,
    widget_area,
};

use super::{
    event::{GSubMenuChangedParam, GSubMenuEvent},
    menu_item::GMenuItemWidgetRefExt,
};

live_design! {
    GSubMenuBase = {{GSubMenu}} {
        border_radius: 0.0,
        border_width: 0.0,
        spread_radius: 0.0,
        background_visible: false,
        height: Fit,
        width: Fill,
        // animation_key: true,
        flow: Down,
        spacing: 0.0,
    }
}

#[derive(Live, Widget)]
pub struct GSubMenu {
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
    #[live(0.0)]
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
    pub draw_sub_menu: DrawGView,
    #[find]
    #[live]
    #[redraw]
    pub title: GView,
    #[find]
    #[live]
    #[redraw]
    pub items: GView,
    #[rust]
    pub item_modes: Vec<MenuItemMode>,
    #[rust]
    pub selected: Option<Vec<usize>>,
    #[layout]
    pub layout: Layout,
    #[walk]
    pub walk: Walk,
    #[live(true)]
    pub event_key: bool,
}

impl Widget for GSubMenu {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.is_visible() {
            return DrawStep::done();
        }
        self.draw_sub_menu.begin(cx, walk, self.layout);
        if self.title.is_visible() {
            let title_walk = self.title.walk(cx);
            let _ = self.title.draw_walk(cx, scope, title_walk);
        }
        if self.items.is_visible() {
            let items_walk = self.items.walk(cx);
            let _ = self.items.draw_walk(cx, scope, items_walk);
        }

        self.draw_sub_menu.end(cx);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible {
            return;
        }

        if self.title.is_visible() {
            let _ = self.title.handle_event(cx, event, scope);
        }
        if self.items.is_visible() {
            let actions = cx.capture_actions(|cx| self.items.handle_event(cx, event, scope));

            let mut fresh = None;
            for (index, ((id, child), item_mode)) in self
                .items
                .children
                .iter()
                .zip(self.item_modes.iter())
                .enumerate()
            {
                match item_mode {
                    MenuItemMode::SubMenu(_) => {
                        child.as_gsub_menu().borrow_mut().map(|mut sub_menu| {
                            sub_menu.handle_event(cx, event, scope);
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
            }
            if let Some((id, e)) = fresh {
                self.fresh_selected(cx);

                cx.widget_action(
                    self.widget_uid(),
                    &scope.path,
                    GSubMenuEvent::Changed(GSubMenuChangedParam {
                        selected: self.selected.clone(),
                        selected_id: id,
                        e,
                    }),
                );
            }
        }
    }
}

impl LiveHook for GSubMenu {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        if self.title.is_visible() {
            self.title.after_apply(cx, apply, index, nodes);
        }
        if self.items.is_visible() {
            self.items.after_apply(cx, apply, index, nodes);
            let _ = self.find_selected();
        }
    }
}

impl GSubMenu {
    pub fn clear_selected(&mut self, cx: &mut Cx) {
        self.selected = None;
        for (_index, ((_, child), item_mode)) in self
            .items
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
    }
    pub fn fresh_selected(&mut self, cx: &mut Cx) {
        // let all children unselected
        for (_index, ((_, child), item_mode)) in self
            .items
            .children
            .iter()
            .zip(self.item_modes.iter())
            .enumerate()
        {
            match item_mode {
                MenuItemMode::SubMenu(_) => {
                    child.as_gsub_menu().borrow_mut().map(|mut sub_menu| {
                        sub_menu.fresh_selected(cx);
                    });
                }
                MenuItemMode::MenuItem(_) => {
                    child.as_gmenu_item().borrow_mut().map(|mut item| {
                        item.selected = false;
                        item.render(cx);
                    });
                }
            }
        }
        // then if selected is not None, set the selected item
        if let Some(selected) = self.selected.as_ref() {
            MenuItemMode::find_node(&mut self.items.children, selected, &mut |item| {
                item.as_gmenu_item().borrow_mut().map(|mut item| {
                    item.selected = true;
                    item.render(cx);
                });
            });
        }
    }
    /// try to find the selected item in the menu item
    fn find_selected(&mut self) -> () {
        for (_id, child) in self.items.children.iter() {
            if let Some(child) = child.as_gmenu_item().borrow() {
                self.item_modes.push(MenuItemMode::MenuItem(child.selected));
            } else if let Some(child) = child.as_gsub_menu().borrow() {
                self.item_modes
                    .push(MenuItemMode::SubMenu(child.item_modes.clone()));
            } else {
                panic!("GSubMenu only allows GMenuItem or GSubMenu as child!");
            }
        }
        self.selected = MenuItemMode::selected(&self.item_modes);
    }
    pub fn redraw(&mut self, cx: &mut Cx) {
        if !self.is_visible() {
            return;
        }
        self.draw_sub_menu.redraw(cx);
        if self.title.is_visible() {
            self.title.redraw(cx);
        }
        if self.items.is_visible() {
            self.items.redraw(cx);
        }
    }
    widget_area! {
        area, draw_sub_menu
    }
    event_option! {
        changed: GSubMenuEvent::Changed => GSubMenuChangedParam
    }
}

impl GSubMenuRef {
    ref_event_option! {
        changed => GSubMenuChangedParam
    }
}

impl GSubMenuSet {
    set_event! {
        changed => GSubMenuChangedParam
    }
}
