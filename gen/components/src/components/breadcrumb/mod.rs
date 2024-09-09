pub mod item;

use item::{GBreadCrumbItemRef, GBreadCrumbItemWidgetRefExt};
use makepad_widgets::*;

use crate::{shader::draw_card::DrawCard, themes::Themes, utils::ThemeColor};

use super::image::GImage;

live_design! {
    GBreadCrumbBase = {{GBreadCrumb}}{
        icon_walk: {
            height: 20.0,
            width: 20.0,
            margin: 0,
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
    pub draw_icon: GImage,
    #[live]
    pub crumb_item: Option<LivePtr>,
    #[rust]
    pub crumb_items: ComponentMap<LiveId, GBreadCrumbItemRef>,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
}

impl Widget for GBreadCrumb {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_bread_crumb.begin(cx, walk, self.layout);
        let _ = self.draw_icon.draw_walk(cx, scope, self.icon_walk);
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
}

impl LiveHook for GBreadCrumb {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
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
        // self.draw_text.apply_over(
        //     cx,
        //     live! {
        //         color: (font_color),
        //         text_style: {
        //             font_size: (self.font_size),
        //             brightness: (self.brightness),
        //             curve: (self.curve),
        //             line_spacing: (self.layout.line_spacing),
        //             top_drop: (self.top_drop),
        //             height_factor: (self.height_factor),
        //         },
        //     },
        // );
        self.draw_bread_crumb.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                border_color: (border_color),
                border_width: (self.border_width),
                border_radius: (self.border_radius),
                pressed_color: (pressed_color),
                hover_color: (hover_color),
                transparent: 1.0
            },
        );

        self.draw_bread_crumb.redraw(cx);
        self.draw_icon.redraw(cx);
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
