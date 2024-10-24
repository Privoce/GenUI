mod register;
pub use register::register;

use makepad_widgets::*;

use crate::{animatie_fn, ref_area, ref_event_option, ref_redraw_mut, ref_render, shader::manual::Direction};

use super::view::{
    event::{GViewClickedParam, GViewFocusLostParam, GViewFocusParam, GViewHoverParam},
    GView,
};

live_design! {
    import makepad_draw::shader::std::*;
    import crate::components::view::GViewBase;

    GDividerBase = {{GDivider}}{
        height: 2.0,
        width: Fill,
        align: {x: 0.5, y: 0.5},
        draw_view: {
            // direction is 1.0 for horizontal and 0.0 for vertical
            instance direction: 1.0,
            instance stroke_width: 1.4,
            fn pixel(self) -> vec4{
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                if self.direction == 1.0 {
                    sdf.box(
                        self.pos.x,
                        self.pos.y + self.rect_size.y / 2.0 - self.stroke_width / 2.0,
                        self.rect_size.x,
                        self.stroke_width,
                        max(1.0, self.border_radius)
                    );
                } else {
                    sdf.box(
                        self.pos.x + self.rect_size.x / 2.0 - self.stroke_width / 2.0,
                        self.pos.y,
                        self.stroke_width,
                        self.rect_size.y,
                        max(1.0, self.border_radius)
                    );
                }

                if self.background_visible != 0.0 {
                    sdf.fill(self.get_background_color());
                }
                return sdf.result;
            }
        }
    }
}

/// # GDivider component
/// The `GDivider` component is used to create a simple dividing line between other UI elements. It can be horizontal or vertical based on the `direction` property.
///
/// ## Animation
/// The `GDivider` inherits animation properties from `GView`, but typically, animations are not the primary focus for dividers. Instead, animations should be handled within inner components.
///
/// ## Event
/// The `GDivider` inherits event handling from `GView`. However, since it functions mainly as a visual separator, its event handling is generally minimal.
///
/// ## Props
/// |macro |prop           |description                          |type         |default|
/// |------|---------------|--------------------------------------|-------------|-------|
/// |live  |direction       |Divider direction: horizontal (1.0) or vertical (0.0)|f32 |1.0|
/// |live  |stroke_width    |The width of the divider's stroke    |f32          |1.4    |
///
/// > Other Props see: [GView]
#[derive(Live, Widget)]
pub struct GDivider {
    #[deref]
    pub deref_widget: GView,
    #[live(1.4)]
    pub stroke_width: f32,
    #[live(Direction::Horizontal)]
    pub direction: Direction
}

impl Widget for GDivider {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        self.deref_widget.handle_event(cx, event, scope)
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GDivider {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
        self.add_render(cx);
    }
}

impl GDivider {
    fn add_render(&mut self, cx: &mut Cx) {
        let stroke_width = self.stroke_width;
        let direction = self.direction.to_f32();
        // now set stroke width to draw_view
        self.draw_view.apply_over(
            cx,
            live! {
                stroke_width: (stroke_width),
                direction: (direction)
            },
        );
    }
    pub fn animate_hover_on(&mut self, cx: &mut Cx) {
        self.deref_widget.animate_hover_on(cx);
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) {
        self.deref_widget.animate_hover_off(cx);
    }
    pub fn animate_focus_on(&mut self, cx: &mut Cx) {
        self.deref_widget.animate_focus_on(cx);
    }
    pub fn animate_focus_off(&mut self, cx: &mut Cx) {
        self.deref_widget.animate_focus_off(cx);
    }
    pub fn clear_animation(&mut self, cx: &mut Cx) {
        self.deref_widget.clear_animation(cx);
    }
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.deref_widget.redraw(cx);
    }
    pub fn render(&mut self, cx: &mut Cx) {
        self.deref_widget.render(cx);
        self.add_render(cx);
    }
    pub fn area(&self) -> Area {
        self.deref_widget.area()
    }
    pub fn hover_in(&self, actions: &Actions) -> Option<GViewHoverParam> {
        self.deref_widget.hover_in(actions)
    }
    pub fn hover_out(&self, actions: &Actions) -> Option<GViewHoverParam> {
        self.deref_widget.hover_out(actions)
    }
    pub fn focus(&self, actions: &Actions) -> Option<GViewFocusParam> {
        self.deref_widget.focus(actions)
    }
    pub fn focus_lost(&self, actions: &Actions) -> Option<GViewFocusLostParam> {
        self.deref_widget.focus_lost(actions)
    }
    pub fn clicked(&self, actions: &Actions) -> Option<GViewClickedParam> {
        self.deref_widget.clicked(actions)
    }
}

impl GDividerRef {
    ref_area!();
    ref_redraw_mut!();
    ref_render!();
    animatie_fn! {
        clear_animation,
        animate_hover_on,
        animate_hover_off,
        animate_focus_on,
        animate_focus_off
    }
    ref_event_option! {
        hover_in => GViewHoverParam,
        hover_out => GViewHoverParam,
        focus => GViewFocusParam,
        focus_lost => GViewFocusLostParam,
        clicked => GViewClickedParam
    }
}
