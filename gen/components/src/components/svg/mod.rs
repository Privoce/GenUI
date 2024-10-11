pub mod event;
mod register;

use event::{GSvgEvent, GSvgEventParam};
pub use register::register;

use makepad_widgets::*;

use crate::{
    animatie_fn, event_option, ref_event_option, set_event,
    shader::draw_svg::DrawGSvg,
    themes::Themes,
    utils::{set_cursor, ThemeColor, ToPath},
    widget_area,
};

live_design! {
    import makepad_draw::shader::std::*;
    GLOBAL_DURATION = 0.25

    GSvgBase = {{GSvg}}{
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_svg: {hover: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_svg: { hover: [{time: 0.0, value: 1.0}],}
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct GSvg {
    #[live]
    pub theme: Themes,
    #[live(1.0)]
    pub brightness: f32,
    #[live(0.6)]
    pub curve: f32,
    #[live(0.5)]
    pub linearize: f32,
    #[live]
    pub src: LiveDependency,
    /// svg path command (todo!)
    // #[live]
    // pub command: Option<String>,
    #[live(1.0)]
    pub scale: f64,
    #[live]
    pub color: Option<Vec4>,
    #[live(1.0)]
    pub draw_depth: f32,
    #[live]
    pub stroke_hover_color: Option<Vec4>,
    #[live]
    pub cursor: Option<MouseCursor>,
    #[live(true)]
    pub grab_key_focus: bool,
    // visible -------------------
    #[live(true)]
    pub visible: bool,
    // animator -----------------
    #[live(false)]
    pub animation_key: bool,
    #[animator]
    animator: Animator,
    // deref -----------------
    #[redraw]
    #[live]
    pub draw_svg: DrawGSvg,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[live(true)]
    pub event_key: bool,
}

impl Widget for GSvg {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        self.draw_svg.draw_walk(cx, walk);

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
}

impl LiveHook for GSvg {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.visible {
            return;
        }
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.stroke_hover_color.get(self.theme, 25);
        // ------------------ color -----------------------------------------------
        let color = self.color.get(self.theme, 25);

        self.draw_svg.apply_over(
            cx,
            live! {
                stroke_hover_color: (hover_color),
                color: (color),
                brightness: (self.brightness),
                curve: (self.curve),
                linearize: (self.linearize),
                scale: (self.scale),
                draw_depth: (self.draw_depth),
            },
        );

        self.draw_svg.set_src(self.src.clone());

        // self.draw_svg.redraw(cx);
    }
}

impl GSvg {
    widget_area! {
        area, draw_svg
    }
    event_option! {
        clicked: GSvgEvent::Clicked => GSvgEventParam,
        hover: GSvgEvent::Hover => GSvgEventParam
    }
    pub fn redraw(&self, cx: &mut Cx) ->(){
        self.draw_svg.redraw(cx);
    }
    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
        self.draw_svg.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) -> () {
        self.draw_svg.apply_over(
            cx,
            live! {
                hover: 0.0,
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
            if self.animator_handle_event(cx, event).must_redraw() {
                self.draw_svg.redraw(cx);
            }
        }

        match hit {
            Hit::FingerDown(_) => {
                if self.grab_key_focus {
                    cx.set_key_focus(focus_area);
                }
            }
            Hit::FingerHoverIn(f_in) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                self.animator_play(cx, id!(hover.on));
                cx.widget_action(
                    uid,
                    &scope.path,
                    GSvgEvent::Hover(GSvgEventParam {
                        src: self.src.to_pathbuf(),
                        key_modifiers: f_in.modifiers,
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
                        GSvgEvent::Clicked(GSvgEventParam {
                            src: self.src.to_pathbuf(),
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
}

impl GSvgRef {
    ref_event_option! {
        clicked => GSvgEventParam,
        hover => GSvgEventParam
    }
    animatie_fn! {
        animate_hover_on,
        animate_hover_off
    }
}

impl GSvgSet {
    set_event! {
        clicked => GSvgEventParam,
        hover => GSvgEventParam
    }
}
