mod event;
// mod copy;
pub mod item;
mod register;

pub use event::*;
pub use register::register;

use item::GBreadCrumbItemWidgetRefExt;
use makepad_widgets::*;

use crate::{
    active_event, animatie_fn, check_event_scope, event_option, ref_area, ref_event_option,
    ref_play_animation, ref_redraw_mut, ref_render, utils::LiveIdGenerate, widget_area,
};

use super::{
    svg::{GSvgRef, GSvgWidgetExt},
    view::GView,
};

live_design! {
    GLOBAL_DURATION = 0.25;
    GBreadCrumbBase = {{GBreadCrumb}}{
        animation_key: true,
    }
}

#[derive(Live, Widget)]
pub struct GBreadCrumb {
    #[deref]
    pub deref_widget: GView,
    #[live]
    pub path: Vec<String>,
    #[live]
    pub item: Option<LivePtr>,
    /// use omit to hide the items if len > 3
    #[live]
    pub omit: bool,
}

impl Widget for GBreadCrumb {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.render_item(cx);
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

        // icon callback ---------------------------------------------------------------------
        // if home icon clicked then callback Home
        let home_icon = self.gsvg(id!(icon));
        if let Some(e) = home_icon.clicked(&actions) {
            self.active_home(cx, Some(e.e));
            return;
        }
        if let Some(e) = home_icon.hover_in(&actions) {
            self.active_hover_in(cx, Some(e.e), GBreadCrumbItemKind::Icon);
            self.play_animation(cx, id!(hover.on));
            return;
        }
        if let Some(e) = home_icon.hover_out(&actions) {
            self.active_hover_out(cx, Some(e.e), GBreadCrumbItemKind::Icon);
            self.play_animation(cx, id!(hover.off));
            return;
        }
        if let Some(e) = home_icon.focus(&actions) {
            self.active_focus(cx, Some(e.e), GBreadCrumbItemKind::Icon);
            self.play_animation(cx, id!(hover.focus));
            return;
        }
        if let Some(e) = home_icon.focus_lost(&actions) {
            self.active_focus_lost(cx, Some(e.e));
            self.play_animation(cx, id!(hover.off));
            return;
        }
        // icon callback ---------------------------------------------------------------------

        for (index, (_, child)) in self.children.clone().iter_mut().enumerate() {
            // cause first is icon, so index need to sub 1
            if let Some(item) = child.as_gbread_crumb_item().borrow_mut() {
                let index = index - 1;
                // item.handle_event(cx, event, scope);
                if let Some(e) = item.clicked(&actions) {
                    self.active_changed(cx, e.e, index, e.text);
                    return;
                }
                if let Some(e) = item.hover_in(&actions) {
                    self.play_animation(cx, id!(hover.on));
                    self.active_hover_in(
                        cx,
                        e.e,
                        GBreadCrumbItemKind::Item {
                            text: item.text(),
                            index,
                        },
                    );
                    return;
                }
                if let Some(e) = item.hover_out(&actions) {
                    self.play_animation(cx, id!(hover.off));
                    self.active_hover_out(
                        cx,
                        e.e,
                        GBreadCrumbItemKind::Item {
                            text: item.text(),
                            index,
                        },
                    );
                    return;
                }
                if let Some(e) = item.focus(&actions) {
                    self.play_animation(cx, id!(hover.focus));
                    self.active_focus(
                        cx,
                        e.e,
                        GBreadCrumbItemKind::Item {
                            text: item.text(),
                            index,
                        },
                    );
                    return;
                }
                if let Some(e) = item.focus_lost(&actions) {
                    self.play_animation(cx, id!(hover.off));
                    self.active_focus_lost(cx, e.e);
                    return;
                }
            }
        }
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GBreadCrumb {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl GBreadCrumb {
    widget_area! {
        area, deref_widget
    }
    event_option! {
        hover_in: GBreadCrumbEvent::HoverIn => GBreadCrumbHoverParam,
        hover_out: GBreadCrumbEvent::HoverOut => GBreadCrumbHoverParam,
        changed: GBreadCrumbEvent::Changed => GBreadCrumbChangedParam,
        focus: GBreadCrumbEvent::Focus => GBreadCrumbFocusParam,
        focus_lost: GBreadCrumbEvent::FocusLost => GBreadCrumbFocusLostParam,
        home: GBreadCrumbEvent::Home => GBreadCrumbHomeParam
    }
    active_event! {
        active_home: GBreadCrumbEvent::Home |e: Option<FingerUpEvent>| => GBreadCrumbHomeParam {e},
        active_focus_lost: GBreadCrumbEvent::FocusLost |e: Option<FingerUpEvent>| => GBreadCrumbFocusLostParam {e}
    }
    check_event_scope!();
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.deref_widget.redraw(cx);
    }
    pub fn render_item(&mut self, cx: &mut Cx) {
        // if omit is true, just need to display 3 realy items (first, second, omit sign, last)
        let path = if self.omit && self.path.len() > 3 {
            vec![
                self.path[0].clone(),
                self.path[1].clone(),
                "…".to_string(),
                self.path[self.path.len() - 1].clone(),
            ]
        } else {
            self.path.clone()
        };

        for (index, path) in path.iter().enumerate() {
            let index = index + 1;
            if self.children.get(index).is_none() {
                let child = WidgetRef::new_from_ptr(cx, self.item);
                let id = index.to_live_id();
                self.children.push((id, child));
            }

            self.children.get_mut(index).map(|(_, child)| {
                child.as_gbread_crumb_item().borrow_mut().map(|mut item| {
                    item.set_text(&path);
                });
            });
        }
    }
    pub fn active_changed(
        &mut self,
        cx: &mut Cx,
        e: Option<FingerUpEvent>,
        index: usize,
        text: String,
    ) {
        self.check_event_scope().map(|path| {
            cx.widget_action(
                self.widget_uid(),
                path,
                GBreadCrumbEvent::Changed(GBreadCrumbChangedParam { index, text, e }),
            );
        });
    }
    pub fn active_hover_in(
        &mut self,
        cx: &mut Cx,
        e: Option<FingerHoverEvent>,
        kind: GBreadCrumbItemKind,
    ) {
        self.check_event_scope().map(|path| {
            cx.widget_action(
                self.widget_uid(),
                path,
                GBreadCrumbEvent::HoverIn(GBreadCrumbHoverParam { kind, e }),
            );
        });
    }
    pub fn active_hover_out(
        &mut self,
        cx: &mut Cx,
        e: Option<FingerHoverEvent>,
        kind: GBreadCrumbItemKind,
    ) {
        self.check_event_scope().map(|path| {
            cx.widget_action(
                self.widget_uid(),
                path,
                GBreadCrumbEvent::HoverOut(GBreadCrumbHoverParam { kind, e }),
            );
        });
    }
    pub fn active_focus(
        &mut self,
        cx: &mut Cx,
        e: Option<FingerDownEvent>,
        kind: GBreadCrumbItemKind,
    ) {
        self.check_event_scope().map(|path| {
            cx.widget_action(
                self.widget_uid(),
                path,
                GBreadCrumbEvent::Focus(GBreadCrumbFocusParam { kind, e }),
            );
        });
    }
    pub fn render(&mut self, cx: &mut Cx) {
        self.deref_widget.render(cx);
    }
    pub fn path(&self) -> &Vec<String> {
        &self.path
    }
    pub fn icon(&self) -> GSvgRef {
        self.gsvg(id!(icon))
    }
}

impl GBreadCrumbRef {
    ref_area!();
    ref_redraw_mut!();
    ref_render!();
    ref_event_option! {
        hover_in => GBreadCrumbHoverParam,
        hover_out => GBreadCrumbHoverParam,
        changed => GBreadCrumbChangedParam,
        focus => GBreadCrumbFocusParam,
        focus_lost => GBreadCrumbFocusLostParam,
        home => GBreadCrumbHomeParam
    }
    animatie_fn! {
        clear_animation,
        animate_hover_on,
        animate_hover_off,
        animate_focus_on,
        animate_focus_off
    }
    ref_play_animation! {
        play_hover_on: id!(hover.on),
        play_hover_off: id!(hover.off),
        play_focus_on: id!(hover.focus),
        play_focus_off: id!(hover.off)
    }
    pub fn path(&self) -> Vec<String> {
        if let Some(c_ref) = self.borrow() {
            c_ref.path().clone()
        } else {
            vec![]
        }
    }
}
