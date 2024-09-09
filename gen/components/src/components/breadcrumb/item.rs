use makepad_widgets::*;
use shader::draw_text::TextWrap;

use crate::{
    shader::{
        draw_card::DrawCard,
        draw_split::{DrawGSplit, GSplitType},
    },
    themes::Themes,
    utils::{get_font_family, ThemeColor},
};

live_design! {
    import makepad_draw::shader::std::*;
    ALIGN_CENTER_WALK = {x: 0.5, y: 0.5};
    GBreadCrumbItemBase = {{GBreadCrumbItem}}{
        height: Fit,
        width: Fit,
        flow: Right,
        padding: 0,
        text_walk: {
            height: Fit,
            width: Fit,
            margin: 0,
        },
    }
}

#[derive(Live, Widget)]
pub struct GBreadCrumbItem {
    #[live(Themes::Dark)]
    pub theme: Themes,
    // text -------------------
    #[live]
    pub color: Option<Vec4>,
    #[live(9.0)]
    pub font_size: f64,
    #[live(1.0)]
    pub brightness: f32,
    #[live(0.5)]
    pub curve: f32,
    // #[live(1.5)]
    // pub line_spacing: f64,
    #[live(0.0)]
    pub top_drop: f64,
    #[live(0.0)]
    pub height_factor: f64,
    #[live(TextWrap::Line)]
    pub wrap: TextWrap,
    #[live]
    pub font_family: LiveDependency,
    #[live]
    pub text: ArcStringMut,
    #[live]
    pub text_walk: Walk,
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
    #[live(GSplitType::Spliter)]
    pub split_type: GSplitType,
    // deref -------------------
    #[live]
    pub draw_text: DrawText,
    #[live]
    pub draw_split: DrawGSplit,
    #[redraw]
    #[live]
    pub draw_item: DrawCard,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
}

impl Widget for GBreadCrumbItem {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, mut walk: Walk) -> DrawStep {
        walk.height = Size::Fixed(self.font_size * 2.8);
        self.draw_item.begin(cx, walk, self.layout);

        self.icon_walk.abs_pos = walk.abs_pos;
        self.icon_walk.height = Size::Fill;
        self.icon_walk.width = Size::Fixed(self.font_size * 1.5);
        self.icon_walk.margin = Margin {
            left: 0.0,
            top: self.font_size * 0.7,
            right: 0.0,
            bottom: 0.0,
        };

        self.draw_split.draw_walk(cx, self.icon_walk);
        let font = get_font_family(&self.font_family, cx);

        self.draw_text.text_style.font = font;
        self.draw_text
            .draw_walk(cx, self.text_walk, Align::default(), self.text.as_ref());

        self.draw_item.end(cx);
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
}

impl LiveHook for GBreadCrumbItem {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        // ------------------ font ------------------------------------------------------
        let font_color = self.color.get(self.theme, 600);
        // ------------------icon color -----------------------------------------------
        let icon_color = self.icon_color.get(self.theme, 500);

        self.draw_split.apply_over(
            cx,
            live! {
                brightness: (self.icon_brightness),
                color: (icon_color),
                curve: (self.icon_curve),
                draw_depth: (self.icon_draw_depth),
                linearize: (self.icon_linearize),
            },
        );
        self.draw_split.apply_split_type(self.split_type.clone());

        self.draw_text.apply_over(
            cx,
            live! {
                color: (font_color),
                text_style: {
                    brightness: (self.brightness),
                    curve: (self.curve),
                    line_spacing: (self.layout.line_spacing),
                    top_drop: (self.top_drop),
                    font_size: (self.font_size),
                    height_factor: (self.height_factor),
                }
            },
        );
        self.draw_text.wrap = self.wrap.clone();
        self.draw_text.redraw(cx);
        self.draw_split.redraw(cx);
    }
}

impl GBreadCrumbItemRef {
    pub fn as_origin(&self) -> Option<std::cell::Ref<GBreadCrumbItem>> {
        self.borrow()
    }
    pub fn as_origin_mut(&mut self) -> Option<std::cell::RefMut<GBreadCrumbItem>> {
        self.borrow_mut()
    }
}
