use crate::shader::draw_icon::DrawGIcon;
use crate::shader::draw_icon_pixel::DrawGIconPixel;
use crate::utils::{get_font_family, DefaultTextStyle};
use crate::{
    shader::draw_card::DrawCard,
    themes::{get_color, Themes},
};
use makepad_widgets::*;

live_design! {
    GBadgeBase = {{GBadge}}{
        height: Fit,
        width: Fit,
        text_walk: {
            height: Fit,
            width: Fit,
        }
        cursor: Hand,
        
        draw_text: {
            instance hover: 0.0,
            instance pressed: 0.0,

            fn get_color(self) -> vec4 {
                let hover_color = self.color - vec4(0.0, 0.0, 0.0, 0.1);
                let pressed_color = self.color - vec4(0.0, 0.0, 0.0, 0.2);

                return mix(
                    mix(
                        self.color,
                        hover_color,
                        self.hover
                    ),
                    pressed_color,
                    self.pressed
                )
            }
        },
        icon_walk: {
            margin: 0,
        },
        icon_layout: {
            padding: 0,
        }
    }
}

#[derive(Live, Widget)]
pub struct GBadge {
    #[live]
    pub theme: Themes,
    #[live]
    pub background_color: Option<Vec4>,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub icon_hover_color: Option<Vec4>,
    #[live]
    pub pressed_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(0.0)]
    pub border_width: f32,
    #[live(2.0)]
    pub border_radius: f32,
    #[live(false)]
    pub round: bool,
    // text -----------------
    #[live]
    pub text: ArcStringMut,
    #[live(10.0)]
    pub font_size: f64,
    #[live]
    pub color: Option<Vec4>,
    #[live]
    pub font_family: LiveDependency,
    #[live]
    pub cursor: Option<MouseCursor>,
    #[live]
    pub closeable: bool,
    #[live]
    pub src: LiveDependency,
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
    // visible -------------------
    #[live(true)]
    pub visible: bool,
    // define area -----------------
    #[live]
    draw_text: DrawText,
    #[live]
    text_walk: Walk,
    #[live(true)]
    grab_key_focus: bool,
    #[live]
    draw_icon: DrawGIcon,
    #[live]
    draw_close: DrawGIconPixel,
    #[live]
    icon_walk: Walk,
    #[live]
    icon_layout: Layout,
    // deref -----------------
    #[redraw]
    #[live]
    draw_badge: DrawCard,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
}

impl Widget for GBadge {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let font = get_font_family(&self.font_family, cx);
        self.draw_text.text_style.font = font;

        self.icon_walk.height = Size::Fixed(self.font_size);
        self.icon_walk.width = Size::Fixed(self.font_size);
        self.text_walk.margin.top = self.font_size / 4.0;
        let _ = self.draw_badge.begin(cx, walk, self.layout);
        let _ = self.draw_icon.draw_walk(cx, self.icon_walk);

        let _ = self
            .draw_text
            .draw_walk(cx, self.text_walk, Align::default(), self.text.as_ref());
        if self.closeable {
            let _ = self.draw_close.draw_walk(cx, self.icon_walk);
        }
        self.draw_badge.end(cx);
        DrawStep::done()
    }

    fn text(&self) -> String {
        self.text.as_ref().to_string()
    }
    fn set_text(&mut self, v: &str) {
        self.text.as_mut_empty().push_str(v);
    }
    fn set_text_and_redraw(&mut self, cx: &mut Cx, v: &str) {
        self.text.as_mut_empty().push_str(v);
        self.redraw(cx)
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GBadge {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        // ----------------- background color -------------------------------------------
        let bg_color = get_color(self.theme, self.background_color, 500);
        // ------------------ hover color -----------------------------------------------
        let hover_color = get_color(self.theme, self.hover_color, 400);
        // ------------------ pressed color ---------------------------------------------
        let pressed_color = get_color(self.theme, self.pressed_color, 600);
        // ------------------ border color ----------------------------------------------
        let border_color = get_color(self.theme, self.border_color, 800);
        // ------------------ font ------------------------------------------------------
        let font_color = get_color(self.theme, self.color, 100);
        // ------------------icon color -----------------------------------------------
        let icon_color = get_color(self.theme, self.icon_color, 100);
        let icon_hover_color = get_color(self.theme, self.icon_hover_color, 50);
        // ------------------ round -----------------------------------------------------

        if self.round {
            self.border_radius = match self.walk.height {
                Size::Fixed(h) => (h * 0.25) as f32,
                Size::Fit => {
                    let mut radius = ((self.draw_text.text_style.font_size
                        + self.walk.margin.top
                        + self.walk.margin.bottom
                        + self.layout.padding.top
                        + self.layout.padding.bottom)
                        * 0.25) as f32;
                    radius += self.border_width;
                    radius += self.font_size as f32 / 8.0 + radius / 10.0;
                    radius
                }
                _ => panic!("round only support fixed and fit"),
            };
        }

        self.draw_badge.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                border_color: (border_color),
                border_width: (self.border_width),
                border_radius: (self.border_radius),
                pressed_color: (pressed_color),
                hover_color: (hover_color),
            },
        );
        self.draw_icon.apply_over(
            cx,
            live! {
                hover_color: (icon_hover_color),
                color: (icon_color),
                brightness: (self.icon_brightness),
                curve: (self.icon_curve),
                linearize: (self.icon_linearize),
                scale: (self.icon_scale),
                draw_depth: (self.icon_draw_depth),
            },
        );

        self.draw_icon.set_src(self.src.clone());
        let default_text_style = DefaultTextStyle::default();
        self.draw_text.apply_over(
            cx,
            live! {
                color: (font_color),
                text_style: {
                    font_size: (self.font_size),
                    brightness: (default_text_style.brightness),
                    curve: (default_text_style.curve),
                    line_spacing: (default_text_style.line_spacing),
                    top_drop: (default_text_style.top_drop),
                    height_factor: (default_text_style.height_factor),
                },
            },
        );
        
        if self.closeable {
            // self.draw_close.apply_over(
            //     cx,
            //     live! {
            //         hover_color: (icon_hover_color),
            //         color: (icon_color),
            //         brightness: (self.icon_brightness),
            //         curve: (self.icon_curve),
            //         linearize: (self.icon_linearize),
            //         scale: (self.icon_scale),
            //         draw_depth: (self.icon_draw_depth),
            //     },
            // );
            self.draw_close.apply_over(
                cx,
                live! {
                    brightness: (self.icon_brightness),
                    color: (icon_color),
                    curve: (self.icon_curve),
                    draw_depth: (self.icon_draw_depth),
                    linearize: (self.icon_linearize),
                },
            );
            self.draw_close.redraw(cx);
        }
        
        self.draw_badge.redraw(cx);
        self.draw_text.redraw(cx);
        self.draw_icon.redraw(cx);
    }
}

impl GBadgeRef {
    pub fn area(&self) -> Area {
        if let Some(btn_ref) = self.borrow() {
            return btn_ref.draw_badge.area();
        }
        Area::Empty
    }
}
