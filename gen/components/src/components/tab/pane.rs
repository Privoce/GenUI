use makepad_widgets::*;

use crate::{components::view::DrawState, shader::draw_tab_pane::DrawTabPane};

use super::body::{GTabBodyRef, GTabBodyWidgetRefExt};

live_design! {
    GTabPaneBase = {{GTabPane}}{}
}

#[derive(Live, Widget)]
pub struct GTabPane {
    #[live(0_usize)]
    pub selected: usize,
    #[rust]
    pub children: ComponentMap<LiveId, GTabBodyRef>,
    #[rust]
    pub draw_order: Vec<LiveId>,
    #[rust]
    pub draw_state: DrawStateWrap<DrawState>,
    #[rust]
    pub defer_walks: Vec<(LiveId, DeferWalk)>,
    #[redraw]
    #[live]
    pub draw_tab_pane: DrawTabPane,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[live(true)]
    pub event_key: bool,
}

impl Widget for GTabPane {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_tab_pane.begin(cx, walk, self.layout);

        let mut target_id = None;

        let _ = self
            .children
            .iter()
            .enumerate()
            .for_each(|(index, (id, _child_ref))| {
                if index == self.selected {
                    target_id.replace(id);
                }
            });

        target_id.map(|id| {
            if let Some(target) = self.children.get(id) {
                let walk = target.walk(cx);
                let _ = target.draw_walk(cx, scope, walk);
            }
        });

        self.draw_tab_pane.end(cx);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // let uid = self.widget_uid();

        let _ = self
            .children
            .iter()
            .enumerate()
            .for_each(|(index, (_id, child_ref))| {
                if index == self.selected {
                    child_ref.handle_event(cx, event, scope);
                }
            });
    }
}

impl LiveHook for GTabPane {
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
                        .get_or_insert(cx, id, |cx| WidgetRef::new(cx).as_gtab_body())
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

impl GTabPane {
    pub fn remove(&mut self, index: usize) {
        self.children.remove(&self.draw_order[index]);
        self.draw_order.remove(index);
    }
    pub fn area(&self) -> Area {
        self.draw_tab_pane.area
    }
    pub fn header_items(&self) -> Vec<String> {
        self.children
            .iter()
            .map(|(_, tab_body_ref)| tab_body_ref.text())
            .collect()
    }
}
