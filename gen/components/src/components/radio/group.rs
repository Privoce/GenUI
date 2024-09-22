use makepad_widgets::*;

use crate::{components::card::GCard, event_option, ref_event_option, set_event};

use super::{
    event::{GRadioGroupEvent, GRadioGroupEventParam},
    GRadioWidgetRefExt,
};

live_design! {
    GRadioGroupBase = {{GRadioGroup}} {
        border_radius: 0.0,
        border_width: 0.0,
        spread_radius: 0.0,
        background_visible: false,
        height: Fit,
        width: Fit,
        animation_open: true,
        spacing: 6.0,
    }
}

#[derive(Live, Widget)]
pub struct GRadioGroup {
    #[deref]
    pub deref_widget: GCard,
    #[live(0)]
    pub selected: i32,
}

impl Widget for GRadioGroup {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let _ = self.animator_handle_event(cx, event);
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        let mut flag = false;
        let mut selected = 0;
        let mut e = None;
        // try only do less to control event loop
        for (index, (_id, child)) in self.children.iter().enumerate() {
            let _ = child.as_gradio().borrow().map(|radio| {
                if let Some(param) = radio.clicked(&actions) {
                    if param.value {
                        if (index as i32).ne(&self.selected) {
                            selected = index;
                            flag = true;
                        } else {
                            flag = false;
                        }
                        e.replace(param.e);
                    }
                }
            });
            // if flag is true break to stop
            if flag {
                break;
            }
        }
        if flag {
            self.set_selected(cx, selected);
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                GRadioGroupEvent::Changed(GRadioGroupEventParam {
                    selected,
                    e: e.unwrap(),
                }),
            );
        }
    }
}

impl LiveHook for GRadioGroup {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
        let _ = self.find_selected();
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
        let mut flag = true;
        let mut selected = 0;
        let _ = self
            .children
            .iter()
            .map(|(_id, child)| {
                if let Some(child) = child.as_gradio().borrow() {
                    child.value
                } else {
                    panic!("GRadioGroup only allows GRadio as child!");
                }
            })
            .enumerate()
            .for_each(|(index, is_selected)| {
                if is_selected && flag {
                    selected = index;
                    flag = false;
                } else if is_selected && !flag {
                    panic!(
                        "In GRadioGroup only allows one radio be selected! The Second is: {}",
                        index
                    );
                }
            });

        if !flag {
            self.selected = selected as i32;
        }
    }
    pub fn area(&self) -> Area {
        self.area
    }
    event_option! {
        changed: GRadioGroupEvent::Changed => GRadioGroupEventParam
    }
}

impl GRadioGroupRef{
    ref_event_option! {
        changed => GRadioGroupEventParam
    }
}

impl GRadioGroupSet {
    set_event! {
        changed => GRadioGroupEventParam
    }
}