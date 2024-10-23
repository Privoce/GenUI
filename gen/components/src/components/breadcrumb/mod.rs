mod event;
pub mod item;
mod register;
pub use event::*;
pub use register::register;

use item::{GBreadCrumbItemRef, GBreadCrumbItemWidgetRefExt};
use makepad_widgets::*;

use crate::{
    default_handle_animation, event_bool, event_option, play_animation, ref_event_bool,
    ref_event_option, set_scope_path,
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
            text_hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {crumb_item: {draw_text: {focus: 0.0, hover: 0.0,}},},
                }
                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                        focus: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {crumb_item: {draw_text: {focus: 0.0, hover: 1.0,}},},
                }
                focus = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {crumb_item: {draw_text: {focus: 1.0, hover: 0.0,}},},
                }
            }
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
    pub color: Option<Vec4>,
    #[live(9.0)]
    pub font_size: f64,
    #[live]
    pub text_walk: Walk,
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
    pub crumb_items: ComponentMap<LiveId, GBreadCrumbItemRef>,
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
            let target = self
                .crumb_items
                .get_or_insert(cx, LiveId(index as u64), |cx| {
                    WidgetRef::new_from_ptr(cx, self.crumb_item).as_gbread_crumb_item()
                });
            target.set_text(&data);
            // target.as_origin_mut().unwrap().walk.margin.top = self.font_size * 0.5;
            target.draw_all(cx, &mut Scope::empty());
        }

        self.draw_bread_crumb.end(cx);
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        default_handle_animation!(self, cx, event);

        // if click the home icon, call ToHome Event
        match event.hits(cx, self.icon_area()) {
            Hit::FingerDown(_) => {
                self.play_animation(cx, id!(icon_hover.focus));
                // self.icon.play_animation(cx, id!(hover.focus));
                // self.icon.animate_focus_on(cx);
            }
            Hit::FingerHoverIn(_) => {
                self.play_animation(cx, id!(icon_hover.on));
                let _ = set_cursor(cx, self.icon.cursor.as_ref());
            }
            Hit::FingerHoverOut(_) => {
                self.play_animation(cx, id!(icon_hover.off));
            }
            Hit::FingerUp(f_up) => {
                if f_up.is_over {
                    if f_up.device.has_hovers() {
                        self.play_animation(cx, id!(icon_hover.on));
                    } else {
                        self.play_animation(cx, id!(icon_hover.off));
                    }
                    cx.widget_action(uid, &scope.path, GBreadCrumbEvent::ToHome);
                } else {
                    self.play_animation(cx, id!(icon_hover.off));
                }
            }
            _ => {}
        }

        for (index, (_id, c_ref)) in self.crumb_items.clone().iter_mut().enumerate() {
            match event.hits(cx, c_ref.area()) {
                Hit::FingerDown(_) => {
                    self.play_animation(cx, id!(text_hover.focus));
                }
                Hit::FingerHoverIn(f_in) => {
                    // self.play_animation(cx, id!(hover.on));
                    let _ = set_cursor(cx, c_ref.as_origin().unwrap().cursor.as_ref());
                    // c_ref.as_origin_mut().unwrap().draw_text.apply_over(cx, live!{
                    //     hover: 1.0
                    // });
                    // c_ref.animate_hover_on(cx);
                    self.play_animation(cx, id!(text_hover.on));
                    cx.widget_action(
                        uid,
                        &scope.path,
                        GBreadCrumbEvent::Hover(GBreadCrumbHoverParam {
                            index,
                            item: c_ref.text(),
                            e: f_in.clone(),
                        }),
                    );
                }
                Hit::FingerHoverOut(_) => {
                    // c_ref.animate_hover_off(cx);
                    self.play_animation(cx, id!(text_hover.off));
                }
                Hit::FingerUp(f_up) => {
                    if f_up.is_over {
                        cx.widget_action(
                            uid,
                            &scope.path,
                            GBreadCrumbEvent::Clicked(GBreadCrumbClickedParam {
                                index,
                                item: c_ref.text(),
                                e: f_up.clone(),
                            }),
                        );

                        if f_up.device.has_hovers() {
                            self.play_animation(cx, id!(text_hover.on));
                            // c_ref.animate_hover_on(cx);
                        } else {
                            self.play_animation(cx, id!(text_hover.off));
                            // c_ref.animate_hover_off(cx);
                        }
                    } else {
                        self.play_animation(cx, id!(text_hover.off));
                        // c_ref.animate_hover_off(cx);
                    }
                }
                _ => {}
            }
        }
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
    event_option! {
        clicked: GBreadCrumbEvent::Clicked => GBreadCrumbClickedParam,
        hover: GBreadCrumbEvent::Hover => GBreadCrumbHoverParam
    }
    event_bool! {
        to_home: GBreadCrumbEvent::ToHome
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
        clicked => GBreadCrumbClickedParam,
        hover => GBreadCrumbHoverParam
    }
    ref_event_bool! {
        to_home
    }
}
