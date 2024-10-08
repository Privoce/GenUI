pub mod menu_item;
pub mod sub_menu;
mod register;
pub mod event;

use makepad_widgets::*;
pub use register::register;
use crate::shader::draw_view::DrawGView;

use super::view::GView;

live_design! {
    GMenuBase = <GMenu>{

    }
}

#[derive(Live, Widget)]
pub struct GMenu {
    #[live(true)]
    pub visible: bool,
    #[redraw]
    #[live]
    pub draw_menu: DrawGView,
    #[live(-1)]
    pub selected: i32,
    #[find]
    #[live]
    #[redraw]
    pub header: GView,
    #[find]
    #[live]
    #[redraw]
    pub footer: GView,
    #[find]
    #[live]
    #[redraw]
    pub body: GView,
    #[layout]
    pub layout: Layout,
    #[walk]
    pub walk: Walk,
}

impl Widget for GMenu {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.is_visible() {
           return DrawStep::done();
        }

        self.draw_menu.begin(cx, walk, self.layout);
        if self.header.is_visible() {
            let header_walk = self.header.walk(cx);
            let _ = self.header.draw_walk(cx, scope, header_walk);
        }
        if self.body.is_visible() {
            let body_walk = self.body.walk(cx);
            let _ = self.body.draw_walk(cx, scope, body_walk);
        }
        if self.footer.is_visible() {
            let footer_walk = self.footer.walk(cx);
            let _ = self.footer.draw_walk(cx, scope, footer_walk);
        }
        self.draw_menu.end(cx);
        DrawStep::done()
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GMenu{}