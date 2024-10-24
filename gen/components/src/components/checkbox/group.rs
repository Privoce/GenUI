use makepad_widgets::*;

use crate::{
    components::view::GView, event_option, ref_actives, ref_area, ref_event_option, ref_redraw_mut,
    ref_render, set_event,
};

use super::{
    event::{GCheckBoxGroupEvent, GCheckBoxGroupEventParam},
    GCheckBoxRef, GCheckBoxWidgetRefExt,
};

live_design! {
    GCheckBoxGroupBase = {{GCheckBoxGroup}} {
        border_radius: 0.0,
        border_width: 0.0,
        spread_radius: 0.0,
        background_visible: false,
        height: Fit,
        width: Fit,
        animation_key: true,
        spacing: 8.0,
    }
}

#[derive(Live, Widget)]
pub struct GCheckBoxGroup {
    #[deref]
    pub deref_widget: GView,
    // selected indexs of checkbox, if selected.len() == 0, means no checkbox is selected
    #[live(vec![])]
    pub selected: Vec<i32>,
}

impl Widget for GCheckBoxGroup {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.is_visible() {
            return;
        }
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        };
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

        let e = match event.hits(cx, self.area()) {
            Hit::FingerUp(e) => Some(e),
            _ => None,
        };
        let mut flag = false;
        for (_, (_id, child)) in self.children.iter().enumerate() {
            let _ = child.as_gcheck_box().borrow().map(|checkbox| {
                if let Some(_) = checkbox.clicked(&actions) {
                    // here we just make sure the clicked is exist
                    flag = true;
                }
            });
            if flag {
                break;
            }
        }
        // ok, we know clicked happened, now we need to find selected
        if flag {
            self.find_selected();
            if let Some(path) = self.scope_path.as_ref() {
                let values = self.values();
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    GCheckBoxGroupEvent::Changed(GCheckBoxGroupEventParam {
                        selected: self.selected.iter().map(|x| *x as usize).collect(),
                        values,
                        e,
                    }),
                );
            }
        }
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GCheckBoxGroup {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
        if self.selected.len() == 0 {
            let _ = self.find_selected();
        } else {
            self.set_selected(cx, self.selected.clone());
        }
    }
}

impl GCheckBoxGroup {
    pub fn set_selected(&mut self, cx: &mut Cx, selected: Vec<i32>) -> () {
        // loop all gcheckbox child and let selected == false except self.selected is true
        self.children
            .iter_mut()
            .enumerate()
            .for_each(|(index, (_id, child))| {
                if let Some(mut child) = child.as_gcheck_box().borrow_mut() {
                    let selected = &selected[index];

                    child.toggle(cx, *selected == index as i32);
                } else {
                    panic!("GCheckBoxGroup only allows GCheckBox as child!");
                }
            });

        self.selected = selected;
    }
    fn find_selected(&mut self) -> () {
        self.selected = self.children.iter().enumerate().fold(
            Vec::new(),
            |mut selected, (index, (_, child))| {
                if let Some(child) = child.as_gcheck_box().borrow() {
                    if child.selected {
                        selected.push(index as i32);
                    }
                } else {
                    panic!("GCheckBoxGroup only allows GCheckBox as child!");
                }
                selected
            },
        );
    }
    pub fn area(&self) -> Area {
        self.area
    }
    pub fn get(&self, index: usize) -> Option<(LiveId, GCheckBoxRef)> {
        self.children
            .get(index)
            .map(|(id, child)| (id.clone(), child.as_gcheck_box()))
    }
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.deref_widget.redraw(cx);
    }
    pub fn render(&mut self, cx: &mut Cx) {
        self.deref_widget.render(cx);
    }
    pub fn active_selected(&mut self, cx: &mut Cx, e: Option<FingerUpEvent>) -> () {
        // for loop self.selected and get value
        let values = self.values();

        if let Some(path) = self.scope_path.as_ref() {
            cx.widget_action(
                self.widget_uid(),
                path,
                GCheckBoxGroupEvent::Changed(GCheckBoxGroupEventParam {
                    selected: self.selected.iter().map(|x| *x as usize).collect(),
                    values,
                    e,
                }),
            );
        }
    }
    /// Change the selected radio by index. It will call the changed event.
    pub fn change(&mut self, cx: &mut Cx, index: Vec<usize>) {
        if index.len() >= self.children.len() {
            panic!("Index out of range!");
        }

        self.set_selected(cx, index.iter().map(|x| *x as i32).collect());
        self.active_selected(cx, None);
    }
    pub fn values(&self) -> Vec<Option<String>> {
        self.selected
            .iter()
            .map(|x| {
                if let Some((_, checkbox)) = self.get(*x as usize) {
                    checkbox.value()
                } else {
                    // I am sure this will never happen but may be checkbox.value() can return None!
                    None
                }
            })
            .collect()
    }
    event_option! {
        changed: GCheckBoxGroupEvent::Changed => GCheckBoxGroupEventParam
    }
}

impl GCheckBoxGroupRef {
    ref_event_option! {
        changed => GCheckBoxGroupEventParam
    }
    ref_area!();
    ref_redraw_mut!();
    ref_render!();
    pub fn get(&self, index: usize) -> Option<(LiveId, GCheckBoxRef)> {
        self.borrow().map(|c_ref| c_ref.get(index)).flatten()
    }
    pub fn change(&self, cx: &mut Cx, index: Vec<usize>) {
        self.borrow_mut().map(|mut c_ref| c_ref.change(cx, index));
    }
    ref_actives! {
        active_selected: Option<FingerUpEvent>
    }
}

impl GCheckBoxGroupSet {
    set_event! {
        changed => GCheckBoxGroupEventParam
    }
}
