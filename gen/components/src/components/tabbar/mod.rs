mod event;
pub mod item;
mod register;

pub use event::*;
use item::GTabbarItemWidgetRefExt;
pub use register::register;

use makepad_widgets::*;

use crate::{default_handle_animation, event_option, ref_event_option, set_event};

use super::view::GView;

live_design! {
    GTabbarBase = {{GTabbar}}{}
}

#[derive(Live, Widget)]
pub struct GTabbar {
    #[deref]
    pub deref_widget: GView,
    #[live(-1)]
    pub selected: i32,
}

impl Widget for GTabbar {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        default_handle_animation!(self, cx, event);
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        let _ = self.item_action(cx, scope, &actions).map(|_| {
            return;
        });
    }
}

impl LiveHook for GTabbar {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
        if self.selected < 0 {
            let _ = self.find_selected(cx);
        } else {
            self.set_selected(cx, self.selected as usize);
        }
    }
}

impl GTabbar {
    event_option! {
        changed: GTabbarEvent::Changed => GTabbarEventParam
    }
    fn item_action(&mut self, cx: &mut Cx, scope: &mut Scope, actions: &Actions) -> Option<()> {
        let mut flag = false;
        let mut selected = 0;
        let mut e = None;
        // try only do less to control event loop
        for (index, (_id, child)) in self.children.iter().enumerate() {
            let _ = child.as_gtabbar_item().borrow().map(|item| {
                if let Some(param) = item.clicked(&actions) {
                    if param.value {
                        if (index as i32).ne(&self.selected) {
                            selected = index;
                            flag = true;
                        } else {
                            flag = false;
                        }
                        e.replace(param.e);
                    }
                }
            });
            // if flag is true break to stop
            if flag {
                break;
            }
        }
        if flag {
            self.set_selected(cx, selected);
            self.redraw(cx);
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                GTabbarEvent::Changed(GTabbarEventParam {
                    selected,
                    e: e.unwrap(),
                }),
            );
            Some(())
        } else {
            None
        }
    }
    pub fn set_selected(&mut self, cx: &mut Cx, selected: usize) -> () {
        self.selected = selected as i32;

        // loop all gtabbar_item child and let selected == false except self.selected is true
        self.children
            .iter_mut()
            .enumerate()
            .for_each(|(index, (_id, child))| {
                if let Some(mut child) = child.as_gtabbar_item().borrow_mut() {
                    child.toggle(cx, index == selected);
                } else {
                    panic!("GTabbar only allows Gtabbar_item as child!");
                }
            });
    }
    fn find_selected(&mut self, cx: &mut Cx) -> () {
        let mut flag = true;
        let mut selected = 0;
        let _ = self
            .children
            .iter()
            .map(|(_id, child)| {
                if let Some(child) = child.as_gtabbar_item().borrow() {
                    child.selected
                } else {
                    panic!("GTabbar only allows tabbar_item as child!");
                }
            })
            .enumerate()
            .for_each(|(index, is_selected)| {
                if is_selected && flag {
                    selected = index;
                    flag = false;
                } else if is_selected && !flag {
                    panic!(
                        "In GTabbar only allows one tabbar_item be selected! The Second is: {}",
                        index
                    );
                }
            });

        if !flag {
            self.selected = selected as i32;
        } else {
            // here means no tabbar_item is selected
            self.set_selected(cx, 0);
        }
    }
    pub fn redraw(&mut self, cx: &mut Cx) -> () {
        self.deref_widget.redraw(cx);
    }
}

impl GTabbarRef {
    ref_event_option! {
        changed => GTabbarEventParam
    }
}

impl GTabbarSet {
    set_event! {
        changed => GTabbarEventParam
    }
}
