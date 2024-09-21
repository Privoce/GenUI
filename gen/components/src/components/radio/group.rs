use makepad_widgets::*;

use crate::components::card::GCard;

use super::GRadioWidgetRefExt;

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
    pub select: i32,
}

impl Widget for GRadioGroup {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let _ = self.animator_handle_event(cx, event);
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        let mut flag = false;
        let mut select = 0;

        // try only do less to control event loop
        for (index, (_id, child)) in self.children.iter().enumerate() {
            let _ = child.as_gradio().borrow().map(|radio| {
                if let Some(param) = radio.clicked(&actions) {
                    if param.value {
                        if (index as i32).ne(&self.select) {
                            select = index;
                            flag = true;
                        } else {
                            flag = false;
                        }
                    }
                }
            });
            // if flag is true break to stop
            if flag {
                break;
            }
        }
        if flag {
            self.select = select as i32;
            self.set_select(cx);
        }
       
    }
}

impl LiveHook for GRadioGroup {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
        let _ = self.find_select();
    }
}

impl GRadioGroup {
    fn set_select(&mut self, cx: &mut Cx) -> () {
        let select = self.select as usize;
        // loop all gradio child and let select == false except self.select is true
        self.children
            .iter_mut()
            .enumerate()
            .for_each(|(index, (_id, child))| {
                if let Some(mut child) = child.as_gradio().borrow_mut() {
                    child.toggle(cx, index == select);
                } else {
                    panic!("GRadioGroup only allows GRadio as child!");
                }
            });
    }
    fn find_select(&mut self) -> () {
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
            self.select = selected as i32;
        }
    }
}
