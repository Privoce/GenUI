use makepad_widgets::*;

use crate::{
    components::{
        view::DrawState,
        table::row::{GTableRowRef, GTableRowWidgetRefExt},
    },
    shader::draw_view::DrawGView,
    utils::{BoolToF32, ThemeColor}, widget_area,
};

live_design! {
    GTableHeaderBase = {{GTableHeader}}{
        padding: 0.0,
        margin: 0.0,
        flow: Down,
    }
}

#[derive(Live, Widget)]
pub struct GTableHeader {
    #[live]
    pub background_color: Option<Vec4>,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub pressed_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live]
    pub shadow_color: Option<Vec4>,
    #[live(true)]
    pub visible: bool,
    #[live(false)]
    pub background_visible: bool,
    #[live(0.0)]
    pub border_width: f32,
    #[live(0.0)]
    pub spread_radius: f32,
    #[live(4.8)]
    pub blur_radius: f32,
    #[live]
    pub shadow_offset: Vec2,
    #[redraw]
    #[live]
    pub draw_table_header: DrawGView,
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

impl Widget for GTableHeader {
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
            let _ = self.draw_table_header.begin(cx, walk, self.layout);
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
                self.draw_table_header.end(cx);
            }
            self.draw_state.end();
        }

        DrawStep::done()
    }
    // fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
    //     self.deref_widget.handle_event(cx, event, scope)
    // }

    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GTableHeader {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        let bg_color = self.background_color.use_or("#F9FAFB");
        let hover_color = self.hover_color.use_or("#F9FAFB");
        let pressed_color = self.pressed_color.use_or("#F9FAFB");
        let border_color = self.border_color.use_or("#EAECF0");
        let shadow_color = self.shadow_color.use_or("#FFFFFF00");
        self.draw_table_header.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                background_visible: (self.background_visible.to_f32()),
                border_width: (self.border_width),
                border_radius: 0.0,
                pressed_color: (pressed_color),
                hover_color: (hover_color),
                shadow_color: (shadow_color),
                border_color: (border_color),
                shadow_offset: (self.shadow_offset),
                spread_radius: (self.spread_radius),
                blur_radius: (self.blur_radius)
            },
        );
        self.draw_table_header.redraw(cx);
    }
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

impl GTableHeader {
    widget_area! {
        area, draw_table_header
    }
}