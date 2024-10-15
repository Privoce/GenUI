use makepad_widgets::*;

use crate::{
    components::{
        view::DrawState,
        table::row::{GTableRowRef, GTableRowWidgetRefExt},
    },
    shader::draw_view::DrawGView,
};

live_design! {
    GTableBodyBase = {{GTableBody}}{
        // background_visible: true,
        // border_width: 0.0,
        // border_radius: 0.0,
        padding: 0.0,
        margin: 0.0,
        flow: Down,
    }
}

#[derive(Live, Widget)]
pub struct GTableBody {
    #[live(true)]
    pub visible: bool,
    #[redraw]
    #[live]
    pub draw_table_body: DrawGView,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[rust]
    pub children: ComponentMap<LiveId, GTableRowRef>,
    #[rust]
    pub draw_order: Vec<LiveId>,
    #[rust]
    pub defer_walks: Vec<(LiveId, DeferWalk)>,
    #[rust]
    pub draw_state: DrawStateWrap<DrawState>,
    #[live(true)]
    pub event_key: bool,
}

impl Widget for GTableBody {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // begin the draw state
        if self.draw_state.begin(cx, DrawState::Drawing(0, false)) {
            if !self.visible {
                // visible is false, so we are done
                self.draw_state.end();
                return DrawStep::done();
            }
            self.defer_walks.clear();

            // begin draw the view
            let _ = self.draw_table_body.begin(cx, walk, self.layout);
        }
        // loop handle the inner children
        while let Some(DrawState::Drawing(step, resumed)) = self.draw_state.get() {
            if step < self.draw_order.len() {
                // get id from draw_order list
                let id = self.draw_order[step];
                // get the child widget by id
                if let Some(child) = self.children.get_mut(&id) {
                    // is the child visible?
                    // true -> draw the child walk
                    if child.is_visible() {
                        let walk = child.walk(cx);
                        // if resumed
                        if !resumed {
                            self.draw_state.set(DrawState::Drawing(step, true));
                        }
                        scope.with_id(id, |scope| child.draw_walk(cx, scope, walk))?;
                    }
                }
                // set the next step
                self.draw_state.set(DrawState::Drawing(step + 1, false));
            } else {
                self.draw_state.set(DrawState::DeferWalk(0));
            }
        }

        // loop handle the defer walk
        while let Some(DrawState::DeferWalk(step)) = self.draw_state.get() {
            if step < self.defer_walks.len() {
                let (id, d_walk) = &mut self.defer_walks[step];
                if let Some(child) = self.children.get_mut(&id) {
                    let walk = d_walk.resolve(cx);
                    scope.with_id(*id, |scope| child.draw_walk(cx, scope, walk))?;
                }
                self.draw_state.set(DrawState::DeferWalk(step + 1));
            } else {
                // draw background
                self.draw_table_body.end(cx);
            }
            self.draw_state.end();
        }

        DrawStep::done()
    }
    // fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
    //     self.deref_widget.handle_event(cx, event, scope)
    // }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        for (_index, (_id, row)) in self.children.iter().enumerate() {
            if row.is_visible() {
                row.handle_event(cx, event, scope);
            }
        }
    }

    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GTableBody {
    fn apply_value_instance(
        &mut self,
        cx: &mut Cx,
        apply: &mut Apply,
        index: usize,
        nodes: &[LiveNode],
    ) -> usize {
        let id = nodes[index].id;
        match apply.from {
            ApplyFrom::Animate | ApplyFrom::Over => {
                if let Some(child) = self.children.get_mut(&id) {
                    child.apply(cx, apply, index, nodes)
                } else {
                    nodes.skip_node(index)
                }
            }
            ApplyFrom::NewFromDoc { .. } | ApplyFrom::UpdateFromDoc { .. } => {
                if nodes[index].is_instance_prop() {
                    self.draw_order.push(id);
                    return self
                        .children
                        .get_or_insert(cx, id, |cx| WidgetRef::new(cx).as_gtable_row())
                        .apply(cx, apply, index, nodes);
                } else {
                    cx.apply_error_no_matching_field(live_error_origin!(), index, nodes);
                    nodes.skip_node(index)
                }
            }
            _ => nodes.skip_node(index),
        }
    }
}

impl GTableBody {
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.draw_table_body.redraw(cx);
        for (_, child) in self.children.iter() {
            child.borrow_mut().map(|mut child| {
                child.redraw(cx);
            });
        }
    }
}
