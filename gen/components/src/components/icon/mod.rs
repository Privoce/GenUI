mod register;

use makepad_widgets::*;
pub use register::register;

use crate::{
    animatie_fn, event_option, ref_event_option, set_event, shader::icon_lib::{
        arrow::DrawGIconArrow,
        base::DrawGIconBase,
        code::DrawGIconCode,
        emoji::DrawGIconEmoji,
        fs::DrawGIconFs,
        person::DrawGIconPerson,
        relation::DrawGIconRelation,
        state::DrawGIconState,
        time::DrawGIconTime,
        tool::DrawGIconTool,
        types::{
            arrow::Arrow, base::Base, code::Code, emoji::Emoji, fs::Fs, person::Person,
            relation::Relation, state::State, time::Time, tool::Tool, ui::UI, DrawGIconType,
            IconType,
        },
        ui::DrawGIconUI,
    }, themes::Themes, utils::{set_cursor, ThemeColor}, widget_area
};

live_design! {
    import makepad_draw::shader::std::*;
    GLOBAL_DURATION = 0.25
    GIconBase = {{GIcon}}{
        draw_icon: {
            instance hover: 0.0,
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
                        draw_icon: {hover: 0.0},
                        icon_base: {hover: 0.0},
                        icon_arrow: {hover: 0.0},
                        icon_code: {hover: 0.0},
                        icon_emoji: {hover: 0.0},
                        icon_fs: {hover: 0.0},
                        icon_ui: {hover: 0.0},
                        icon_person: {hover: 0.0},
                        icon_relation: {hover: 0.0},
                        icon_state: {hover: 0.0},
                        icon_time: {hover: 0.0},
                        icon_tool: {hover: 0.0},
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                        pressed: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_icon: {hover: [{time: 0.0, value: 1.0}],}
                        icon_base: {hover: 1.0},
                        icon_arrow: {hover: 1.0},
                        icon_code: {hover: 1.0},
                        icon_emoji: {hover: 1.0},
                        icon_fs: {hover: 1.0},
                        icon_ui: {hover: 1.0},
                        icon_person: {hover: 1.0},
                        icon_relation: {hover: 1.0},
                        icon_state: {hover: 1.0},
                        icon_time: {hover: 1.0},
                        icon_tool: {hover: 1.0},
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct GIcon {
    #[live]
    pub theme: Themes,
    #[live]
    pub stroke_color: Option<Vec4>,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live(1.0)]
    pub stroke_width: f32,
    #[live]
    pub cursor: Option<MouseCursor>,
    #[live(true)]
    pub visible: bool,
    #[live(true)]
    pub grab_key_focus: bool,
    #[live(false)]
    pub animation_open: bool,
    #[animator]
    pub animator: Animator,
    // redraw -------------------------------------
    #[redraw]
    #[live]
    pub draw_icon: DrawQuad,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    // icon lib draw shader ------------------------
    #[live]
    icon_base: Option<DrawGIconBase>,
    #[live]
    icon_arrow: Option<DrawGIconArrow>,
    #[live]
    icon_code: Option<DrawGIconCode>,
    #[live]
    icon_emoji: Option<DrawGIconEmoji>,
    #[live]
    icon_fs: Option<DrawGIconFs>,
    #[live]
    icon_ui: Option<DrawGIconUI>,
    #[live]
    icon_person: Option<DrawGIconPerson>,
    #[live]
    icon_relation: Option<DrawGIconRelation>,
    #[live]
    icon_state: Option<DrawGIconState>,
    #[live]
    icon_time: Option<DrawGIconTime>,
    #[live]
    icon_tool: Option<DrawGIconTool>,
    // draw icon type ------------------------------
    #[rust]
    pub draw_type: Option<DrawGIconType>,
    // icon type -----------------------------------
    #[live]
    pub icon_type: IconType,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum GIconEvent {
    Hover(KeyModifiers),
    Clicked(KeyModifiers),
    None,
}

impl Widget for GIcon {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        self.draw_icon.begin(cx, walk, self.layout);
        match self.draw_type.as_ref().unwrap() {
            crate::shader::icon_lib::types::DrawGIconType::Base => {
                self.icon_base
                    .as_mut()
                    .unwrap()
                    .begin(cx, walk, self.layout);
                self.icon_base.as_mut().unwrap().end(cx);
            }
            crate::shader::icon_lib::types::DrawGIconType::Code => {
                self.icon_code
                    .as_mut()
                    .unwrap()
                    .begin(cx, walk, self.layout);
                self.icon_code.as_mut().unwrap().end(cx);
            }
            crate::shader::icon_lib::types::DrawGIconType::Arrow => {
                self.icon_arrow
                    .as_mut()
                    .unwrap()
                    .begin(cx, walk, self.layout);
                self.icon_arrow.as_mut().unwrap().end(cx);
            }
            crate::shader::icon_lib::types::DrawGIconType::Emoji => {
                self.icon_emoji
                    .as_mut()
                    .unwrap()
                    .begin(cx, walk, self.layout);
                self.icon_emoji.as_mut().unwrap().end(cx);
            }
            crate::shader::icon_lib::types::DrawGIconType::Fs => {
                self.icon_fs.as_mut().unwrap().begin(cx, walk, self.layout);
                self.icon_fs.as_mut().unwrap().end(cx);
            }
            crate::shader::icon_lib::types::DrawGIconType::UI => {
                self.icon_ui.as_mut().unwrap().begin(cx, walk, self.layout);
                self.icon_ui.as_mut().unwrap().end(cx);
            }
            crate::shader::icon_lib::types::DrawGIconType::Person => {
                self.icon_person
                    .as_mut()
                    .unwrap()
                    .begin(cx, walk, self.layout);
                self.icon_person.as_mut().unwrap().end(cx);
            }
            crate::shader::icon_lib::types::DrawGIconType::Relation => {
                self.icon_relation
                    .as_mut()
                    .unwrap()
                    .begin(cx, walk, self.layout);
                self.icon_relation.as_mut().unwrap().end(cx);
            }
            crate::shader::icon_lib::types::DrawGIconType::State => {
                self.icon_state
                    .as_mut()
                    .unwrap()
                    .begin(cx, walk, self.layout);
                self.icon_state.as_mut().unwrap().end(cx);
            }
            crate::shader::icon_lib::types::DrawGIconType::Time => {
                self.icon_time
                    .as_mut()
                    .unwrap()
                    .begin(cx, walk, self.layout);
                self.icon_time.as_mut().unwrap().end(cx);
            }
            crate::shader::icon_lib::types::DrawGIconType::Tool => {
                self.icon_tool
                    .as_mut()
                    .unwrap()
                    .begin(cx, walk, self.layout);
                self.icon_tool.as_mut().unwrap().end(cx);
            }
        }
        self.draw_icon.end(cx);
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

impl LiveHook for GIcon {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.visible {
            return;
        }

        let color = self.stroke_color.get(self.theme, 200);
        let hover_color = self.hover_color.get(self.theme, 100);

        self.draw_type.replace(self.icon_type.to_draw_type());
        match self.draw_type.as_ref().unwrap() {
            crate::shader::icon_lib::types::DrawGIconType::Base => {
                self.icon_base.as_mut().unwrap().apply_over(
                    cx,
                    live! {
                        stroke_color: (color),
                        stroke_width: (self.stroke_width),
                        hover_color: (hover_color),
                    },
                );
                self.icon_base
                    .as_mut()
                    .unwrap()
                    .apply_type(Base::try_from(&self.icon_type).unwrap());
            }
            crate::shader::icon_lib::types::DrawGIconType::Code => {
                self.icon_code.as_mut().unwrap().apply_over(
                    cx,
                    live! {
                        stroke_color: (color),
                        stroke_width: (self.stroke_width),
                        hover_color: (hover_color),
                    },
                );
                self.icon_code
                    .as_mut()
                    .unwrap()
                    .apply_type(Code::try_from(&self.icon_type).unwrap());
            }
            crate::shader::icon_lib::types::DrawGIconType::Arrow => {
                self.icon_arrow.as_mut().unwrap().apply_over(
                    cx,
                    live! {
                        stroke_color: (color),
                        stroke_width: (self.stroke_width),
                        hover_color: (hover_color),
                    },
                );
                self.icon_arrow
                    .as_mut()
                    .unwrap()
                    .apply_type(Arrow::try_from(&self.icon_type).unwrap());
            }
            crate::shader::icon_lib::types::DrawGIconType::Emoji => {
                self.icon_emoji.as_mut().unwrap().apply_over(
                    cx,
                    live! {
                        stroke_color: (color),
                        stroke_width: (self.stroke_width),
                        hover_color: (hover_color),
                    },
                );
                self.icon_emoji
                    .as_mut()
                    .unwrap()
                    .apply_type(Emoji::try_from(&self.icon_type).unwrap());
            }
            crate::shader::icon_lib::types::DrawGIconType::Fs => {
                self.icon_fs.as_mut().unwrap().apply_over(
                    cx,
                    live! {
                        stroke_color: (color),
                        stroke_width: (self.stroke_width),
                        hover_color: (hover_color),
                    },
                );
                self.icon_fs
                    .as_mut()
                    .unwrap()
                    .apply_type(Fs::try_from(&self.icon_type).unwrap());
            }
            crate::shader::icon_lib::types::DrawGIconType::UI => {
                self.icon_ui.as_mut().unwrap().apply_over(
                    cx,
                    live! {
                        stroke_color: (color),
                        stroke_width: (self.stroke_width),
                        hover_color: (hover_color),
                    },
                );
                self.icon_ui
                    .as_mut()
                    .unwrap()
                    .apply_type(UI::try_from(&self.icon_type).unwrap());
            }
            crate::shader::icon_lib::types::DrawGIconType::Person => {
                self.icon_person.as_mut().unwrap().apply_over(
                    cx,
                    live! {
                        stroke_color: (color),
                        stroke_width: (self.stroke_width),
                        hover_color: (hover_color),
                    },
                );
                self.icon_person
                    .as_mut()
                    .unwrap()
                    .apply_type(Person::try_from(&self.icon_type).unwrap());
            }
            crate::shader::icon_lib::types::DrawGIconType::Relation => {
                self.icon_relation.as_mut().unwrap().apply_over(
                    cx,
                    live! {
                        stroke_color: (color),
                        stroke_width: (self.stroke_width),
                        hover_color: (hover_color),
                    },
                );
                self.icon_relation
                    .as_mut()
                    .unwrap()
                    .apply_type(Relation::try_from(&self.icon_type).unwrap());
            }
            crate::shader::icon_lib::types::DrawGIconType::State => {
                self.icon_state.as_mut().unwrap().apply_over(
                    cx,
                    live! {
                        stroke_color: (color),
                        stroke_width: (self.stroke_width),
                        hover_color: (hover_color),
                    },
                );
                self.icon_state
                    .as_mut()
                    .unwrap()
                    .apply_type(State::try_from(&self.icon_type).unwrap());
            }
            crate::shader::icon_lib::types::DrawGIconType::Time => {
                self.icon_time.as_mut().unwrap().apply_over(
                    cx,
                    live! {
                        stroke_color: (color),
                        stroke_width: (self.stroke_width),
                        hover_color: (hover_color),
                    },
                );
                self.icon_time
                    .as_mut()
                    .unwrap()
                    .apply_type(Time::try_from(&self.icon_type).unwrap());
            }
            crate::shader::icon_lib::types::DrawGIconType::Tool => {
                self.icon_tool.as_mut().unwrap().apply_over(
                    cx,
                    live! {
                        stroke_color: (color),
                        stroke_width: (self.stroke_width),
                        hover_color: (hover_color),
                    },
                );
                self.icon_tool
                    .as_mut()
                    .unwrap()
                    .apply_type(Tool::try_from(&self.icon_type).unwrap());
            }
        }

        self.draw_icon.redraw(cx);
    }
}

impl GIcon {
    widget_area! {
        area, draw_icon
    }
    event_option! {
        clicked : GIconEvent::Clicked => KeyModifiers,
        hover : GIconEvent::Hover => KeyModifiers
    }
    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
        self.draw_icon.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) -> () {
        self.draw_icon.apply_over(
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

        if self.animation_open {
            if self.animator_handle_event(cx, event).must_redraw() {
                self.draw_icon.redraw(cx);
            }
        }

        match hit {
            Hit::FingerDown(_) => {
                if self.grab_key_focus {
                    cx.set_key_focus(focus_area);
                }
                // self.animator_play(cx, id!(hover.pressed));
            }
            Hit::FingerHoverIn(h) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                self.animator_play(cx, id!(hover.on));
                cx.widget_action(uid, &scope.path, GIconEvent::Hover(h.modifiers));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerUp(f_up) => {
                if f_up.is_over {
                    cx.widget_action(uid, &scope.path, GIconEvent::Clicked(f_up.modifiers));
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

impl GIconRef {
    ref_event_option! {
        clicked => KeyModifiers,
        hover => KeyModifiers
    }
    animatie_fn! {
        animate_hover_on,
        animate_hover_off
    }
}

impl GIconSet {
    set_event! {
        clicked,
        hover
    }
}
