use makepad_widgets::*;

use crate::{
    components::{view::GView, image::GImage, label::GLabel},
    shader::{
        draw_view::DrawGView,
        icon_lib::{base::DrawGIconBase, types::base::Base},
    },
    themes::{hex_to_vec4, Themes},
    utils::{set_cursor, BoolToF32, RectExpand, ThemeColor, ToDVec},
    widget_area,
};

use super::{GSelectItemClickedParam, GSelectItemEvent};

live_design! {
    GLOBAL_DURATION = 0.25;
    GSelectItemBase = {{GSelectItem}}{
        height: 36.0,
        width: 180.0,
        border_width: 0.0,
        border_radius: 0.0,
        spread_radius: 1.0,
        shadow_offset: vec2(0.0, 0.0),
        blur_radius: 5.0,
        clip_x: false,
        clip_y: false,
        background_visible: true,
        padding: {
            left: 8.0,
            right: 8.0
        },
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_item: {hover: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_item: {hover: 1.0}
                    }
                }
            }
            focus = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_item: { pressed: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_item: { pressed: 1.0}
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct GSelectItem {
    #[live]
    pub theme: Themes,
    #[live]
    pub stroke_color: Option<Vec4>,
    #[live]
    pub stroke_hover_color: Option<Vec4>,
    #[live(1.0)]
    pub stroke_width: f32,
    #[live]
    pub background_color: Option<Vec4>,
    #[live(true)]
    pub background_visible: bool,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub pressed_color: Option<Vec4>,
    #[live]
    pub shadow_color: Option<Vec4>,
    #[live(4.8)]
    pub spread_radius: f32,
    #[live(4.8)]
    pub blur_radius: f32,
    #[live]
    pub shadow_offset: Vec2,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(0.0)]
    pub border_width: f32,
    #[live(0.0)]
    pub border_radius: f32,
    #[live]
    pub cursor: Option<MouseCursor>,
    #[live(Base::Correct)]
    pub icon_type: Base,
    // visible -------------------
    #[live(true)]
    pub visible: bool,
    #[live]
    pub left_visible: bool,
    #[live]
    pub right_visible: bool,
    #[redraw]
    #[live]
    pub draw_item: DrawGView,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[live]
    #[redraw]
    #[find]
    pub left: GView,
    #[live]
    #[redraw]
    #[find]
    pub center: GView,
    #[live]
    #[redraw]
    pub right: GView,
    #[live]
    #[redraw]
    pub draw_select: DrawGIconBase,
    #[live]
    pub selected: bool,
    #[live(true)]
    pub grab_key_focus: bool,
    // animator -----------------
    #[live(true)]
    pub animation_key: bool,
    #[animator]
    pub animator: Animator,
}

impl Widget for GSelectItem {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        self.draw_item.begin(cx, walk, self.layout);
        if self.left_visible {
            let left_walk = self.left.walk(cx);
            let _ = self.left.draw_walk(cx, scope, left_walk);
        }
        let center_walk = self.center.walk(cx);
        let _ = self.center.draw_walk(cx, scope, center_walk);
        if self.right_visible {
            let right_walk = self.right.walk(cx);
            let _ = self.right.draw_walk(cx, scope, right_walk);
        }
        let select_walk = Walk {
            height: Size::Fixed(16.0),
            width: Size::Fixed(16.0),
            abs_pos: Some(DVec2 { x: 16.0, y: 48.0 }),
            ..Default::default()
        };
        let select_rect = if self.selected {
            let select_rect = self.draw_select.draw_walk(cx, select_walk);
            Some(select_rect)
        } else {
            None
        };
        self.draw_item.end(cx);
        select_rect.map(|mut select_rect| {
            let rect = self.area().rect(cx);
            let x = -16.0 - self.layout.padding.right;
            select_rect.abs_end_center(&rect, Some(dvec2(x, 0.0)));
            self.draw_select.update_abs(cx, select_rect);
        });
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();

        if self.animation_key {
            let _ = self.animator_handle_event(cx, event);
        }

        match event.hits(cx, self.area()) {
            Hit::FingerHoverIn(_) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                self.animator_play(cx, id!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerDown(_) => {
                if self.grab_key_focus {
                    cx.set_key_focus(self.area());
                }
            }
            Hit::FingerUp(f_up) => {
                if f_up.is_over {
                    self.selected = !self.selected;
                    cx.widget_action(
                        uid,
                        &scope.path,
                        GSelectItemEvent::Clicked(GSelectItemClickedParam {
                            selected: self.selected,
                            e: f_up.clone(),
                        }),
                    );
                    self.redraw(cx);
                    if self.selected {
                        self.animator_play(cx, id!(focus.on));
                    } else {
                        self.animator_play(cx, id!(focus.off));
                    }
                } else {
                    self.animator_play(cx, id!(hover.off));
                }
            }
            _ => (),
        }
    }
}

impl LiveHook for GSelectItem {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.visible {
            return;
        }
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.use_or("#ffffff");
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.use_or("#E3E3E3");
        // ------------------ pressed color ---------------------------------------------
        let pressed_color = self.pressed_color.use_or("#E3E3E3");
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 500);
        let shadow_color = self.shadow_color.use_or("#E3E3E3");
        let background_visible = self.background_visible.to_f32();
        let stroke_color = self.stroke_color.get(self.theme, 600);
        let stroke_hover_color = self.stroke_hover_color.get(self.theme, 600);
        self.draw_item.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                background_visible: (background_visible),
                border_color: (border_color),
                border_width: (self.border_width),
                border_radius: (self.border_radius),
                pressed_color: (pressed_color),
                hover_color: (hover_color),
                shadow_color: (shadow_color),
                shadow_offset: (self.shadow_offset),
                spread_radius: (self.spread_radius),
                blur_radius: (self.blur_radius),
                pressed: (self.selected.to_f32())
            },
        );
        self.draw_select.apply_over(
            cx,
            live! {
                stroke_color: (stroke_color),
                stroke_width: (self.stroke_width),
                stroke_hover_color: (stroke_hover_color),
            },
        );
        self.draw_select.apply_type(self.icon_type);
        self.draw_select.redraw(cx);
        self.draw_item.redraw(cx);
    }
}

impl GSelectItem {
    widget_area! {
        area, draw_item,
        area_left, left,
        area_right, right,
        area_select, draw_select,
        area_center, center
    }
}
