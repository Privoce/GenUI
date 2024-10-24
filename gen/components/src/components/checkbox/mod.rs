pub mod event;
pub mod group;
mod register;

use event::*;
use makepad_widgets::*;
pub use register::register;
use shader::draw_text::TextWrap;

use crate::{
    active_event, animatie_fn, default_handle_animation, default_hit_hover_in, default_hit_hover_out, event_option, play_animation, ref_area, ref_area_ext, ref_event_option, ref_redraw, ref_render, set_event, set_scope_path, set_text_and_visible_fn, shader::{
        draw_check_box::DrawGCheckBox, draw_radio::GChooseType, draw_text::DrawGText,
        draw_view::DrawGView,
    }, themes::Themes, utils::{get_font_family, set_cursor, BoolToF32, ThemeColor}, widget_area
};

live_design! {
    import makepad_draw::shader::std::*;
    GLOBAL_DURATION = 0.25

    GCheckBoxBase = {{GCheckBox}}{
        height: Fit,
        width: Fit,
        font_size: 10.0,
        spacing: 8.0,
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
                        draw_checkbox: {hover: 0.0},
                        draw_checkbox_wrap: {hover: 0.0},
                        draw_text: {hover: 0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_checkbox: {hover: 1.0},
                        draw_checkbox_wrap: {hover: 1.0},
                        draw_text: {hover: 1.0}
                    }
                }
            }
            selected = {
                default: off
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_checkbox: {selected: 0.0},
                        draw_checkbox_wrap: {focus: 0.0},
                        draw_text: {focus: 0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_checkbox: {selected: 1.0},
                        draw_checkbox_wrap: {focus: 1.0},
                        draw_text: {focus: 1.0}
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
    // text ----------------------
    #[live]
    pub color: Option<Vec4>,
    #[live]
    pub text_hover_color: Option<Vec4>,
    #[live]
    pub text_focus_color: Option<Vec4>,
    #[live(12.0)]
    pub font_size: f64,
    #[live(0.0)]
    pub height_factor: f64,
    #[live(TextWrap::Word)]
    pub wrap: TextWrap,
    #[live]
    pub font_family: LiveDependency,
    #[live(true)]
    pub text_visible: bool,
    // checkbox ---------------------
    #[live(8.0)]
    pub size: f32,
    #[live]
    pub checkbox_background_color: Option<Vec4>,
    #[live(true)]
    pub checkbox_background_visible: bool,
    #[live]
    pub checkbox_hover_color: Option<Vec4>,
    #[live]
    pub checkbox_selected_color: Option<Vec4>,
    #[live]
    pub stroke_color: Option<Vec4>,
    #[live]
    pub stroke_hover_color: Option<Vec4>,
    #[live]
    pub stroke_selected_color: Option<Vec4>,
    #[live]
    pub checkbox_border_color: Option<Vec4>,
    #[live(1.0)]
    pub checkbox_border_width: f32,
    #[live(0.48)]
    pub scale: f32,
    // checkbox_wrap ----------------
    #[live]
    pub background_color: Option<Vec4>,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub focus_color: Option<Vec4>,
    #[live]
    pub shadow_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live]
    pub background_visible: bool,
    #[live(0.0)]
    pub border_width: f32,
    #[live(0.0)]
    pub border_radius: f32,
    #[live(0.0)]
    pub spread_radius: f32,
    #[live(4.8)]
    pub blur_radius: f32,
    #[live]
    pub shadow_offset: Vec2,
    #[live(MouseCursor::Hand)]
    pub cursor: Option<MouseCursor>,
    // value -----------------------
    #[live(None)]
    pub value: Option<String>,
    // selected ------------------
    #[live(false)]
    pub selected: bool,
    #[live]
    pub text: ArcStringMut,
    // ---- type
    #[live]
    pub checkbox_type: GChooseType,
    // deref -------------------
    #[redraw]
    #[live]
    pub draw_checkbox: DrawGCheckBox,
    #[redraw]
    #[live]
    pub draw_text: DrawGText,
    #[redraw]
    #[live]
    pub draw_checkbox_wrap: DrawGView,
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
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
}

impl Widget for GCheckBox {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        self.set_scope_path(&scope.path);

        self.draw_checkbox_wrap.begin(cx, walk, self.layout);
        let size = self.size + self.checkbox_border_width;
        let checkbox_walk = Walk {
            width: Size::Fixed((size * 2.0) as f64),
            height: Size::Fixed((size * 2.0) as f64),
            ..Default::default()
        };
        self.draw_checkbox.draw_walk(cx, checkbox_walk);

        if self.text_visible {
            let font = get_font_family(&self.font_family, cx);
            self.draw_text.text_style.font = font;
            let text_walk = Walk {
                width: Size::Fit,
                height: Size::Fit,
                ..Default::default()
            };
            self.draw_text
                .draw_walk(cx, text_walk, Align { x: 0.0, y: 0.0 }, self.text.as_ref());
        }

        self.draw_checkbox_wrap.end(cx);
        DrawStep::done()
    }
    fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
        sweep_area: Area,
    ) {
        if !self.is_visible() {
            return;
        }
        let hit = event.hits_with_options(
            cx,
            self.area(),
            HitOptions::new().with_sweep_area(sweep_area),
        );

        self.handle_widget_event(cx, event, scope, hit, sweep_area)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.is_visible() {
            return;
        }
        let focus_area = self.area();
        let hit = event.hits(cx, self.area());
        self.handle_widget_event(cx, event, scope, hit, focus_area)
    }
    set_text_and_visible_fn!();
}

impl LiveHook for GCheckBox {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.visible {
            return;
        }
        self.render(cx);
    }
}

impl GCheckBox {
    set_scope_path!();
    play_animation!();
    widget_area! {
        area, draw_checkbox_wrap,
        area_checkbox, draw_checkbox,
        area_text, draw_text
    }
    event_option! {
        clicked: GCheckBoxEvent::Clicked => GCheckBoxClickedParam,
        hover_in: GCheckBoxEvent::HoverIn => GCheckBoxHoverParam,
        hover_out: GCheckBoxEvent::HoverOut => GCheckBoxHoverParam
    }
    active_event! {
        active_hover_in: GCheckBoxEvent::HoverIn |e: Option<FingerHoverEvent>| => GCheckBoxHoverParam {e},
        active_hover_out: GCheckBoxEvent::HoverOut |e: Option<FingerHoverEvent>| => GCheckBoxHoverParam {e}
    }
    fn check_event_scope(&self) -> Option<&HeapLiveIdPath> {
        self.event_key.then(|| self.scope_path.as_ref()).flatten()
    }
    fn active_clicked(&mut self, cx: &mut Cx, e: Option<FingerUpEvent>) {
        self.check_event_scope().map(|path| {
            cx.widget_action(
                self.widget_uid(),
                path,
                GCheckBoxEvent::Clicked(GCheckBoxClickedParam {
                    value: self.value.clone(),
                    selected: self.selected,
                    e,
                }),
            );
        });
    }
    pub fn clear_animation(&mut self, cx: &mut Cx) {
        self.draw_checkbox_wrap.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
        self.draw_checkbox.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
    }
    pub fn render(&mut self, cx: &mut Cx) -> () {
        // ----------------- checkbox -----------------------------------------------------
        let checkbox_background_color = self.checkbox_background_color.get(self.theme, 50);
        let checkbox_hover_color = self.checkbox_hover_color.get(self.theme, 100);
        let checkbox_selected_color = self.checkbox_selected_color.get(self.theme, 500);
        let checkbox_border_color = self.checkbox_border_color.get(self.theme, 600);
        let stroke_color = if self.checkbox_background_visible {
            self.stroke_color.get(self.theme, 50)
        } else {
            vec4(0.0, 0.0, 0.0, 0.0)
        };
        let stroke_hover_color = self.stroke_hover_color.get(self.theme, 50);
        let stroke_selected_color = self.stroke_selected_color.get(self.theme, 50);
        let checkbox_background_visible = self.checkbox_background_visible.to_f32();
        // ----------------- checkbox_wrap ------------------------------------------------
        let background_color = self.background_color.get(self.theme, 500);
        let hover_color = self.hover_color.get(self.theme, 400);
        let focus_color = self.focus_color.get(self.theme, 600);
        let border_color = self.border_color.get(self.theme, 600);
        let shadow_color = self.shadow_color.get(self.theme, 700);
        let background_visible = self.background_visible.to_f32();
        // ----------------- text ------------------------------------------------------
        let color = self.color.get(self.theme, 50);
        let text_hover_color = self.text_hover_color.get(self.theme, 25);
        let text_focus_color = self.text_focus_color.get(self.theme, 100);
        // selected --------------------------------------------------------------------
        let selected = self.selected.to_f32();
        // ------------------ apply to draw_checkbox ---------------------------------------
        self.draw_checkbox_wrap.apply_over(
            cx,
            live! {
                background_color: (background_color),
                background_visible: (background_visible),
                border_color: (border_color),
                border_width: (self.border_width),
                border_radius: (self.border_radius),
                focus_color: (focus_color),
                hover_color: (hover_color),
                shadow_color: (shadow_color),
                shadow_offset: (self.shadow_offset),
                spread_radius: (self.spread_radius),
                blur_radius: (self.blur_radius)
            },
        );
        self.draw_checkbox.apply_over(
            cx,
            live! {
                background_color: (checkbox_background_color),
                background_visible: (checkbox_background_visible),
                hover_color: (checkbox_hover_color),
                selected_color: (checkbox_selected_color),
                stroke_color: (stroke_color),
                stroke_hover_color: (stroke_hover_color),
                stroke_selected_color: (stroke_selected_color),
                border_color: (checkbox_border_color),
                border_width: (self.checkbox_border_width),
                scale: (self.scale),
                size: (self.size),
                scale: (self.scale),
                selected: (selected)
            },
        );
        self.draw_checkbox.apply_type(self.checkbox_type.clone());
        if self.text_visible {
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
        }
    }
    pub fn toggle(&mut self, cx: &mut Cx, selected: bool) -> () {
        self.selected = selected;
        self.draw_checkbox.selected = selected.to_f32();
        if selected {
            self.play_animation(cx, id!(selected.on));
        } else {
            self.play_animation(cx, id!(selected.off));
        }
    }
    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.draw_checkbox.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
        self.draw_checkbox_wrap.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
        if self.text_visible {
            self.draw_text.apply_over(
                cx,
                live! {
                    hover: 1.0,
                },
            );
        }
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) -> () {
        self.draw_checkbox.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
        self.draw_checkbox_wrap.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
        if self.text_visible {
            self.draw_text.apply_over(
                cx,
                live! {
                    hover: 0.0,
                },
            );
        }
    }
    pub fn animate_selected_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.selected = true;
        self.draw_checkbox.apply_over(
            cx,
            live! {
                selected: 1.0,
            },
        );
        self.draw_checkbox_wrap.apply_over(
            cx,
            live! {
                focus: 1.0,
            },
        );
        if self.text_visible {
            self.draw_text.apply_over(
                cx,
                live! {
                    focus: 1.0,
                },
            );
        }
    }
    pub fn animate_selected_off(&mut self, cx: &mut Cx) -> () {
        self.selected = false;
        self.draw_checkbox.apply_over(
            cx,
            live! {
                selected: 0.0,
            },
        );
        self.draw_checkbox_wrap.apply_over(
            cx,
            live! {
                focus: 0.0,
            },
        );
        if self.text_visible {
            self.draw_text.apply_over(
                cx,
                live! {
                    focus: 0.0,
                },
            );
        }
    }
    pub fn redraw(&self, cx: &mut Cx) -> () {
        self.draw_text.redraw(cx);
        self.draw_checkbox.redraw(cx);
        self.draw_checkbox_wrap.redraw(cx);
    }
    pub fn handle_widget_event(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        _scope: &mut Scope,
        hit: Hit,
        focus_area: Area,
    ) {
        default_handle_animation!(self, cx, event);

        match hit {
            Hit::FingerHoverIn(e) => {
                default_hit_hover_in!(self, cx, Some(e));
            }
            Hit::FingerHoverOut(e) => {
                default_hit_hover_out!(self, cx, Some(e));
            }
            Hit::FingerDown(_) => {
                if self.grab_key_focus {
                    cx.set_key_focus(focus_area);
                }
            }
            Hit::FingerUp(e) => {
                if self.selected {
                    self.play_animation(cx, id!(selected.on));
                }
                let state = if self.animator_in_state(cx, id!(selected.on)) {
                    self.selected = false;
                    id!(selected.off)
                } else {
                    self.selected = true;
                    id!(selected.on)
                };

                self.play_animation(cx, state);
                self.active_clicked(cx, Some(e));
            }
            _ => (),
        }
    }
    pub fn value(&self) -> Option<String> {
        self.value.clone()
    }
    pub fn is_selected(&self) -> bool {
        self.selected
    }
}

impl GCheckBoxRef {
    ref_area!();
    ref_area_ext! {
        area_checkbox,
        area_text
    }
    ref_redraw!();
    ref_render!();
    ref_event_option! {
        clicked => GCheckBoxClickedParam,
        hover_in => GCheckBoxHoverParam,
        hover_out => GCheckBoxHoverParam
    }
    animatie_fn! {
        animate_hover_on,
        animate_hover_off,
        animate_selected_on,
        animate_selected_off
    }
    /// ## Get the value of the checkbox.
    /// If the checkbox has a value, it will return the Some(value).
    /// Otherwise, it will return None.(include can not find the checkbox)
    pub fn value(&self) -> Option<String> {
        if let Some(c_ref) = self.borrow() {
            c_ref.value()
        } else {
            None
        }
    }
    /// ## Get the selected state of the checkbox.
    /// If the checkbox is selected, it will return true.
    /// Otherwise, it will return false.(include can not find the checkbox)
    pub fn is_selected(&self) -> bool {
        if let Some(c_ref) = self.borrow() {
            c_ref.is_selected()
        } else {
            false
        }
    }
}

impl GCheckBoxSet {
    set_event! {
        clicked => GCheckBoxClickedParam,
        hover_in => GCheckBoxHoverParam,
        hover_out => GCheckBoxHoverParam
    }
}
