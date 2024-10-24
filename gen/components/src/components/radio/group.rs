use makepad_widgets::*;

use crate::{
    components::view::GView, event_option, ref_actives, ref_area, ref_event_option, ref_redraw_mut, ref_render, set_event
};

use super::{
    event::{GRadioGroupEvent, GRadioGroupEventParam},
    GRadioRef, GRadioWidgetRefExt,
};

live_design! {
    GRadioGroupBase = {{GRadioGroup}} {
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
pub struct GRadioGroup {
    #[deref]
    pub deref_widget: GView,
    #[live(-1)]
    pub selected: i32,
}

impl Widget for GRadioGroup {
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
        let mut flag = None;
        let mut selected = 0;
        let mut e = None;
        // try only do less to control event loop
        for (index, (_id, child)) in self.children.iter().enumerate() {
            let _ = child.as_gradio().borrow().map(|radio| {
                if let Some(param) = radio.clicked(&actions) {
                    if param.selected {
                        if (index as i32).ne(&self.selected) {
                            selected = index;
                            flag.replace(param.value);
                        } else {
                            flag = None;
                        }
                        e.replace(param.e);
                    }
                }
            });
            // if flag is true break to stop
            if flag.is_some() {
                break;
            }
        }
        if let Some(value) = flag {
            self.set_selected(cx, selected);
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                GRadioGroupEvent::Changed(GRadioGroupEventParam {
                    selected,
                    e: e.unwrap(),
                    value,
                }),
            );
        }
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GRadioGroup {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
        if self.selected < 0 {
            let _ = self.find_selected();
        } else {
            self.set_selected(cx, self.selected as usize);
        }
    }
}

impl GRadioGroup {
    pub fn set_selected(&mut self, cx: &mut Cx, selected: usize) -> () {
        self.selected = selected as i32;

        // loop all gradio child and let selected == false except self.selected is true
        self.children
            .iter_mut()
            .enumerate()
            .for_each(|(index, (_id, child))| {
                if let Some(mut child) = child.as_gradio().borrow_mut() {
                    child.toggle(cx, index == selected);
                } else {
                    panic!("GRadioGroup only allows GRadio as child!");
                }
            });
    }
    fn find_selected(&mut self) -> () {
        let mut flag = false;
        let mut selected = 0;
        let _ = self
            .children
            .iter()
            .map(|(_id, child)| {
                if let Some(child) = child.as_gradio().borrow() {
                    child.selected
                } else {
                    panic!("GRadioGroup only allows GRadio as child!");
                }
            })
            .enumerate()
            .for_each(|(index, is_selected)| {
                if is_selected && flag {
                    selected = index;
                    flag = true;
                } else if is_selected && !flag {
                    panic!(
                        "In GRadioGroup only allows one radio be selected! The Second is: {}",
                        index
                    );
                }
            });

        if flag {
            self.selected = selected as i32;
        }
    }
    pub fn area(&self) -> Area {
        self.area
    }
    pub fn get(&self, index: usize) -> Option<(LiveId, GRadioRef)> {
        self.children
            .get(index)
            .map(|(id, child)| (id.clone(), child.as_gradio()))
    }
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.deref_widget.redraw(cx);
    }
    pub fn render(&mut self, cx: &mut Cx) {
        self.deref_widget.render(cx);
    }
    event_option! {
        changed: GRadioGroupEvent::Changed => GRadioGroupEventParam
    }
    pub fn active_selected(&mut self, cx: &mut Cx, e: Option<FingerUpEvent>) {
        let value = self
            .get(self.selected as usize)
            .map(|(_, child)| child.value())
            .flatten();

        if let Some(path) = self.scope_path.as_ref() {
            cx.widget_action(
                self.widget_uid(),
                path,
                GRadioGroupEvent::Changed(GRadioGroupEventParam {
                    selected: self.selected as usize,
                    value,
                    e,
                }),
            );
        }
    }
    /// Change the selected radio by index. It will call the changed event.
    pub fn change(&mut self, cx: &mut Cx, index: usize) {
        if index >= self.children.len() {
            panic!("Index out of range!");
        }

        self.set_selected(cx, index);
        self.active_selected(cx, None);
    }
}

impl GRadioGroupRef {
    ref_event_option! {
        changed => GRadioGroupEventParam
    }
    ref_area!();
    ref_redraw_mut!();
    ref_render!();
    pub fn get(&self, index: usize) -> Option<(LiveId, GRadioRef)> {
        self.borrow().map(|c_ref| c_ref.get(index)).flatten()
    }
    pub fn change(&self, cx: &mut Cx, index: usize) {
        self.borrow_mut().map(|mut c_ref| c_ref.change(cx, index));
    }
    ref_actives!{
        active_selected: Option<FingerUpEvent>
    }
}

impl GRadioGroupSet {
    set_event! {
        changed => GRadioGroupEventParam
    }
}
