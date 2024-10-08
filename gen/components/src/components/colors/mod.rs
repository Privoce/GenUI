use makepad_widgets::*;
mod register;

pub use register::register;

use crate::{shader::draw_view::DrawGView, themes::Themes};

use super::{
    label::GLabelWidgetExt,
    view::{GView, GViewWidgetRefExt},
};

live_design! {
    GColorBase = {{GColor}}{}
}

#[derive(Live, Widget)]
pub struct GColor {
    #[redraw]
    #[live]
    pub draw_color: DrawGView,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[live(true)]
    pub visible: bool,
    #[find]
    #[redraw]
    #[live]
    pub header: GView,
    #[find]
    #[redraw]
    #[live]
    pub colors: GView,
    #[live]
    pub theme: Themes,
    #[live]
    pub item: Option<LivePtr>,
}

impl Widget for GColor {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.is_visible() {
            return DrawStep::done();
        }

        let _ = self.draw_color.begin(cx, walk, self.layout);
        if self.header.is_visible() {
            let header_walk = self.header.walk(cx);
            let _ = self.header.draw_walk(cx, scope, header_walk);
        }

        if self.colors.is_visible() {
            let colors_walk = self.colors.walk(cx);
            let _ = self.colors.draw_walk(cx, scope, colors_walk);
        }

        let _ = self.draw_color.end(cx);
        DrawStep::done()
    }

    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GColor {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.is_visible() {
            return;
        }
        self.render(cx);
        if self.header.is_visible() {
            self.header.theme = self.theme;
            self.header
                .glabel(id!(theme_name))
                .set_text_and_redraw(cx, &self.theme.to_string());
            self.header
                .glabel(id!(theme_main))
                .set_text_and_redraw(cx, &self.theme.hex(500));
        }

        if self.colors.is_visible() {
            
            self.colors.children.clear();
            for (index, color) in self.theme.to_vec().into_iter().enumerate() {
                self.colors
                    .children
                    .push((LiveId(index as u64), WidgetRef::new_from_ptr(cx, self.item)));
                self.colors.children.last_mut().map(|(_, view)| {
                    view.as_gview().borrow_mut().map(|mut view| {
                        view.theme = self.theme;
                        view.background_color.replace(color);
                        view.render(cx);
                    });
                });
            }
        }
    }
}

impl GColor {
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.draw_color.redraw(cx);
        if self.header.is_visible() {
            self.header.redraw(cx);
        }
        if self.colors.is_visible() {
            self.colors.redraw(cx);
        }
    }
    pub fn render(&mut self, cx: &mut Cx) {
        // ----------------- background color -------------------------------------------
        let bg_color = self.theme.get(500);
        let shadow_color = self.theme.get(500);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.theme.get(500);
        // ------------------ pressed color ---------------------------------------------
        let pressed_color = self.theme.get(500);
        // ------------------ border color ----------------------------------------------
        let border_color = self.theme.get(500);
        self.draw_color.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                background_visible: 1.0,
                border_color: (border_color),
                // border_width: (self.border_width),
                // border_radius: (self.border_radius),
                pressed_color: (pressed_color),
                hover_color: (hover_color),
                shadow_color: (shadow_color),
                spread_radius: 0.0,
            },
        );
    }
}
