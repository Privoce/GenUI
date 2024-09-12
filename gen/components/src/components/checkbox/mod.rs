mod register; 
pub mod event;

pub use register::register;
use makepad_widgets::*;

use crate::{
    shader::{draw_check_box::DrawGCheckBox, draw_radio::GChooseType},
    themes::Themes,
    utils::{set_cursor, ThemeColor},
};

live_design! {
    import makepad_draw::shader::std::*;
    GLOBAL_DURATION = 0.25

    GCheckBoxBase = {{GCheckBox}}{
        animator: {
            hover = {
                default: off
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_check: {hover: 0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_check: {hover: 1.0}
                    }
                }
            }
            selected = {
                default: off
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_check: {selected: 0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_check: {selected: 1.0}
                    }
                }
            }
        }
    }
}

#[derive(Widget, Live)]
pub struct GCheckBox {
    #[live]
    pub theme: Themes,
    #[live(8.0)]
    pub size: f32,
    #[live]
    pub background_color: Option<Vec4>,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub selected_color: Option<Vec4>,
    #[live]
    pub stroke_color: Option<Vec4>,
    #[live]
    pub stroke_hover_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(1.0)]
    pub border_width: f32,
    #[live(0.64)]
    pub scale: f32,
    #[live(MouseCursor::Hand)]
    pub cursor: Option<MouseCursor>,
    #[live]
    pub value: String,
    // ---- type
    #[live]
    check_type: GChooseType,
    // deref -------------------
    #[redraw]
    #[live]
    draw_check: DrawGCheckBox,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    // animator -----------------
    #[animator]
    animator: Animator,
}

#[derive(DefaultNone, Clone, Debug)]
pub enum GCheckBoxEvent {
    /// changed(is_selected, value)
    Changed((bool, String)),
    Hover,
    None,
}

impl Widget for GCheckBox {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_check.draw_walk(cx, walk);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        self.animator_handle_event(cx, event);

        match event.hits(cx, self.draw_check.area()) {
            Hit::FingerHoverIn(_) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                self.animator_play(cx, id!(hover.on));
                cx.widget_action(uid, &scope.path, GCheckBoxEvent::Hover)
            }
            Hit::FingerHoverOut(_) => {
                cx.set_cursor(MouseCursor::Arrow);
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerDown(_fe) => {
                if self.animator_in_state(cx, id!(selected.on)) {
                    self.animator_play(cx, id!(selected.off));
                    cx.widget_action(
                        uid,
                        &scope.path,
                        GCheckBoxEvent::Changed((false, self.value.to_string())),
                    );
                } else {
                    self.animator_play(cx, id!(selected.on));
                    cx.widget_action(
                        uid,
                        &scope.path,
                        GCheckBoxEvent::Changed((true, self.value.to_string())),
                    );
                }
            }
            Hit::FingerUp(_fe) => {}
            Hit::FingerMove(_fe) => {}
            _ => (),
        }
    }
    fn data_to_widget(&mut self, cx: &mut Cx, nodes: &[LiveNode], path: &[LiveId]) {
        if let Some(value) = nodes.read_field_value(path) {
            if let Some(v) = value.as_bool() {
                // set the selected state
                // if the value is true, set the selected state to on or off
                self.animator_toggle(cx, v, Animate::Yes, id!(selected.on), id!(selected.off))
            }
        }
    }
}

impl LiveHook for GCheckBox {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.get(self.theme, 50);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.get(self.theme, 100);
        // ------------------ selected color ---------------------------------------------
        let selected_color = self.selected_color.get(self.theme, 50);
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 600);
        // ------------------ stroke color ---------------------------------------------
        let stroke_color = self.stroke_color.get(self.theme, 600);
        // ------------------ stroke hover color ---------------------------------------
        let stroke_hover_color = self.stroke_hover_color.get(self.theme, 600);
        // ------------------ apply to draw_check ----------------------------------------
        self.draw_check.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                hover_color: (hover_color),
                selected_color: (selected_color),
                stroke_color: (stroke_color),
                stroke_hover_color: (stroke_hover_color),
                border_color: (border_color),
                border_width: (self.border_width),
                scale: (self.scale),
                size: (self.size),
                scale: (self.scale),
            },
        );
        self.draw_check.apply_check_type(self.check_type.clone());

        self.draw_check.redraw(cx);
    }
}

impl GCheckBox {
    pub fn changed(&self, actions: &Actions) -> Option<(bool, String)> {
        if let GCheckBoxEvent::Changed(changed) = actions.find_widget_action_cast(self.widget_uid())
        {
            return Some(changed);
        }
        None
    }
    pub fn hover(&self, actions: &Actions) -> bool {
        if let GCheckBoxEvent::Hover = actions.find_widget_action_cast(self.widget_uid()) {
            true
        } else {
            false
        }
    }
}

impl GCheckBoxRef {
    pub fn changed(&self, actions: &Actions) -> Option<(bool, String)> {
        if let Some(check_box_ref) = self.borrow() {
            return check_box_ref.changed(actions);
        }
        None
    }
    pub fn hover(&self, actions: &Actions) -> bool {
        if let Some(check_box_ref) = self.borrow() {
            return check_box_ref.hover(actions);
        }
        false
    }
}
