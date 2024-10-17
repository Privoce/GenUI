pub mod event;
pub mod group;
mod register;

use event::{GRadioClickedParam, GRadioEvent, GRadioHoverParam};
pub use register::register;

use makepad_widgets::*;
use shader::draw_text::TextWrap;

use crate::{
    animatie_fn, event_option, ref_event_option, set_event, set_text_and_visible_fn,
    shader::{
        draw_radio::{DrawGRadio, GChooseType},
        draw_text::DrawGText,
    },
    themes::Themes,
    utils::{get_font_family, set_cursor, BoolToF32, ThemeColor},
    widget_area, widget_origin_fn,
};

live_design! {
    import makepad_draw::shader::std::*;
    GLOBAL_DURATION = 0.25

    GRadioBase = {{GRadio}}{
        height: 18.0,
        width: Fit,
        draw_radio_wrap: {
            fn pixel(self) ->vec4{
                return vec4(0.0);
            }
        },
        font_size: 10.0,
        spacing: 6.0,
        align: {
            x: 0.0,
            y: 0.5
        },
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
    #[live]
    pub color: Option<Vec4>,
    #[live]
    pub text_hover_color: Option<Vec4>,
    #[live]
    pub text_focus_color: Option<Vec4>,
    #[live(12.0)]
    pub font_size: f64,
    // #[live(0.0)]
    // pub top_drop: f64,
    #[live(0.0)]
    pub height_factor: f64,
    #[live(TextWrap::Word)]
    pub wrap: TextWrap,
    #[live]
    pub font_family: LiveDependency,
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
    #[live(0.48)]
    pub scale: f32,
    #[live(MouseCursor::Hand)]
    pub cursor: Option<MouseCursor>,
    // value --------------------
    #[live(None)]
    pub value: Option<String>,
    // selected ------------------
    #[live(false)]
    pub selected: bool,
    #[live]
    pub text: ArcStringMut,
    // ---- type
    #[live]
    pub radio_type: GChooseType,
    // deref -------------------
    #[redraw]
    #[live]
    pub draw_radio: DrawGRadio,
    #[redraw]
    #[live]
    pub draw_text: DrawGText,
    #[redraw]
    #[live]
    pub draw_radio_wrap: DrawQuad,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    // visible -------------------
    #[live(true)]
    pub visible: bool,
    // animator -----------------
    #[live(true)]
    pub animation_key: bool,
    #[animator]
    pub animator: Animator,
    #[live(true)]
    pub grab_key_focus: bool,
    #[live(true)]
    pub event_key: bool,
}

impl Widget for GRadio {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let font = get_font_family(&self.font_family, cx);

        self.draw_text.text_style.font = font;
        self.draw_radio_wrap.begin(cx, walk, self.layout);
        let size = self.size + self.border_width;
        let radio_walk = Walk {
            width: Size::Fixed((size * 2.0) as f64),
            height: Size::Fixed((size * 2.0) as f64),
            ..Default::default()
        };
        self.draw_radio.draw_walk(cx, radio_walk);
        let text_walk = Walk {
            width: Size::Fit,
            height: Size::Fit,
            ..Default::default()
        };
        self.draw_text
            .draw_walk(cx, text_walk, Align { x: 0.0, y: 0.0 }, self.text.as_ref());
        self.draw_radio_wrap.end(cx);
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
    set_text_and_visible_fn!();
}

impl LiveHook for GRadio {
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
        let color = self.color.get(self.theme, 50);
        let text_hover_color = self.text_hover_color.get(self.theme, 25);
        let text_focus_color = self.text_focus_color.get(self.theme, 100);
        let hover_color = self.hover_color.get(self.theme, 100);
        let stroke_hover_color = self.stroke_hover_color.get(self.theme, 50);
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 600);
        // ------------------ selected color ---------------------------------------------
        let selected_color = self.selected_color.get(self.theme, 500);
        let stroke_selected_color = self.stroke_selected_color.get(self.theme, 50);
        let selected = self.selected.to_f32();
        let background_visible = self.background_visible.to_f32();
        // ------------------ apply to draw_radio ----------------------------------------
        self.draw_radio.apply_over(
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
        self.draw_radio.apply_type(self.radio_type.clone());
        self.draw_text.apply_over(
            cx,
            live! {
                color: (color),
                stroke_hover_color: (text_hover_color),
                stroke_focus_color: (text_focus_color),
                text_style: {
                    // top_drop: (self.top_drop),
                    font_size: (self.font_size),
                    height_factor: (self.height_factor),
                }
            },
        );
        self.draw_text.wrap = self.wrap.clone();
        self.draw_radio.redraw(cx);
        self.draw_text.redraw(cx);
    }
}

impl GRadio {
    widget_area! {
        area, draw_radio_wrap,
        area_radio, draw_radio,
        area_text, draw_text
    }
    event_option! {
        clicked: GRadioEvent::Clicked => GRadioClickedParam,
        hover: GRadioEvent::Hover => GRadioHoverParam
    }
    pub fn toggle(&mut self, cx: &mut Cx, selected: bool) -> () {
        self.selected = selected;
        self.draw_radio.selected = selected.to_f32();
        if selected {
            self.animator_play(cx, id!(selected.on));
        } else {
            self.animator_play(cx, id!(selected.off));
        }
    }
    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
        self.draw_radio.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) -> () {
        self.draw_radio.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
    }
    pub fn animate_selected_on(&mut self, cx: &mut Cx) -> () {
        self.selected = true;
        self.draw_radio.apply_over(
            cx,
            live! {
                selected: 1.0,
            },
        );
    }
    pub fn animate_selected_off(&mut self, cx: &mut Cx) -> () {
        self.selected = false;
        self.draw_radio.apply_over(
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
        if self.animation_key {
            self.animator_handle_event(cx, event);
        }

        match hit {
            Hit::FingerHoverIn(f_in) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                self.animator_play(cx, id!(hover.on));
                cx.widget_action(
                    uid,
                    &scope.path,
                    GRadioEvent::Hover(GRadioHoverParam {
                        selected: self.selected,
                        value: self.value.clone(),
                        e: f_in.clone(),
                    }),
                )
            }
            Hit::FingerHoverOut(_) => {
                cx.set_cursor(MouseCursor::Arrow);
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerDown(_fe) => {
                if self.grab_key_focus {
                    cx.set_key_focus(focus_area);
                }
            }
            Hit::FingerUp(f_up) => {
                // if self.selected{
                //     self.animator_play(cx, id!(selected.on));
                // }
                if self.animator_in_state(cx, id!(selected.off)) {
                    self.selected = true;
                    self.animator_play(cx, id!(selected.on));
                    cx.widget_action(
                        uid,
                        &scope.path,
                        GRadioEvent::Clicked(GRadioClickedParam {
                            selected: self.selected,
                            e: f_up.clone(),
                            value: self.value.clone(),
                        }),
                    );
                }
            }
            _ => (),
        }
    }
}

impl GRadioRef {
    ref_event_option! {
        clicked => GRadioClickedParam,
        hover => GRadioHoverParam
    }
    animatie_fn! {
        animate_hover_on,
        animate_hover_off,
        animate_selected_on,
        animate_selected_off
    }
    widget_origin_fn!(GRadio);
}

impl GRadioSet {
    set_event! {
        clicked => GRadioClickedParam,
        hover => GRadioHoverParam
    }
}
