pub mod event;
pub mod menu_item;
mod register;
pub mod sub_menu;

use crate::{
    shader::draw_view::DrawGView, themes::Themes, utils::{BoolToF32, ThemeColor}
};
use makepad_widgets::*;
pub use register::register;

use super::view::GView;

live_design! {
    GMenuBase = {{GMenu}}{

    }
}

#[derive(Live, Widget)]
pub struct GMenu {
    #[live(Themes::Dark)]
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
    #[live(0.0)]
    pub border_radius: f32,
    #[live(true)]
    pub background_visible: bool,
    #[live]
    pub shadow_color: Option<Vec4>,
    #[live(4.8)]
    pub spread_radius: f32,
    #[live(4.8)]
    pub blur_radius: f32,
    #[live]
    pub shadow_offset: Vec2,
    #[live]
    pub cursor: Option<MouseCursor>,
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
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.is_visible() {
            return;
        }
        if self.header.is_visible() {
            self.header.handle_event(cx, event, scope);
        }
        if self.body.is_visible() {
            self.body.handle_event(cx, event, scope);
        }
        if self.footer.is_visible() {
            self.footer.handle_event(cx, event, scope);
        }
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GMenu {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.is_visible() {
            return;
        }
        self.render(cx);
        self.redraw(cx);
    }
}

impl GMenu {
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.draw_menu.redraw(cx);
        if self.header.is_visible() {
            self.header.redraw(cx);
        }
        if self.body.is_visible() {
            self.body.redraw(cx);
        }
        if self.footer.is_visible() {
            self.footer.redraw(cx);
        }
    }
    pub fn render(&mut self, cx: &mut Cx) {
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.get(self.theme, 500);
        let shadow_color = self.shadow_color.get(self.theme, 700);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.get(self.theme, 400);
        // ------------------ pressed color ---------------------------------------------
        let pressed_color = self.pressed_color.get(self.theme, 600);
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 600);
        // ------------------ is background_visible --------------------------------------------
        let background_visible = self.background_visible.to_f32();
        self.draw_menu.apply_over(
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
                blur_radius: (self.blur_radius),
            },
        );
    }
}
