use makepad_widgets::*;

use crate::{components::card::DrawState, shader::draw_card::DrawGCard};

use super::cell::{GTableCellRef, GTableCellWidgetRefExt};

live_design! {
    GTableRowBase = {{GTableRow}}{
    }
}

#[derive(Live, Widget)]
pub struct GTableRow {
    #[rust]
    pub draw_order: Vec<LiveId>,
    #[redraw]
    #[live]
    pub draw_table_row: DrawGCard,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[rust]
    pub children: ComponentMap<LiveId, GTableCellRef>,
    #[rust]
    pub draw_state: DrawStateWrap<DrawState>,
    #[live(true)]
    pub visible: bool,
    #[rust]
    pub defer_walks: Vec<(LiveId, DeferWalk)>,
}

impl Widget for GTableRow {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // begin the draw state
        if self.draw_state.begin(cx, DrawState::Drawing(0, false)) {
            if !self.visible {
                // visible is false, so we are done
                self.draw_state.end();
                return DrawStep::done();
            }
            self.defer_walks.clear();

            // begin draw the card
            let _ = self.draw_table_row.begin(cx, walk, self.layout);
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
                self.draw_table_row.end(cx);
            }
            self.draw_state.end();
        }

        DrawStep::done()
    }
    // fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
    //     let uid = self.widget_uid();
    //     if self.animator_handle_event(cx, event).must_redraw() {
    //         self.redraw(cx);
    //     }

    //     if self.block_signal_event {
    //         if let Event::Signal = event {
    //             return;
    //         }
    //     }

    //     if let Some(scroll_bars) = &mut self.scroll_bars_obj {
    //         let mut actions = Vec::new();
    //         scroll_bars.handle_main_event(cx, event, &mut actions);
    //         if actions.len().gt(&0) {
    //             cx.redraw_area_and_children(self.area());
    //         }
    //     }

    //     match &self.event_order {
    //         EventOrder::Down => {
    //             for id in self.draw_order.iter() {
    //                 if let Some(child) = self.children.get_mut(id) {
    //                     if child.is_visible() || !event.requires_visibility() {
    //                         scope.with_id(*id, |scope| {
    //                             child.handle_event(cx, event, scope);
    //                         })
    //                     }
    //                 }
    //             }
    //         }
    //         EventOrder::Up => {
    //             // the default event order is Up
    //             for id in self.draw_order.iter().rev() {
    //                 if let Some(child) = self.children.get_mut(id) {
    //                     if child.is_visible() || !event.requires_visibility() {
    //                         scope.with_id(*id, |scope| {
    //                             child.handle_event(cx, event, scope);
    //                         });
    //                     }
    //                 }
    //             }
    //         }
    //         EventOrder::List(list) => {
    //             for id in list {
    //                 if let Some(child) = self.children.get_mut(id) {
    //                     if child.is_visible() || !event.requires_visibility() {
    //                         scope.with_id(*id, |scope| {
    //                             child.handle_event(cx, event, scope);
    //                         })
    //                     }
    //                 }
    //             }
    //         }
    //     }

    //     // handle event and set cursor to control
    //     match event.hits(cx, self.area()) {
    //         Hit::KeyDown(e) => {
    //             if self.grab_key_focus {
    //                 cx.widget_action(uid, &scope.path, CardEvent::KeyDown(e))
    //             }
    //         }
    //         Hit::KeyUp(e) => {
    //             if self.grab_key_focus {
    //                 cx.widget_action(uid, &scope.path, CardEvent::KeyUp(e))
    //             }
    //         }
    //         // Hit::FingerScroll(e) => cx.widget_action(uid, &scope.path, CardEvent::FingerScroll(e)),
    //         Hit::FingerDown(e) => {
    //             if self.grab_key_focus {
    //                 cx.set_key_focus(self.area());
    //             }
    //             cx.widget_action(uid, &scope.path, CardEvent::FingerDown(e));
    //         }
    //         Hit::FingerMove(e) => cx.widget_action(uid, &scope.path, CardEvent::FingerMove(e)),
    //         Hit::FingerHoverIn(e) => {
    //             let _ = set_cursor(cx, self.cursor.as_ref());
    //             cx.widget_action(uid, &scope.path, CardEvent::FingerHoverIn(e));
    //             if self.animator.live_ptr.is_some() && self.animator_key {
    //                 self.animator_play(cx, id!(hover.on))
    //             }
    //         }
    //         Hit::FingerHoverOver(e) => {
    //             cx.widget_action(uid, &scope.path, CardEvent::FingerHoverOver(e));
    //         }
    //         Hit::FingerHoverOut(e) => {
    //             cx.widget_action(uid, &scope.path, CardEvent::FingerHoverOut(e));
    //             if self.animator.live_ptr.is_some() && self.animator_key {
    //                 self.animator_play(cx, id!(hover.off))
    //             }
    //         }
    //         Hit::FingerUp(e) => {
    //             cx.widget_action(uid, &scope.path, CardEvent::FingerUp(e));
    //         }
    //         _ => (),
    //     }
    //     if let Some(scroll_bars) = &mut self.scroll_bars_obj {
    //         scroll_bars.handle_scroll_event(cx, event, &mut Vec::new());
    //     }
    // }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GTableRow {
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
                        .get_or_insert(cx, id, |cx| WidgetRef::new(cx).as_gtable_cell())
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
