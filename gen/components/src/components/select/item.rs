use makepad_widgets::*;

use crate::{
    shader::{draw_card::DrawGCard, draw_text::DrawGText},
    utils::{get_font_family, ToBool},
};

use super::{GSelectItemClickedParam, GSelectItemEvent};

live_design! {
    GSelectItemBase = {{GSelectItem}} {
        width: Fill,
        height: 36.0,
        padding: {left: 8.0, right: 8.0},
        animator: {
            hover = {
                default: off
                off = {
                    from: {all: Snap}
                    apply: {
                        draw_item: {hover: 0.0}
                        draw_name: {hover: 0.0}
                    }
                }
                on = {
                    cursor: Hand
                    from: {all: Snap}
                    apply: {
                        draw_item: {hover: 1.0}
                        draw_name: {hover: 1.0}
                    }
                }
            }

            select = {
                default: off
                off = {
                    from: {all: Snap}
                    apply: {
                        draw_item: {pressed: 0.0,}
                        draw_name: {pressed: 0.0,}
                    }
                }
                on = {
                    from: {all: Snap}
                    apply: {
                        draw_item: {pressed: 1.0,}
                        draw_name: {pressed: 1.0,}
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveRegister)]
pub struct GSelectItem {
    #[live]
    pub draw_item: DrawGCard,
    #[live]
    pub draw_text: DrawGText,
    #[layout]
    pub layout: Layout,
    #[walk]
    pub walk: Walk,
    #[live]
    pub hover: f32,
    #[live]
    pub selected: f32,
    #[animator]
    pub animator: Animator,
    #[live]
    pub text: String,
    #[live]
    pub value: String,
    #[live]
    pub font_family: LiveDependency,
}

impl LiveHook for GSelectItem {}

impl GSelectItem {
    pub fn area(&self) -> Area {
        self.draw_item.area()
    }
    pub fn draw_item(&mut self, cx: &mut Cx2d, text: &str,value: &str) {
        let _ = self.draw_item.begin(cx, self.walk, self.layout);
        let font = get_font_family(&self.font_family, cx);
        self.draw_text.text_style.font = font;
        let _ = self
            .draw_text
            .draw_walk(cx, Walk::fit(), Align { x: 0.0, y: 0.5 }, text);
        self.value = value.to_string();
        let _ = self.draw_item.end(cx);
    }
    pub fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        sweep_area: Area,

        dispatch_action: &mut dyn FnMut(&mut Cx, GSelectItemEvent),
    ) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.draw_item.area().redraw(cx);
        }
        match event.hits_with_options(
            cx,
            self.area(),
            HitOptions::new().with_sweep_area(sweep_area),
        ) {
            Hit::FingerHoverIn(_) => {
                self.animator_play(cx, id!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerDown(_) => {
                self.animator_play(cx, id!(hover.on));
                self.animator_play(cx, id!(select.on));
            }
            Hit::FingerUp(se) => {
                if !se.is_sweep {
                    dispatch_action(
                        cx,
                        GSelectItemEvent::Clicked(GSelectItemClickedParam {
                            selected: self.selected.to_bool(),
                            e: se.clone(),
                            text: self.text.to_string(),
                            value: self.value.to_string(),
                            
                        }),
                    );
                } else {
                    self.animator_play(cx, id!(hover.off));
                    self.animator_play(cx, id!(select.off));
                }
            }
            _ => {}
        }
    }
}
