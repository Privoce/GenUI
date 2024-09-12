pub mod event;
mod register;

use event::*;
use makepad_widgets::*;
pub use register::register;

use crate::{
    animatie_fn, event_option, ref_event_option, set_event,
    shader::{draw_check_box::DrawGCheckBox, draw_radio::GChooseType},
    themes::Themes,
    utils::{set_cursor, BoolToF32, ThemeColor},
    widget_area,
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
                        draw_checkbox: {hover: 0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_checkbox: {hover: 1.0}
                    }
                }
            }
            selected = {
                default: off
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_checkbox: {selected: 0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_checkbox: {selected: 1.0}
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
    #[live(true)]
    pub background_visible: bool,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub selected_color: Option<Vec4>,
    #[live]
    pub stroke_color: Option<Vec4>,
    #[live]
    pub stroke_hover_color: Option<Vec4>,
    #[live]
    pub stroke_selected_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(1.0)]
    pub border_width: f32,
    #[live(0.64)]
    pub scale: f32,
    #[live(MouseCursor::Hand)]
    pub cursor: Option<MouseCursor>,
    #[live]
    pub value: bool,
    // ---- type
    #[live]
    pub check_type: GChooseType,
    // deref -------------------
    #[redraw]
    #[live]
    pub draw_checkbox: DrawGCheckBox,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    // visible -------------------
    #[live(true)]
    pub visible: bool,
    // animator -----------------
    #[live(true)]
    pub animation_open: bool,
    #[animator]
    pub animator: Animator,
    #[live(true)]
    pub grab_key_focus: bool,
}

impl Widget for GCheckBox {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_checkbox.draw_walk(cx, walk);
        DrawStep::done()
    }
    fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
        sweep_area: Area,
    ) {
        let hit = event.hits_with_options(
            cx,
            self.area(),
            HitOptions::new().with_sweep_area(sweep_area),
        );

        self.handle_widget_event(cx, event, scope, hit, sweep_area)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let focus_area = self.area();
        let hit = event.hits(cx, self.area());
        self.handle_widget_event(cx, event, scope, hit, focus_area)
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
    // seems useless
    // fn data_to_widget(&mut self, cx: &mut Cx, nodes: &[LiveNode], path: &[LiveId]) {
    //     if let Some(value) = nodes.read_field_value(path) {
    //         dbg!(value);
    //         if let Some(v) = value.as_bool() {
    //             // set the selected state
    //             // if the value is true, set the selected state to on or off
    //             self.animator_toggle(cx, v, Animate::Yes, id!(selected.on), id!(selected.off))
    //         }
    //     }
    // }
}

impl LiveHook for GCheckBox {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.visible {
            return;
        }
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.get(self.theme, 50);
        let stroke_color = if self.background_visible {
            self.stroke_color.get(self.theme, 50)
        } else {
            vec4(0.0, 0.0, 0.0, 0.0)
        };
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.get(self.theme, 100);
        let stroke_hover_color = self.stroke_hover_color.get(self.theme, 50);
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 600);
        // ------------------ selected color ---------------------------------------------
        let selected_color = self.selected_color.get(self.theme, 500);
        let stroke_selected_color = self.stroke_selected_color.get(self.theme, 50);
        let selected = self.value.to_f32();
        let background_visible = self.background_visible.to_f32();
        // ------------------ apply to draw_checkbox ----------------------------------------
        self.draw_checkbox.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                background_visible: (background_visible),
                hover_color: (hover_color),
                selected_color: (selected_color),
                stroke_color: (stroke_color),
                stroke_hover_color: (stroke_hover_color),
                stroke_selected_color: (stroke_selected_color),
                border_color: (border_color),
                border_width: (self.border_width),
                scale: (self.scale),
                size: (self.size),
                scale: (self.scale),
                selected: (selected)
            },
        );
        self.draw_checkbox.apply_type(self.check_type.clone());

        self.draw_checkbox.redraw(cx);
    }
}

impl GCheckBox {
    widget_area! {
        area, draw_checkbox
    }
    event_option! {
        clicked: GCheckBoxEvent::Clicked => GCheckBoxClickedParam,
        hover: GCheckBoxEvent::Hover => GCheckBoxHoverParam
    }
    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
        self.draw_checkbox.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) -> () {
        self.draw_checkbox.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
    }
    pub fn animate_selected_on(&mut self, cx: &mut Cx) -> () {
        self.value = true;
        self.draw_checkbox.apply_over(
            cx,
            live! {
                selected: 1.0,
            },
        );
    }
    pub fn animate_selected_off(&mut self, cx: &mut Cx) -> () {
        self.value = false;
        self.draw_checkbox.apply_over(
            cx,
            live! {
                selected: 0.0,
            },
        );
    }
    pub fn handle_widget_event(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
        hit: Hit,
        focus_area: Area,
    ) {
        let uid = self.widget_uid();
        if self.animation_open{
            self.animator_handle_event(cx, event);
        }

        match hit {
            Hit::FingerHoverIn(f_in) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                self.animator_play(cx, id!(hover.on));
                cx.widget_action(
                    uid,
                    &scope.path,
                    GCheckBoxEvent::Hover(GCheckBoxHoverParam {
                        value: self.value,
                        e: f_in.clone(),
                    }),
                )
            }
            Hit::FingerHoverOut(_) => {
                cx.set_cursor(MouseCursor::Arrow);
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerDown(_) => {
                if self.grab_key_focus {
                    cx.set_key_focus(focus_area);
                }
            }
            Hit::FingerUp(f_up) => {
                if self.value {
                    self.animator_play(cx, id!(selected.on));
                }
                let state = if self.animator_in_state(cx, id!(selected.on)) {
                    self.value = false;
                    id!(selected.off)
                } else {
                    self.value = true;
                    id!(selected.on)
                };

                self.animator_play(cx, state);
                cx.widget_action(
                    uid,
                    &scope.path,
                    GCheckBoxEvent::Clicked(GCheckBoxClickedParam {
                        value: self.value,
                        e: f_up.clone(),
                    }),
                );
            }
            _ => (),
        }
    }
}

impl GCheckBoxRef {
    ref_event_option! {
        clicked => GCheckBoxClickedParam,
        hover => GCheckBoxHoverParam
    }
    animatie_fn! {
        animate_hover_on,
        animate_hover_off,
        animate_selected_on,
        animate_selected_off
    }
}

impl GCheckBoxSet {
    set_event! {
        clicked => GCheckBoxClickedParam,
        hover => GCheckBoxHoverParam
    }
}
