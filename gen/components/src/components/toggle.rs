use makepad_widgets::*;

use crate::{
    shader::draw_toggle::DrawGToggle,
    themes::Themes,
    utils::{set_cursor, ThemeColor},
};

live_design! {
    import makepad_draw::shader::std::*;
    GLOBAL_DURATION = 0.25
    GToggleBase = {{GToggle}}{
        animator: {
            hover = {
                default: off
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_toggle: {hover: 0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_toggle: {hover: 1.0}
                    }
                }
            }
            selected = {
                default: off
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_toggle: {selected: 0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_toggle: {selected: 1.0}
                    }
                }
            }
        }
    }
}
#[derive(Live, Widget)]
pub struct GToggle {
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
    // deref -------------------
    #[redraw]
    #[live]
    draw_toggle: DrawGToggle,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    // animator -----------------
    #[animator]
    animator: Animator,
    #[live]
    pub value: String,
}

#[derive(DefaultNone, Clone, Debug)]
pub enum GToggleEvent {
    /// changed(is_selected, value)
    Changed((bool, String)),
    Hover,
    None,
}

impl Widget for GToggle {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_toggle.draw_walk(cx, walk);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        self.animator_handle_event(cx, event);

        match event.hits(cx, self.draw_toggle.area()) {
            Hit::FingerHoverIn(_) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                self.animator_play(cx, id!(hover.on));
                cx.widget_action(uid, &scope.path, GToggleEvent::Hover)
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
                        GToggleEvent::Changed((false, self.value.to_string())),
                    );
                } else {
                    self.animator_play(cx, id!(selected.on));
                    cx.widget_action(
                        uid,
                        &scope.path,
                        GToggleEvent::Changed((true, self.value.to_string())),
                    );
                }
            }
            Hit::FingerUp(_fe) => {}
            Hit::FingerMove(_fe) => {}
            _ => (),
        }
    }
}

impl LiveHook for GToggle {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.get(self.theme, 500);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.get(self.theme, 400);
        // ------------------ selected color ---------------------------------------------
        let selected_color = self.selected_color.get(self.theme, 500);
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 600);
        // ------------------ stroke color ---------------------------------------------
        let stroke_color = self.stroke_color.get(self.theme, 50);
        // ------------------ stroke hover color ---------------------------------------
        let stroke_hover_color = self.stroke_hover_color.get(self.theme, 600);
        // ------------------ apply to draw_toggle ----------------------------------------
        self.draw_toggle.apply_over(
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
            },
        );
        // self.draw_toggle.apply_check_type(self.check_type.clone());

        self.draw_toggle.redraw(cx);
    }
}

impl GToggle {
    pub fn changed(&self, actions: &Actions) -> Option<(bool, String)> {
        if let GToggleEvent::Changed(changed) = actions.find_widget_action_cast(self.widget_uid()) {
            return Some(changed);
        }
        None
    }
    pub fn hover(&self, actions: &Actions) -> bool {
        if let GToggleEvent::Hover = actions.find_widget_action_cast(self.widget_uid()) {
            true
        } else {
            false
        }
    }
}

impl GToggleRef {
    pub fn changed(&self, actions: &Actions) -> Option<(bool, String)> {
        if let Some(toggle_ref) = self.borrow() {
            return toggle_ref.changed(actions);
        }
        None
    }
    pub fn hover(&self, actions: &Actions) -> bool {
        if let Some(toggle_ref) = self.borrow() {
            return toggle_ref.hover(actions);
        }
        false
    }
}
