use makepad_widgets::*;

use crate::{
    shader::{draw_card::DrawGCard, draw_text::DrawGText}, themes::Themes, utils::{get_font_family, BoolToF32, ThemeColor, ToBool}
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
    #[live]
    pub draw_item: DrawGCard,
    #[live]
    pub draw_text: DrawGText,
    #[layout]
    pub layout: Layout,
    #[walk]
    pub walk: Walk,
    #[live]
    pub hover: f32,
    #[live]
    pub selected: f32,
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
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
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
        self.draw_text.redraw(cx);
        self.draw_item.redraw(cx);
    }
}

impl GSelectItem {
    pub fn area(&self) -> Area {
        self.draw_item.area()
    }
    pub fn draw_item(&mut self, cx: &mut Cx2d, text: &str, value: &str) {
        let _ = self.draw_item.begin(cx, self.walk, self.layout);
        let font = get_font_family(&self.font_family, cx);
        self.draw_text.text_style.font = font;
        let _ = self
            .draw_text
            .draw_walk(cx, Walk::fit(), Align::default(), text);
        self.value = value.to_string();
        self.text = text.to_string();
        let _ = self.draw_item.end(cx);
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
            Hit::FingerDown(_) => {
                self.animator_play(cx, id!(hover.on));
                self.animator_play(cx, id!(select.on));
            }
            Hit::FingerUp(se) => {
                if !se.is_sweep {
                    dispatch_action(
                        cx,
                        GSelectItemEvent::Clicked(GSelectItemClickedParam {
                            selected: self.selected.to_bool(),
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
