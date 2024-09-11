use makepad_widgets::*;

use crate::{
    shader::{draw_popup::DrawGPopup, manual::PopupMode},
    themes::Themes,
    utils::{BoolToF32, ThemeColor},
};

use super::card::Card;

live_design! {
    GPopupContainerBase = {{GPopupContainer}} {}
    GPopupBase = {{GPopup}} {}
}
#[derive(Live, LiveRegister)]
pub struct GPopupContainer {
    #[live]
    #[deref]
    pub super_widget: Card,
}

impl LiveHook for GPopupContainer {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.get(self.theme, 500);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.get(self.theme, 400);
        // ------------------ pressed color ---------------------------------------------
        let pressed_color = self.pressed_color.get(self.theme, 600);
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 800);
        // ------------------ is background_visible --------------------------------------------
        let background_visible = self.background_visible.to_f32();
        // ------------------ apply draw_popup --------------------------------------------
        let border_width = self.border_width;
        let border_radius = self.border_radius;
        self.draw_card.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                border_color: (border_color),
                border_width: (border_width),
                border_radius: (border_radius),
                pressed_color: (pressed_color),
                hover_color: (hover_color),
                background_visible: (background_visible),
            },
        );
        self.draw_card.redraw(cx);
    }
}

impl GPopupContainer {
    pub fn area(&self) -> Area {
        self.draw_card.area
    }
    pub fn draw_item(&mut self, cx: &mut Cx2d, scope: &mut Scope) {
        let _ = self.super_widget.draw_walk(cx, scope, self.walk);
    }

    pub fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        sweep_area: Area,
        scope: &mut Scope,
    ) {
        let _ = self
            .super_widget
            .handle_event_with(cx, event, scope, sweep_area);
    }
}

/// A popup is a floating window that appears on top of other content
/// It can be used to display an additional information or to ask for a confirmation
#[derive(Live, LiveRegister)]
pub struct GPopup {
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
    #[live(true)]
    pub visible: bool,
    #[live(false)]
    pub background_visible: bool,
    #[live]
    pub cursor: Option<MouseCursor>,
    #[live(false)]
    pub animator_key: bool,
    #[live]
    pub mode: PopupMode,
    // deref ---------------------
    #[live]
    pub draw_popup: DrawGPopup,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[live]
    pub container: GPopupContainer,
    /// draw list is necessary!!!
    /// because we need to draw the popup on top of everything
    /// although the name of DrawList2d may let you think it's only for 2d list drawing
    /// actually it's for all the drawing that needs to be on top of everything!!!
    #[live]
    draw_list: DrawList2d,
}

impl LiveHook for GPopup {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.get(self.theme, 500);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.get(self.theme, 400);
        // ------------------ pressed color ---------------------------------------------
        let pressed_color = self.pressed_color.get(self.theme, 600);
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 800);
        // ------------------ is background_visible --------------------------------------------
        let background_visible = self.background_visible.to_f32();
        // ------------------ apply draw_popup --------------------------------------------
        self.draw_popup.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                border_color: (border_color),
                border_width: (self.border_width),
                border_radius: (self.border_radius),
                pressed_color: (pressed_color),
                hover_color: (hover_color),
                background_visible: (background_visible),
            },
        );
        self.draw_popup.redraw(cx);
    }
}

impl GPopup {
    pub fn area(&self) -> Area {
        self.draw_popup.area()
    }
    /// ## Get the popup container position
    /// if you need to get the position of the popup container, use this method
    /// ### Example
    /// ```rust
    /// let global = cx.global::<PopupMenuGlobal>().clone();
    /// let mut map = global.map.borrow_mut();
    /// let popup_menu = map.get_mut(&self.popup.unwrap()).unwrap();
    /// if let Event::MouseDown(e) = event {
    ///     if !popup_menu.menu_contains_pos(cx, e.abs) {
    ///         self.close(cx);
    ///         self.animator_play(cx, id!(hover.off));
    ///         return;
    ///     }
    /// }
    /// ```
    pub fn menu_contains_pos(&self, cx: &mut Cx, pos: DVec2) -> bool {
        self.draw_popup.area().clipped_rect(cx).contains(pos)
    }
    pub fn container_contains_pos(&self, cx: &mut Cx, pos: DVec2) -> bool {
        self.container.area().clipped_rect(cx).contains(pos)
    }
    /// ## Begin to draw popup
    /// this method is used to begin drawing the popup
    pub fn begin(&mut self, cx: &mut Cx2d) {
        self.draw_list.begin_overlay_reuse(cx);
        cx.begin_pass_sized_turtle(Layout::flow_down());

        self.draw_popup.begin(cx, self.walk, self.layout);
    }
    /// ## End to draw popup
    pub fn end(&mut self, cx: &mut Cx2d, _scope: &mut Scope, shift_area: Area, shift: DVec2) {
        self.draw_popup.end(cx);
        cx.end_pass_sized_turtle_with_shift(shift_area, shift);
        self.draw_list.end(cx);
    }
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.draw_list.redraw(cx);
        // self.draw_popup.redraw(cx);
    }
    /// ## Draw items
    pub fn draw_container(&mut self, cx: &mut Cx2d, scope: &mut Scope) {
        self.container.draw_item(cx, scope);
    }
    pub fn container_area(&self) -> Area {
        self.container.area()
    }
    pub fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
        sweep_area: Area,
    ) {
        self.container
            .handle_event_with(cx, event, sweep_area, scope)
    }
}
