use makepad_widgets::*;

use crate::shader::draw_card::DrawGCard;

live_design! {
    GCollapseBase = {{GCollapse}}{}
}

#[derive(Live, Widget)]
pub struct GCollapse {
    #[live]
    #[redraw]
    #[find]
    pub header: WidgetRef,
    #[live]
    #[redraw]
    #[find]
    pub body: WidgetRef,
    #[redraw]
    #[live]
    pub draw_collapse: DrawGCard,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[rust]
    rect_size: f64,
    #[rust]
    area: Area,
    #[live]
    opened: f64,
    #[rust]
    draw_state: DrawStateWrap<DrawCollapseState>,
    #[animator]
    animator: Animator,
}

#[derive(Clone, Copy)]
enum DrawCollapseState {
    DrawHeader,
    DrawBody,
}

#[derive(Live, LiveHook, Clone, Copy)]
#[live_ignore]
#[repr(u32)]
pub enum IconPosition {
    #[pick]
    Left,
    Right,
}

impl Widget for GCollapse {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if self.draw_state.begin(cx, DrawCollapseState::DrawHeader) {
            cx.begin_turtle(walk, self.layout);
        }
        if let Some(DrawCollapseState::DrawHeader) = self.draw_state.get() {
            let walk = self.header.walk(cx);
            self.header.draw_walk(cx, scope, walk)?;

            let body_walk = self.body.walk(cx);
            
            cx.begin_turtle(
                body_walk,
                Layout::flow_down().with_scroll(dvec2(0.0, self.rect_size * (1.0 - self.opened))),
            );
            self.draw_state.set(DrawCollapseState::DrawBody);
        }
        if self.opened == 0.0 {
            cx.end_turtle();
            cx.end_turtle_with_area(&mut self.area);
            self.draw_state.end();
        }else{
            if let Some(DrawCollapseState::DrawBody) = self.draw_state.get() {
                let walk = self.body.walk(cx);
                self.body.draw_walk(cx, scope, walk)?;
                self.rect_size = cx.turtle().used().y;
                cx.end_turtle();
                cx.end_turtle_with_area(&mut self.area);
                self.draw_state.end();
            }
        }

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            if self.animator.is_track_animating(cx, id!(open)) {
                self.area.redraw(cx);
            }
        };
        self.header.handle_event(cx, event, scope);
        if let Event::Actions(actions) = event {
            match actions
                .find_widget_action(self.header.widget(id!(fold_button)).widget_uid())
                .cast()
            {
                FoldButtonAction::Opening => self.animator_play(cx, id!(open.on)),
                FoldButtonAction::Closing => self.animator_play(cx, id!(open.off)),
                _ => (),
            }
        }
    }
}

impl LiveHook for GCollapse {}
