pub mod body;
pub mod button;
pub mod header;

use header::GTabHeader;
use makepad_widgets::*;

use crate::{
    shader::draw_card::DrawCard,
    themes::{get_color, Themes},
};

use super::card::Card;

live_design! {
    GTabBase = {{GTab}}{}
}

#[derive(Live, Widget)]
pub struct GTab {
    #[live]
    pub theme: Themes,
    #[live]
    pub items: Vec<String>,
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
    #[redraw]
    #[live]
    pub draw_tab: DrawCard,
    #[live]
    pub header: GTabHeader,
    #[live]
    pub body: Card,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
}

impl Widget for GTab {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_tab.begin(cx, walk, self.layout);
        self.header.set_items(self.items.clone());
        let _ = self.header.draw_walk(cx, scope, self.header.walk);
        
        let _ = self.body.draw_walk(cx, scope, self.body.walk);

        self.draw_tab.end(cx);

        DrawStep::done()
    }
}

impl LiveHook for GTab {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        // ----------------- background color -------------------------------------------
        let bg_color = get_color(self.theme, self.background_color, 500);
        // ------------------ hover color -----------------------------------------------
        let hover_color = get_color(self.theme, self.hover_color, 400);
        // ------------------ pressed color ---------------------------------------------
        let pressed_color = get_color(self.theme, self.pressed_color, 600);
        // ------------------ border color ----------------------------------------------
        let border_color = get_color(self.theme, self.border_color, 800);
        self.draw_tab.apply_over(
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
        self.draw_tab.redraw(cx);
    }
}
