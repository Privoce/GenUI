use makepad_widgets::*;

use crate::{
    shader::{draw_view::DrawGView, draw_text::DrawGText},
    themes::Themes,
    utils::{get_font_family, BoolToF32, ThemeColor},
};

use super::{GSelectItemClickedParam, GSelectItemEvent};

live_design! {
    import makepad_draw::shader::std::*;
    GSelectItemBase = {{GSelectItem}} {
        width: Fill,
        height: 36.0,
        align: {x: 0.0, y: 0.5},
        draw_item: {
            instance stroke_color: vec4,
            instance stroke_width: 1.4,
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size3);
                sdf.box(
                    self.sdf_rect_pos.x,
                    self.sdf_rect_pos.y,
                    self.sdf_rect_size.x,
                    self.sdf_rect_size.y,
                    max(1.0, self.border_radius)
                );

                if self.background_visible != 0.0 {
                    sdf.fill_keep(self.get_background_color());
                }
                if self.spread_radius != 0.0 {
                    if sdf.shape > -1.0{
                        let m = self.blur_radius;
                        let o = self.shadow_offset + self.rect_shift;
                        if self.border_radius != 0.0 {
                            let v = GaussShadow::rounded_box_shadow(vec2(m) + o, self.rect_size2 + o, self.pos * (self.rect_size3+vec2(m)), self.spread_radius * 0.5, self.border_radius*2.0);
                            let shadow_color = vec4(self.shadow_color.rgb, self.shadow_color.a * v);
                            sdf.clear(shadow_color);
                        }else{
                            let v = GaussShadow::box_shadow(vec2(m) + o, self.rect_size2 + o, self.pos * (self.rect_size3+vec2(m)), self.spread_radius * 0.5);
                            let shadow_color = vec4(self.shadow_color.rgb, self.shadow_color.a * v);
                            sdf.clear(shadow_color);
                        }
                    }
                }

                sdf.stroke(self.get_border_color(), self.border_width);
                if self.focus == 1.0{
                    let start_p = vec2(self.sdf_rect_size.x - 26.0, self.sdf_rect_size.y * 0.5 - 7.0);
                    let end_p = vec2(self.sdf_rect_size.x - 12.0, self.sdf_rect_size.y * 0.5 + 7.0);
                    let center_y = self.sdf_rect_size.y * 0.5;
                    sdf.move_to(start_p.x, center_y + 1.0);
                    sdf.line_to(self.sdf_rect_size.x - 22.0, end_p.y - 1.0);
                    sdf.line_to(end_p.x, start_p.y + 2.0);
                    sdf.stroke(self.stroke_color, self.stroke_width);
                }

                return sdf.result
            }
        }
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
                        draw_item: {focus: 0.0,}
                        draw_text: {focus: 0.0,}
                    }
                }
                on = {
                    from: {all: Snap}
                    apply: {
                        draw_item: {focus: 1.0,}
                        draw_text: {focus: 1.0,}
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
    pub focus_color: Option<Vec4>,
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
    #[live(1.4)]
    pub stroke_width: f32,
    #[live]
    pub draw_item: DrawGView,
    #[live]
    pub draw_text: DrawGText,
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
    #[live(true)]
    pub event_key: bool,
}

impl LiveHook for GSelectItem {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.use_or("#FFFFFF00");
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.use_or("#F9FAFB");
        // ------------------ focus color ---------------------------------------------
        let focus_color = self.focus_color.use_or("#F9FAFB");
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 600);
        let shadow_color = self.shadow_color.get(self.theme, 700);
        let background_visible = self.background_visible.to_f32();
        let color = self.color.use_or("#101828");
        let stroke_color = self.stroke_color.get(self.theme, 600);
        // let stroke_hover_color = self.stroke_hover_color.get(self.theme, 600);
        self.draw_item.apply_over(
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
                blur_radius: (self.blur_radius),
                focus: (self.selected.to_f32()),
                stroke_color: (stroke_color),
                stroke_width: (self.stroke_width),
            },
        );
        self.draw_text.apply_over(
            cx,
            live! {
                color: (color),
                stroke_hover_color: (color),
                stroke_focus_color: (color),
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
    pub fn draw_item(&mut self, cx: &mut Cx2d, text: &str, value: &str, theme: Themes) {
        self.theme = theme;
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
