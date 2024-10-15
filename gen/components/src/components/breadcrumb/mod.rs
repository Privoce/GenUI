pub mod event;
pub mod item;
mod register;
use event::{GBreadCrumbClickedParam, GBreadCrumbEvent, GBreadCrumbHoverParam};
pub use register::register;

use item::{GBreadCrumbItemRef, GBreadCrumbItemWidgetRefExt};
use makepad_widgets::*;

use crate::{
    event_bool, event_option, ref_event_bool, ref_event_option, shader::draw_view::DrawGView, themes::Themes, utils::{set_cursor, ThemeColor}, widget_area, widget_origin_fn
};

use super::icon::GIcon;

live_design! {
    GLOBAL_DURATION = 0.25,
    GBreadCrumbBase = {{GBreadCrumb}}{
        icon_walk: {
            height: 20.0,
            width: 20.0,
            margin: 0,
        },
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        icon: {
                            draw_icon: {hover: 0.0},
                            icon_base: {hover: 0.0},
                            icon_arrow: {hover: 0.0},
                            icon_code: {hover: 0.0},
                            icon_emoji: {hover: 0.0},
                            icon_fs: {hover: 0.0},
                            icon_ui: {hover: 0.0},
                            icon_person: {hover: 0.0},
                            icon_relation: {hover: 0.0},
                            icon_state: {hover: 0.0},
                            icon_time: {hover: 0.0},
                            icon_tool: {hover: 0.0},
                        },
                        crumb_item: {
                            draw_text: {focus: [{time: 0.0, value: 0.0}], hover: 0.0,}
                        }
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                        focus: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        icon: {
                            draw_icon: {hover: 1.0},
                            icon_base: {hover: 1.0},
                            icon_arrow: {hover: 1.0},
                            icon_code: {hover: 1.0},
                            icon_emoji: {hover: 1.0},
                            icon_fs: {hover: 1.0},
                            icon_ui: {hover: 1.0},
                            icon_person: {hover: 1.0},
                            icon_relation: {hover: 1.0},
                            icon_state: {hover: 1.0},
                            icon_time: {hover: 1.0},
                            icon_tool: {hover: 1.0},
                        },
                        crumb_item: {
                            draw_text: {focus: [{time: 0.0, value: 1.0}], hover: 1.0,}
                        }
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
    // text -------------------
    #[live]
    pub color: Option<Vec4>,
    #[live(9.0)]
    pub font_size: f64,
    #[live]
    pub text_walk: Walk,
    #[live]
    pub labels: Vec<String>,
    // icon -------------------
    #[live(1.0)]
    pub icon_brightness: f32,
    #[live(0.6)]
    pub icon_curve: f32,
    #[live(0.5)]
    pub icon_linearize: f32,
    #[live(1.0)]
    pub icon_scale: f64,
    #[live]
    pub stroke_color: Option<Vec4>,
    #[live]
    pub stroke_hover_color: Option<Vec4>,
    #[live(1.0)]
    pub stroke_width: f32,
    #[live(1.0)]
    pub icon_draw_depth: f32,
    #[live]
    pub icon_walk: Walk,
    // deref -------------------
    #[redraw]
    #[live]
    pub draw_bread_crumb: DrawGView,
    #[live]
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
}

impl Widget for GBreadCrumb {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        self.draw_bread_crumb.begin(cx, walk, self.layout);
        let _ = self.icon.draw_walk(cx, scope, self.icon_walk);
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
            target.as_origin_mut().unwrap().walk.margin.top = self.font_size * 0.5;
            target.draw_all(cx, &mut Scope::empty());
        }

        self.draw_bread_crumb.end(cx);
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();

        if self.animation_key {
            if self.animator_handle_event(cx, event).must_redraw() {
                self.draw_bread_crumb.redraw(cx);
            }
        }

        // if click the home icon, call ToHome Event
        match event.hits(cx, self.icon_area()) {
            Hit::FingerDown(_) => {
                if self.grab_key_focus {
                    cx.set_key_focus(self.icon_area());
                }
            }
            Hit::FingerHoverIn(_) => {
                self.animator_play(cx, id!(hover.on));
                let _ = set_cursor(cx, self.icon.cursor.as_ref());
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerUp(f_up) => {
                if f_up.is_over {
                    cx.widget_action(uid, &scope.path, GBreadCrumbEvent::ToHome);

                    if f_up.device.has_hovers() {
                        self.animator_play(cx, id!(hover.on));
                    } else {
                        self.animator_play(cx, id!(hover.off));
                    }
                } else {
                    self.animator_play(cx, id!(hover.off));
                }
            }
            _ => {}
        }

        for (index, (_id, c_ref)) in self.crumb_items.clone().iter_mut().enumerate() {
            match event.hits(cx, c_ref.area()) {
                Hit::FingerDown(_) => {
                    if self.grab_key_focus {
                        cx.set_key_focus(c_ref.area());
                    }
                }
                Hit::FingerHoverIn(f_in) => {
                    // self.animator_play(cx, id!(hover.on));
                    let _ = set_cursor(cx, c_ref.as_origin().unwrap().cursor.as_ref());
                    // c_ref.as_origin_mut().unwrap().draw_text.apply_over(cx, live!{
                    //     hover: 1.0
                    // });
                    c_ref.animate_hover_on(cx);
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
                    c_ref.animate_hover_off(cx);
                    // self.animator_play(cx, id!(hover.off));
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
                            // self.animator_play(cx, id!(hover.on));
                            c_ref.animate_hover_on(cx);
                        } else {
                            // self.animator_play(cx, id!(hover.off));
                            c_ref.animate_hover_off(cx);
                        }
                    } else {
                        // self.animator_play(cx, id!(hover.off));
                        c_ref.animate_hover_off(cx);
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
        // ------------------ font ------------------------------------------------------
        let stroke_color = self.stroke_color.get(self.theme, 200);
        let stroke_hover_color = self.stroke_hover_color.get(self.theme, 100);
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.get(self.theme, 200);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.get(self.theme, 400);
        // ------------------ focus color ---------------------------------------------
        let focus_color = self.focus_color.get(self.theme, 600);
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 800);
        self.draw_bread_crumb.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                border_color: (border_color),
                border_width: (self.border_width),
                border_radius: (self.border_radius),
                focus_color: (focus_color),
                hover_color: (hover_color),
                background_visible: 0.0
            },
        );
        self.icon.apply_over(cx, live!{
            color: (stroke_color),
            stroke_width: (self.stroke_width),
            stroke_hover_color: (stroke_hover_color),
        });

        self.draw_bread_crumb.redraw(cx);
        self.icon.redraw(cx);
    }
}

impl GBreadCrumb {
    widget_area! {
        area, draw_bread_crumb,
        icon_area, icon
    }
    event_option! {
        clicked: GBreadCrumbEvent::Clicked => GBreadCrumbClickedParam,
        hover: GBreadCrumbEvent::Hover => GBreadCrumbHoverParam
    }
    event_bool!{
        to_home: GBreadCrumbEvent::ToHome
    }
    // pub fn area(&self) -> Area {
    //     self.draw_bread_crumb.area
    // }
    // pub fn icon_area(&self) -> Area {
    //     self.icon.area()
    // }
    // pub fn clicked(&self, actions: &Actions) -> Option<GBreadCrumbEventParam> {
    //     if let GBreadCrumbEvent::Clicked(e) = actions.find_widget_action(self.widget_uid()).cast() {
    //         Some(e)
    //     } else {
    //         None
    //     }
    // }
    // pub fn hover(&self, actions: &Actions) -> Option<GBreadCrumbEventParam> {
    //     if let GBreadCrumbEvent::Hover(e) = actions.find_widget_action(self.widget_uid()).cast() {
    //         Some(e)
    //     } else {
    //         None
    //     }
    // }
    // pub fn to_home(&self, actions: &Actions) -> bool {
    //     if let GBreadCrumbEvent::ToHome = actions.find_widget_action(self.widget_uid()).cast() {
    //         true
    //     } else {
    //         false
    //     }
    // }
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
    // pub fn clicked(&self, actions: &Actions) -> Option<GBreadCrumbEventParam> {
    //     if let Some(c_ref) = self.borrow() {
    //         return c_ref.clicked(actions);
    //     }
    //     None
    // }
    // pub fn hover(&self, actions: &Actions) -> Option<GBreadCrumbEventParam> {
    //     if let Some(c_ref) = self.borrow() {
    //         return c_ref.hover(actions);
    //     }
    //     None
    // }
    // pub fn to_home(&self, actions: &Actions) -> bool {
    //     if let Some(c_ref) = self.borrow() {
    //         return c_ref.to_home(actions);
    //     }
    //     false
    // }
}
