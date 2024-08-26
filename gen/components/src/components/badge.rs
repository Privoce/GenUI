use crate::utils::get_font_family;
use crate::{
    shader::draw_card::DrawCard,
    themes::{get_color, Themes},
};
use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;
    GLOBAL_DURATION = 0.25

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
    pub text: RcStringMut,
    #[live(10.0)]
    pub font_size: f64,
    #[live]
    pub color: Option<Vec4>,
    #[live]
    pub font_family: LiveDependency,
    #[live]
    pub cursor: Option<MouseCursor>,
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

        let _ = self.draw_badge.begin(cx, walk, self.layout);

        let _ = self
            .draw_text
            .draw_walk(cx, self.text_walk, Align::default(), self.text.as_ref());

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
                    radius += radius / 10.0 + 0.25;
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
        self.draw_text.apply_over(
            cx,
            live! {
                color: (font_color),
                text_style: {
                    font_size: (self.font_size),
                },
            },
        );
        self.draw_badge.redraw(cx);
        self.draw_text.redraw(cx);
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
