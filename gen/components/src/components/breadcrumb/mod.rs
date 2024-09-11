pub mod event;
pub mod item;
mod register;
use event::{GBreadCrumbEvent, GBreadCrumbEventParam};
pub use register::register;

use item::{GBreadCrumbItemRef, GBreadCrumbItemWidgetRefExt};
use makepad_widgets::*;

use crate::{
    shader::draw_card::DrawCard,
    themes::Themes,
    utils::{set_cursor, ThemeColor},
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
    pub pressed_color: Option<Vec4>,
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
    pub icon_color: Option<Vec4>,
    #[live(1.0)]
    pub icon_draw_depth: f32,
    #[live]
    pub icon_walk: Walk,
    // deref -------------------
    #[redraw]
    #[live]
    pub draw_bread_crumb: DrawCard,
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
    pub animation_open: bool,
    #[animator]
    pub animator: Animator,
    #[live(true)]
    pub visible: bool,
    #[live(Some(MouseCursor::Hand))]
    pub cursor: Option<MouseCursor>,
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

        if self.animation_open {
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
                        GBreadCrumbEvent::Hover(GBreadCrumbEventParam {
                            index,
                            item: c_ref.text(),
                            key_modifiers: f_in.modifiers,
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
                            GBreadCrumbEvent::Clicked(GBreadCrumbEventParam {
                                index,
                                item: c_ref.text(),
                                key_modifiers: f_up.modifiers,
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
        let _icon_color = self.color.get(self.theme, 100);
        let _icon_hover_color = self.color.get(self.theme, 50);
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.get(self.theme, 200);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.get(self.theme, 400);
        // ------------------ pressed color ---------------------------------------------
        let pressed_color = self.pressed_color.get(self.theme, 600);
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 800);
        self.draw_bread_crumb.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                border_color: (border_color),
                border_width: (self.border_width),
                border_radius: (self.border_radius),
                pressed_color: (pressed_color),
                hover_color: (hover_color),
                background_visible: 1.0
            },
        );

        self.draw_bread_crumb.redraw(cx);
        self.icon.redraw(cx);
    }
}

impl GBreadCrumb {
    pub fn area(&self) -> Area {
        self.draw_bread_crumb.area
    }
    pub fn icon_area(&self) -> Area {
        self.icon.area()
    }
}

impl GBreadCrumbRef {
    pub fn as_origin(&self) -> Option<std::cell::Ref<GBreadCrumb>> {
        self.borrow()
    }
    pub fn as_origin_mut(&mut self) -> Option<std::cell::RefMut<GBreadCrumb>> {
        self.borrow_mut()
    }
}
