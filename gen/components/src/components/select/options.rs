use makepad_widgets::*;

use crate::{components::card::GCard, shader::draw_card::DrawGCard};

use super::{item::GSelectItem, GSelectItemEvent, GSelectOptionsChangedParam, GSelectOptionsEvent};

live_design! {
    GSelectOptionsBase = {{GSelectOptions}}{
        height: Fill,
        width: 360.0,
    }
}

#[derive(Live, LiveRegister)]
pub struct GSelectOptions {
    #[live]
    pub draw_options: DrawGCard,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[live(true)]
    pub visible: bool,
    #[rust]
    pub children: ComponentMap<LiveId, GSelectItem>,
    #[live]
    pub draw_list: DrawList2d,
    #[live]
    pub item: Option<LivePtr>,
}

impl LiveHook for GSelectOptions {}

impl GSelectOptions {
    pub fn area(&self) -> Area {
        self.draw_options.area()
    }
    pub fn menu_contains_pos(&self, cx: &mut Cx, pos: DVec2) -> bool {
        self.draw_options.area().clipped_rect(cx).contains(pos)
    }
    /// ## Begin to draw popup
    /// this method is used to begin drawing the popup
    pub fn begin(&mut self, cx: &mut Cx2d) {
        self.draw_list.begin_overlay_reuse(cx);
        cx.begin_pass_sized_turtle(Layout::flow_down());
        self.draw_options.begin(cx, self.walk, self.layout);
    }
    /// ## End to draw popup
    pub fn end(&mut self, cx: &mut Cx2d, _scope: &mut Scope, shift_area: Area, shift: DVec2) {
        self.draw_options.end(cx);
        cx.end_pass_sized_turtle_with_shift(shift_area, shift);
        self.draw_list.end(cx);
    }
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.draw_list.redraw(cx);
        // self.draw_options.redraw(cx);
    }
    pub fn draw_options(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        item_id: LiveId,
        text: &str,
        value: &str,
    ) {
        let target = self
            .children
            .get_or_insert(cx, item_id, |cx| GSelectItem::new_from_ptr(cx, self.item));
        target.draw_item(cx, text, value);
    }
    pub fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        sweep_area: Area,
        dispatch_action: &mut dyn FnMut(&mut Cx, GSelectOptionsEvent),
    ) {
        let mut actions = Vec::new();
        for (item_id, node) in self.children.iter_mut() {
            node.handle_event_with(cx, event, sweep_area, &mut |_, e| {
                actions.push((*item_id, e))
            });
        }

        for (node_id, action) in actions {
            match action {
                GSelectItemEvent::Clicked(param) => {
                    // if is item clicked, do options event change
                    dispatch_action(
                        cx,
                        GSelectOptionsEvent::Changed(GSelectOptionsChangedParam {
                            selected: param.selected,
                            text: param.text,
                            value: param.value,
                            selected_id: node_id.0 as usize,
                            e: param.e,
                        }),
                    )
                }
                _ => (),
            }
        }
    }
}
