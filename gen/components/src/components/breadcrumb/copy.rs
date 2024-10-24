mod event;
pub mod item;
mod register;
pub use event::*;
pub use register::register;

use item::{GBreadCrumbItem, GBreadCrumbItemRef, GBreadCrumbItemWidgetRefExt};
use makepad_widgets::*;

use crate::{
    active_event, check_event_scope, default_handle_animation, event_bool, event_option,
    play_animation, ref_event_bool, ref_event_option, set_scope_path,
    shader::draw_view::DrawGView,
    themes::Themes,
    utils::{set_cursor, BoolToF32, ThemeColor},
    widget_area, widget_origin_fn,
};

use super::icon::GIcon;

live_design! {
    GLOBAL_DURATION = 0.25;
    GBreadCrumbBase = {{GBreadCrumb}}{
        animator: {
            icon_hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        icon: {
                            draw_icon: {hover: 0.0, focus: 0.0},
                            icon_base: {hover: 0.0, focus: 0.0},
                            icon_arrow: {hover: 0.0, focus: 0.0},
                            icon_code: {hover: 0.0, focus: 0.0},
                            icon_emoji: {hover: 0.0, focus: 0.0},
                            icon_fs: {hover: 0.0, focus: 0.0},
                            icon_ui: {hover: 0.0, focus: 0.0},
                            icon_person: {hover: 0.0, focus: 0.0},
                            icon_relation: {hover: 0.0, focus: 0.0},
                            icon_state: {hover: 0.0, focus: 0.0},
                            icon_time: {hover: 0.0, focus: 0.0},
                            icon_tool: {hover: 0.0, focus: 0.0},
                        },
                    },
                }
                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                        focus: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        icon: {
                            draw_icon: {hover: 1.0, focus: 0.0}
                            icon_base: {hover: 1.0, focus: 0.0},
                            icon_arrow: {hover: 1.0, focus: 0.0},
                            icon_code: {hover: 1.0, focus: 0.0},
                            icon_emoji: {hover: 1.0, focus: 0.0},
                            icon_fs: {hover: 1.0, focus: 0.0},
                            icon_ui: {hover: 1.0, focus: 0.0},
                            icon_person: {hover: 1.0, focus: 0.0},
                            icon_relation: {hover: 1.0, focus: 0.0},
                            icon_state: {hover: 1.0, focus: 0.0},
                            icon_time: {hover: 1.0, focus: 0.0},
                            icon_tool: {hover: 1.0, focus: 0.0},
                        },
                    }
                }

                focus = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        icon: {
                            draw_icon: {hover: 0.0, focus: 1.0}
                            icon_base: {hover: 0.0, focus: 1.0},
                            icon_arrow: {hover: 0.0, focus: 1.0},
                            icon_code: {hover: 0.0, focus: 1.0},
                            icon_emoji: {hover: 0.0, focus: 1.0},
                            icon_fs: {hover: 0.0, focus: 1.0},
                            icon_ui: {hover: 0.0, focus: 1.0},
                            icon_person: {hover: 0.0, focus: 1.0},
                            icon_relation: {hover: 0.0, focus: 1.0},
                            icon_state: {hover: 0.0, focus: 1.0},
                            icon_time: {hover: 0.0, focus: 1.0},
                            icon_tool: {hover: 0.0, focus: 1.0},
                        },
                    }
                }
            },
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_bread_crumb: {focus: 0.0, hover: 0.0},
                    },
                }
                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                        focus: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_bread_crumb: {focus: 0.0, hover: 1.0},
                    }
                }
                focus = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_bread_crumb: {focus: 1.0, hover: 0.0},
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct GBreadCrumb {
    #[live]
    pub theme: Themes,
    #[live]
    pub background_color: Option<Vec4>,
    #[live]
    pub background_visible: bool,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub focus_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(0.0)]
    pub border_width: f32,
    #[live(4.0)]
    pub border_radius: f32,
    #[live]
    pub shadow_color: Option<Vec4>,
    #[live(0.0)]
    pub spread_radius: f32,
    #[live(4.8)]
    pub blur_radius: f32,
    #[live]
    pub shadow_offset: Vec2,
    // text -------------------
    #[live]
    pub labels: Vec<String>,
    // deref -------------------
    #[redraw]
    #[live]
    pub draw_bread_crumb: DrawGView,
    #[live]
    #[find]
    #[redraw]
    pub icon: GIcon,
    #[live]
    pub crumb_item: Option<LivePtr>,
    #[rust]
    pub crumb_items: Vec<(LiveId, GBreadCrumbItem)>,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[live(true)]
    pub grab_key_focus: bool,
    // animator -----------------
    #[live(true)]
    pub animation_key: bool,
    #[animator]
    pub animator: Animator,
    #[live(true)]
    pub visible: bool,
    #[live(Some(MouseCursor::Hand))]
    pub cursor: Option<MouseCursor>,
    #[live(true)]
    pub event_key: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
}

impl Widget for GBreadCrumb {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        self.set_scope_path(&scope.path);
        self.draw_bread_crumb.begin(cx, walk, self.layout);
        if self.icon.is_visible() {
            let icon_walk = self.icon.walk(cx);
            let _ = self.icon.draw_walk(cx, scope, icon_walk);
        }
        let len = self.labels.len();
        let labels = if len <= 3 {
            self.labels.clone()
        } else {
            // if more then 3, just draw the first and latest 2, other do not render use more_crumb to replace
            vec![
                self.labels[0].to_string(),
                "...".to_string(),
                self.labels[len - 2].to_string(),
                self.labels[len - 1].to_string(),
            ]
        };
        for (index, data) in labels.iter().enumerate() {
            let lb = GBreadCrumbItem::new_from_ptr(cx, self.crumb_item);
            if self.crumb_items.get(index).is_none() {
                // not exist
                self.crumb_items.push((LiveId(index as u64), lb));
            }
            self.crumb_items.get_mut(index).map(|(_, item)| {
                item.set_text(&data);
                item.draw_all(cx, &mut Scope::empty());
            });
        }
        self.draw_bread_crumb.end(cx);
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible{
            return;
        }
        // default_handle_animation!(self, cx, event);
        if let Event::Actions(actions) = event{
            // for action in actions {
                
            // }
            if let Some(action) = actions.find_widget_action(self.widget_uid()) {
                    dbg!(action);
            }
        }
        

        // if click the home icon, call Home Event
        // match event.hits(cx, self.icon_area()) {
        //     Hit::FingerDown(e) => {
        //         self.play_animation(cx, id!(icon_hover.focus));
        //         // self.play_animation(cx, id!(hover.focus));
        //         // self.active_focus(cx, Some(e), GBreadCrumbItemKind::Icon);
        //     }
        //     Hit::FingerHoverIn(e) => {
        //         let _ = set_cursor(cx, self.icon.cursor.as_ref());
        //         self.play_animation(cx, id!(icon_hover.on));
        //         // self.play_animation(cx, id!(hover.on));
        //         // self.active_hover_in(cx, Some(e), GBreadCrumbItemKind::Icon);
        //     }
        //     Hit::FingerHoverOut(_) => {
        //         self.play_animation(cx, id!(icon_hover.off));
        //         // self.play_animation(cx, id!(hover.off));
        //         // self.active_hover_out(cx, None, GBreadCrumbItemKind::Icon);
        //     }
        //     Hit::FingerUp(e) => {
        //         if e.is_over {
        //             if e.device.has_hovers() {
        //                 self.play_animation(cx, id!(icon_hover.on));
        //                 // self.play_animation(cx, id!(hover.on));
        //             } else {
        //                 self.play_animation(cx, id!(icon_hover.off));
        //                 // self.play_animation(cx, id!(hover.off));
        //             }
        //             self.active_home(cx, Some(e));
        //         } else {
        //             self.play_animation(cx, id!(icon_hover.off));
        //             // self.play_animation(cx, id!(hover.off));
        //         }
        //     }
        //     _ => {}
        // }

        // for (index, (_id, c)) in self.crumb_items.iter_mut().enumerate() {
        //     match event.hits(cx, c.area()) {
        //         Hit::FingerDown(e) => {
        //             if c.grab_key_focus {
        //                 cx.set_key_focus(c.area());
        //             }
        //             c.animate_focus_on(cx);
        //             self.play_animation(cx, id!(hover.focus));
        //             self.active_focus(cx, Some(e), GBreadCrumbItemKind::Item { text: c.text(), index: index });
        //         }
        //         Hit::FingerHoverIn(f_in) => {
        //             c.animate_hover_on(cx);
        //             // cx.widget_action(
        //             //     uid,
        //             //     &scope.path,
        //             //     GBreadCrumbEvent::Hover(GBreadCrumbHoverParam {
        //             //         index,
        //             //         item: c_ref.text(),
        //             //         e: f_in.clone(),
        //             //     }),
        //             // );
        //         }
        //         Hit::FingerHoverOut(_) => {
        //             c.animate_hover_off(cx);
        //         }
        //         Hit::FingerUp(f_up) => {
        //             if f_up.is_over {
        //                 dbg!("click");
        //                 // cx.widget_action(
        //                 //     uid,
        //                 //     &scope.path,
        //                 //     GBreadCrumbEvent::Clicked(GBreadCrumbClickedParam {
        //                 //         index,
        //                 //         item: c.text(),
        //                 //         e: f_up.clone(),
        //                 //     }),
        //                 // );

        //                 if f_up.device.has_hovers() {
        //                     c.animate_hover_on(cx);
        //                 } else {
        //                     c.animate_hover_off(cx);
        //                 }
        //             } else {
        //                 c.animate_hover_off(cx);
        //             }
        //         }
        //         _ => {}
        //     }
        // }
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GBreadCrumb {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.visible {
            return;
        }
        self.render(cx);
    }
}

impl GBreadCrumb {
    set_scope_path!();
    play_animation!();
    widget_area! {
        area, draw_bread_crumb,
        icon_area, icon
    }
    pub fn area_text(&self, index: usize) -> Area {
        if index > self.crumb_items.len() {
            return Area::Empty;
        }
        self.crumb_items[index].1.area()
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
    pub fn clear_animation(&mut self, cx: &mut Cx) {
        self.icon.clear_animation(cx);
        self.draw_bread_crumb.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
    }
    pub fn render(&mut self, cx: &mut Cx) {
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.get(self.theme, 500);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.get(self.theme, 400);
        // ------------------ focus color ---------------------------------------------
        let focus_color = self.focus_color.get(self.theme, 600);
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 600);
        let shadow_color = self.shadow_color.get(self.theme, 700);
        let background_visible = self.background_visible.to_f32();
        // apply over props to draw_button ----------------------------------------------
        self.draw_bread_crumb.apply_over(
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
    }
}

impl GBreadCrumbRef {
    widget_origin_fn!(GBreadCrumb);
    ref_event_option! {
        hover_in => GBreadCrumbHoverParam,
        hover_out => GBreadCrumbHoverParam,
        changed => GBreadCrumbChangedParam,
        focus => GBreadCrumbFocusParam,
        focus_lost => GBreadCrumbFocusLostParam,
        home => GBreadCrumbHomeParam
    }
}
