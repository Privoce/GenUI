use makepad_widgets::*;

use crate::{
    components::view::GView, event_option, ref_event_option, set_event,
    shader::draw_view::DrawGView, themes::Themes, widget_area,
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
        // animation_open: true,
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
    pub pressed_color: Option<Vec4>,
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
    #[live(-1)]
    pub selected: i32,
    /// id of sub menu, it alaways be unique and used as `usize`
    #[live]
    pub id: i32,
    #[layout]
    pub layout: Layout,
    #[walk]
    pub walk: Walk,
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

            let mut selected = None;
            let mut e = None;
            // try only do less to control event loop
            for (index, (_, child)) in self.items.children.iter().enumerate() {
                let _ = child.as_gmenu_item().borrow().map(|item| {
                    if let Some(param) = item.clicked(&actions) {
                        if param.selected {
                            if (index as i32).ne(&self.selected) {
                                selected.replace(index);
                            }
                            e.replace(param.e);
                        }
                    }
                });
                // if flag is true break to stop
                if selected.is_some() {
                    break;
                }
            }
            if let Some(selected) = selected {
                self.set_selected(cx, selected);
                cx.widget_action(
                    self.widget_uid(),
                    &scope.path,
                    GSubMenuEvent::Changed(GSubMenuChangedParam {
                        selected,
                        e: e.unwrap(),
                        sub_menu_id: self.id(),
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
            if self.selected < 0 {
                let _ = self.find_selected();
            } else {
                self.set_selected(cx, self.selected as usize);
            }
        }
    }
}

impl GSubMenu {
    pub fn id(&self) -> usize {
        self.id as usize
    }
    pub fn set_selected(&mut self, cx: &mut Cx, selected: usize) -> () {
        self.selected = selected as i32;

        // loop all GMenuItem child and let selected == false except self.selected is true
        self.items
            .children
            .iter_mut()
            .enumerate()
            .for_each(|(index, (_id, child))| {
                if let Some(mut child) = child.as_gmenu_item().borrow_mut() {
                    child.toggle(cx, index == selected);
                } else {
                    panic!("GSubMenu only allows GMenuItem as child!");
                }
            });
    }
    fn find_selected(&mut self) -> () {
        let mut flag = true;
        let mut selected = 0;
        let _ = self
            .items
            .children
            .iter()
            .map(|(_id, child)| {
                if let Some(child) = child.as_gmenu_item().borrow() {
                    child.selected
                } else {
                    panic!("GSubMenu only allows GMenuItem as child!");
                }
            })
            .enumerate()
            .for_each(|(index, is_selected)| {
                if is_selected && flag {
                    selected = index;
                    flag = false;
                } else if is_selected && !flag {
                    panic!(
                        "In GSubMenu only allows one item be selected! The Second is: {}",
                        index
                    );
                }
            });

        if !flag {
            self.selected = selected as i32;
        }
        // else{
        //     // here means no item is selected
        //     self.set_selected(cx, 0);
        // }
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
