pub mod body;
pub mod cell;
pub mod column;
pub mod header;
mod register;
pub mod row;
pub mod virt;

use body::GTableBody;
use header::GTableHeader;
use makepad_widgets::*;
pub use register::register;
use virt::GVTableBody;

use crate::{
    shader::{draw_view::DrawGView, manual::ComponentMode},
    themes::Themes,
    utils::{BoolToF32, ThemeColor},
    widget_area,
};

live_design! {
    GTableBase = {{GTable}}{
        flow: Down,
        padding: 0.0,
        margin: 0.0,
    }
}

#[derive(Live, Widget)]
pub struct GTable {
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
    #[live(true)]
    pub visible: bool,
    #[live(false)]
    pub background_visible: bool,
    #[live]
    pub shadow_color: Option<Vec4>,
    #[live(0.0)]
    pub spread_radius: f32,
    #[live(4.8)]
    pub blur_radius: f32,
    #[live]
    pub shadow_offset: Vec2,
    #[redraw]
    #[live]
    pub draw_table: DrawGView,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[live]
    #[redraw]
    #[find]
    pub header: GTableHeader,
    #[live]
    #[redraw]
    #[find]
    pub body: GTableBody,
    #[live]
    #[redraw]
    #[find]
    pub body_virtual: GVTableBody,
    #[live]
    pub mode: ComponentMode,
}

impl Widget for GTable {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        self.draw_table.begin(cx, walk, self.layout);
        let header_walk = self.header.walk(cx);
        let _ = self.header.draw_walk(cx, scope, header_walk);
        match self.mode {
            ComponentMode::Real => {
                let body_walk = self.body.walk(cx);
                let _ = self.body.draw_walk(cx, scope, body_walk);
            }
            ComponentMode::Virtual => {
                let body_walk = self.body_virtual.walk(cx);
                let _ = self.body_virtual.draw_walk(cx, scope, body_walk);
            }
        }

        self.draw_table.end(cx);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        match self.mode{
            ComponentMode::Real => {
                self.body.handle_event(cx, event, scope);
            },
            ComponentMode::Virtual => {
                self.body_virtual.handle_event(cx, event, scope);
            },
        }   
    }

    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GTable {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.visible {
            return;
        }
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
        self.draw_table.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                border_color: (border_color),
                border_width: (self.border_width),
                border_radius: (self.border_radius),
                pressed_color: (pressed_color),
                hover_color: (hover_color),
                background_visible: (background_visible),
                shadow_color: (shadow_color),
                shadow_offset: (self.shadow_offset),
                spread_radius: (self.spread_radius),
                blur_radius: (self.blur_radius)
            },
        );
        self.draw_table.redraw(cx);
    }
}

impl GTable {
    widget_area! {
        area, draw_table,
        area_header, header,
        area_body, body
    }
}
