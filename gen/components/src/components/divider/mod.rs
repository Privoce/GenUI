mod register;
pub use register::register;

use makepad_widgets::*;

use super::view::GView;

live_design! {
    import makepad_draw::shader::std::*;
    import crate::components::view::GViewBase;

    GDividerBase = {{GDivider}}{
        height: Fit,
        width: Fill,
        align: {x: 0.5, y: 0.5},
        draw_view: {
            instance stroke_width: 1.4,
            fn pixel(self) -> vec4{
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    self.pos.x,
                    self.pos.y + self.rect_size.y / 2.0 - self.stroke_width / 2.0,
                    self.rect_size.x,
                    self.stroke_width,
                    max(1.0, self.border_radius)
                );
                if self.background_visible != 0.0 {
                    sdf.fill(self.get_color());
                }
                return sdf.result;
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct GDivider {
    #[deref]
    pub deref_widget: GView,
    #[live(1.4)]
    pub stroke_width: f32,
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
        let stroke_width = self.stroke_width;
        // now set stroke width to draw_view
        self.draw_view.apply_over(
            cx,
            live! {
                stroke_width: (stroke_width)
            },
        );
    }
}

impl GDivider {}
