use makepad_widgets::*;
use shader::draw_text::TextWrap;

use crate::{
    animatie_fn, event_option, ref_event_option, set_event, set_text_and_visible_fn, shader::{
        draw_split::{DrawGSplit, GSplitType},
        draw_text::DrawGText,
    }, themes::Themes, utils::{get_font_family, set_cursor, ThemeColor}, widget_area, widget_origin_fn
};

use super::event::{GBreadCrumbEventItemParam, GBreadCrumbItemEvent};

live_design! {
    import makepad_draw::shader::std::*;
    GLOBAL_DURATION = 0.25,
    GBreadCrumbItemBase = {{GBreadCrumbItem}}{
        spacing: 4.0,
        flow: Right,
        padding: 0,
        text_walk: {
            height: Fit,
            width: Fit
        },
        icon_walk: {
            height: 14.0,
            width: 14.0,
        }
        draw_item: {
            fn pixel(self) -> vec4{
                return vec4(0.0);
            }
        },
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_text: {pressed: 0.0, hover: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                        pressed: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_text: {pressed: 0.0, hover: [{time: 0.0, value: 1.0}],}
                    }
                }

                pressed = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_text: {pressed: [{time: 0.0, value: 1.0}], hover: 1.0,}
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct GBreadCrumbItem {
    #[live(Themes::Dark)]
    pub theme: Themes,
    // text -------------------
    #[live]
    pub color: Option<Vec4>,
    #[live]
    pub text_hover_color: Option<Vec4>,
    #[live]
    pub text_pressed_color: Option<Vec4>,
    #[live(9.0)]
    pub font_size: f64,
    // #[live(1.0)]
    // pub brightness: f32,
    // #[live(0.5)]
    // pub curve: f32,
    // #[live(1.5)]
    // pub line_spacing: f64,
    #[live(0.0)]
    pub top_drop: f64,
    #[live(0.0)]
    pub height_factor: f64,
    #[live(TextWrap::Line)]
    pub wrap: TextWrap,
    #[live]
    pub font_family: LiveDependency,
    #[live]
    pub text: ArcStringMut,
    #[live]
    pub text_walk: Walk,
    // icon -------------------
    #[live(1.0)]
    pub icon_brightness: f32,
    #[live(0.6)]
    pub icon_curve: f32,
    #[live(0.5)]
    pub icon_linearize: f32,
    #[live(1.0)]
    pub icon_scale: f64,
    #[live]
    pub icon_color: Option<Vec4>,
    #[live(1.0)]
    pub icon_draw_depth: f32,
    #[live]
    pub icon_walk: Walk,
    #[live(GSplitType::Spliter)]
    pub split_type: GSplitType,
    // deref -------------------
    #[live]
    pub draw_text: DrawGText,
    #[live]
    pub draw_split: DrawGSplit,
    #[redraw]
    #[live]
    pub draw_item: DrawQuad,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[live(true)]
    pub grab_key_focus: bool,
    // animator -----------------
    #[live(true)]
    pub animation_open: bool,
    #[animator]
    pub animator: Animator,
    #[live(true)]
    pub visible: bool,
    #[live(Some(MouseCursor::Hand))]
    pub cursor: Option<MouseCursor>,
}

impl Widget for GBreadCrumbItem {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        self.draw_item.begin(cx, walk, self.layout);

        self.draw_split.draw_walk(cx, self.icon_walk);
        let font = get_font_family(&self.font_family, cx);

        self.draw_text.text_style.font = font;
        self.draw_text
            .draw_walk(cx, self.text_walk, Align::default(), self.text.as_ref());

        self.draw_item.end(cx);
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

impl LiveHook for GBreadCrumbItem {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.visible {
            return;
        }
        // ------------------ font ------------------------------------------------------
        let font_color = self.color.get(self.theme, 100);
        let text_hover_color = self.text_hover_color.get(self.theme, 400);
        let text_pressed_color = self.text_pressed_color.get(self.theme, 200);
        // ------------------icon color -----------------------------------------------
        let icon_color = self.icon_color.get(self.theme, 100);

        self.draw_split.apply_over(
            cx,
            live! {
                brightness: (self.icon_brightness),
                color: (icon_color),
                curve: (self.icon_curve),
                draw_depth: (self.icon_draw_depth),
                linearize: (self.icon_linearize),
            },
        );
        self.draw_split.apply_split_type(self.split_type.clone());

        self.draw_text.apply_over(
            cx,
            live! {
                color: (font_color),
                hover_color: (text_hover_color),
                pressed_color: (text_pressed_color),
                text_style: {
                    // brightness: (self.brightness),
                    // curve: (self.curve),
                    line_spacing: (self.layout.line_spacing),
                    top_drop: (self.top_drop),
                    font_size: (self.font_size),
                    height_factor: (self.height_factor),
                }
            },
        );
        self.draw_text.wrap = self.wrap.clone();
        self.draw_text.redraw(cx);
        self.draw_split.redraw(cx);
    }
}

impl GBreadCrumbItem {
    widget_area!{
        area, draw_item
    }
    event_option!{
        clicked : GBreadCrumbItemEvent::Clicked => GBreadCrumbEventItemParam,
        hover : GBreadCrumbItemEvent::Hover => GBreadCrumbEventItemParam
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

        if self.animation_open {
            if self.animator_handle_event(cx, event).must_redraw() {
                self.draw_item.redraw(cx);
            }
        }
        match hit {
            Hit::FingerDown(_) => {
                if self.grab_key_focus {
                    cx.set_key_focus(focus_area);
                }
                self.animator_play(cx, id!(hover.pressed));
            }
            Hit::FingerHoverIn(h) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                self.animator_play(cx, id!(hover.on));
                cx.widget_action(
                    uid,
                    &scope.path,
                    GBreadCrumbItemEvent::Hover(GBreadCrumbEventItemParam {
                        item: self.text(),
                        key_modifiers: h.modifiers,
                    }),
                );
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerUp(f_up) => {
                if f_up.is_over {
                    cx.widget_action(
                        uid,
                        &scope.path,
                        GBreadCrumbItemEvent::Clicked(GBreadCrumbEventItemParam {
                            item: self.text(),
                            key_modifiers: f_up.modifiers,
                        }),
                    );

                    if f_up.device.has_hovers() {
                        self.animator_play(cx, id!(hover.on));
                    } else {
                        self.animator_play(cx, id!(hover.off));
                    }
                } else {
                    self.animator_play(cx, id!(hover.off));
                }
            }
            _ => (),
        }
    }
    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 1.0,
                pressed: 0.0
            },
        );
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) -> () {
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 0.0,
                pressed: 0.0
            },
        );
    }
    pub fn animate_pressed(&mut self, cx: &mut Cx) -> () {
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 1.0,
                pressed: 1.0
            },
        );
    }
}

impl GBreadCrumbItemRef {
    widget_origin_fn!(GBreadCrumbItem);
    ref_event_option!{
        clicked => GBreadCrumbEventItemParam,
        hover => GBreadCrumbEventItemParam
    }
    animatie_fn!{
        animate_hover_on,
        animate_hover_off,
        animate_pressed
    }
}

impl GBreadCrumbItemSet {
    set_event!{
       clicked,
       hover
    }
}
