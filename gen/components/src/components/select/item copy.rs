use makepad_widgets::*;

use crate::{
    shader::{
        draw_view::DrawGView,
        draw_text::DrawGText,
        icon_lib::{base::DrawGIconBase, types::base::Base},
    },
    themes::Themes,
    utils::{get_font_family, BoolToF32, RectExpand, ThemeColor},
};

use super::{GSelectItemClickedParam, GSelectItemEvent};

live_design! {
    GSelectItemBase = {{GSelectItem}} {
        width: Fill,
        height: 36.0,
        align: {x: 0.0, y: 0.5},
        animator: {
            hover = {
                default: off
                off = {
                    from: {all: Snap}
                    apply: {
                        draw_item: {hover: 0.0}
                        draw_text: {hover: 0.0}
                    }
                }
                on = {
                    cursor: Hand
                    from: {all: Snap}
                    apply: {
                        draw_item: {hover: 1.0}
                        draw_text: {hover: 1.0}
                    }
                }
            }

            select = {
                default: off
                off = {
                    from: {all: Snap}
                    apply: {
                        draw_item: {pressed: 0.0,}
                        draw_text: {pressed: 0.0,}
                    }
                }
                on = {
                    from: {all: Snap}
                    apply: {
                        draw_item: {pressed: 1.0,}
                        draw_text: {pressed: 1.0,}
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveRegister)]
pub struct GSelectItem {
    #[live]
    pub theme: Themes,
    #[live(9.0)]
    pub font_size: f32,
    #[live]
    pub stroke_color: Option<Vec4>,
    #[live]
    pub stroke_hover_color: Option<Vec4>,
    #[live]
    pub color: Option<Vec4>,
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
    #[live(0.0)]
    pub spread_radius: f32,
    #[live(2.8)]
    pub blur_radius: f32,
    #[live]
    pub shadow_offset: Vec2,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(0.0)]
    pub border_width: f32,
    #[live(0.0)]
    pub border_radius: f32,
    #[live(1.0)]
    pub stroke_width: f32,
    #[live]
    pub draw_item: DrawGView,
    #[live]
    pub draw_text: DrawGText,
    #[live]
    pub draw_selector: DrawGIconBase,
    #[live(Base::Correct)]
    pub icon_type: Base,
    #[layout]
    pub layout: Layout,
    #[walk]
    pub walk: Walk,
    #[live]
    pub hover: f32,
    #[live]
    pub selected: bool,
    #[animator]
    pub animator: Animator,
    #[live]
    pub text: String,
    #[live]
    pub value: String,
    #[live]
    pub font_family: LiveDependency,
}

impl LiveHook for GSelectItem {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.use_or("#FFFFFF00");
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.use_or("#F9FAFB");
        // ------------------ pressed color ---------------------------------------------
        let pressed_color = self.pressed_color.use_or("#F9FAFB");
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 600);
        let shadow_color = self.shadow_color.get(self.theme, 700);
        let background_visible = self.background_visible.to_f32();
        let color = self.color.use_or("#101828");
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
                blur_radius: (self.blur_radius)
            },
        );
        self.draw_text.apply_over(
            cx,
            live! {
                color: (color),
                hover_color: (color),
                pressed_color: (color),
                text_style: {
                    font_size: (self.font_size),
                }
            },
        );
        self.draw_selector.apply_over(
            cx,
            live! {
                stroke_color: (stroke_color),
                stroke_width: (self.stroke_width),
                stroke_hover_color: (stroke_hover_color),
            },
        );
        self.draw_selector.apply_type(self.icon_type);
        self.draw_selector.redraw(cx);
        self.draw_text.redraw(cx);
        self.draw_item.redraw(cx);
    }
}

impl GSelectItem {
    pub fn area(&self) -> Area {
        self.draw_item.area()
    }
    pub fn draw_item(&mut self, cx: &mut Cx2d, text: &str, value: &str, theme: Themes) {
        self.theme = theme;
        let _ = self.draw_item.begin(cx, self.walk, self.layout);
        let font = get_font_family(&self.font_family, cx);
        self.draw_text.text_style.font = font;
        let _ = self
            .draw_text
            .draw_walk(cx, Walk::fit(), Align::default(), text);
        let icon_walk = Walk {
            height: Size::Fixed(16.0),
            width: Size::Fixed(16.0),
            abs_pos: Some(DVec2 { x: 16.0, y: 48.0 }),
            ..Default::default()
        };
        let select_rect = if self.selected {
            let select_rect = self.draw_selector.draw_walk(cx, icon_walk);
            Some(select_rect)
        } else {
            None
        };
        self.value = value.to_string();
        self.text = text.to_string();
        let _ = self.draw_item.end(cx);
        select_rect.map(|mut select_rect| {
            let rect = self.area().rect(cx);
            let x = -16.0 - self.layout.padding.right;
            select_rect.abs_end_center(&rect, Some(dvec2(x, 0.0)));
            self.draw_selector.update_abs(cx, select_rect);
        });
    }
    pub fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        sweep_area: Area,

        dispatch_action: &mut dyn FnMut(&mut Cx, GSelectItemEvent),
    ) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.draw_item.area().redraw(cx);
        }
        match event.hits_with_options(
            cx,
            self.area(),
            HitOptions::new().with_sweep_area(sweep_area),
        ) {
            Hit::FingerHoverIn(_) => {
                self.animator_play(cx, id!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerDown(_) => {}
            Hit::FingerUp(se) => {
                self.selected = !self.selected;
                
                if !se.is_sweep {
                    dispatch_action(
                        cx,
                        GSelectItemEvent::Clicked(GSelectItemClickedParam {
                            selected: self.selected,
                            e: se.clone(),
                            text: self.text.to_string(),
                            value: self.value.to_string(),
                        }),
                    );
                } else {
                    self.animator_play(cx, id!(hover.off));
                    self.animator_play(cx, id!(select.off));
                }
            }
            _ => {}
        }
    }
}
