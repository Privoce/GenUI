use makepad_widgets::*;

use crate::{
    shader::draw_radio::{DrawGRadio, GChooseType},
    themes::Themes,
    utils::{set_cursor, ThemeColor},
};

live_design! {
    import makepad_draw::shader::std::*;
    GLOBAL_DURATION = 0.25

    GRadioBase = {{GRadio}}{
        animator: {
            hover = {
                default: off
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_radio: {hover: 0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_radio: {hover: 1.0}
                    }
                }
            }
            selected = {
                default: off
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_radio: {selected: 0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_radio: {selected: 1.0}
                    }
                }
            }
        }
    }
}

#[derive(Widget, Live)]
pub struct GRadio {
    #[live]
    pub theme: Themes,
    #[live(8.0)]
    pub size: f32,
    #[live]
    pub background_color: Option<Vec4>,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub focus_color: Option<Vec4>,
    #[live]
    pub selected_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(1.0)]
    pub border_width: f32,
    #[live(0.48)]
    pub scale: f32,
    #[live(MouseCursor::Hand)]
    pub cursor: Option<MouseCursor>,
    // value ------------------
    #[live]
    value: String,
    // ---- type
    #[live]
    radio_type: GChooseType,
    // deref -------------------
    #[redraw]
    #[live]
    draw_radio: DrawGRadio,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    // animator -----------------
    #[animator]
    animator: Animator,
}

#[derive(DefaultNone, Clone, Debug)]
pub enum GRadioEvent {
    Clicked(String),
    Hover,
    None,
}

impl Widget for GRadio {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_radio.draw_walk(cx, walk);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        self.animator_handle_event(cx, event);

        match event.hits(cx, self.draw_radio.area()) {
            Hit::FingerHoverIn(_) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                self.animator_play(cx, id!(hover.on));
                cx.widget_action(uid, &scope.path, GRadioEvent::Hover)
            }
            Hit::FingerHoverOut(_) => {
                cx.set_cursor(MouseCursor::Arrow);
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerDown(_fe) => {
                if self.animator_in_state(cx, id!(selected.off)) {
                    self.animator_play(cx, id!(selected.on));
                    cx.widget_action(
                        uid,
                        &scope.path,
                        GRadioEvent::Clicked(self.value.to_string()),
                    );
                }
            }
            Hit::FingerUp(_fe) => {}
            Hit::FingerMove(_fe) => {}
            _ => (),
        }
    }
}

impl LiveHook for GRadio {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.get(self.theme, 50);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.get(self.theme, 100);
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 600);
        // ------------------ focus color -----------------------------------------------
        let focus_color = self.focus_color.get(self.theme, 600);
        // ------------------ selected color ---------------------------------------------
        let selected_color = self.selected_color.get(self.theme, 600);
        // ------------------ apply to draw_radio ----------------------------------------
        self.draw_radio.apply_over(
            cx,
            live! {
                color: (bg_color),
                hover_color: (hover_color),
                focus_color: (focus_color),
                selected_color: (selected_color),
                border_color: (border_color),
                border_width: (self.border_width),
                scale: (self.scale),
                size: (self.size),
                scale: (self.scale),
            },
        );
        self.draw_radio.apply_radio_type(self.radio_type.clone());

        self.draw_radio.redraw(cx);
    }
}

impl GRadio {
    pub fn clicked(&self, actions: &Actions) -> Option<String> {
        if let GRadioEvent::Clicked(value) = actions.find_widget_action_cast(self.widget_uid()) {
            Some(value)
        } else {
            None
        }
    }
    pub fn hover(&self, actions: &Actions) -> bool {
        if let GRadioEvent::Hover = actions.find_widget_action_cast(self.widget_uid()) {
            true
        } else {
            false
        }
    }
}

impl GRadioRef {
    pub fn clicked(&self, actions: &Actions) -> Option<String> {
        if let Some(radio_ref) = self.borrow() {
            return radio_ref.clicked(actions);
        }
        None
    }
    pub fn hover(&self, actions: &Actions) -> bool {
        if let Some(radio_ref) = self.borrow() {
            return radio_ref.hover(actions);
        }
        false
    }
}
