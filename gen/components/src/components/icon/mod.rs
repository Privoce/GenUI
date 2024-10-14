mod event;
mod register;

pub use event::*;
use makepad_widgets::*;
pub use register::register;

use crate::{
    active_event, animatie_fn, default_handle_animation, default_hit_finger_down,
    default_hit_finger_up, default_hit_hover_in, default_hit_hover_out, event_option,
    play_animation, ref_event_option, set_event, set_scope_path,
    shader::icon_lib::{
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
        ApplyIconType,
    },
    themes::Themes,
    utils::{set_cursor, ThemeColor},
    widget_area,
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
    pub color: Option<Vec4>,
    #[live]
    pub stroke_hover_color: Option<Vec4>,
    #[live]
    pub stroke_focus_color: Option<Vec4>,
    #[live(1.0)]
    pub stroke_width: f32,
    #[live]
    pub cursor: Option<MouseCursor>,
    #[live(true)]
    pub visible: bool,
    #[live(true)]
    pub grab_key_focus: bool,
    #[live(false)]
    pub animation_key: bool,
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
    #[live(true)]
    pub event_key: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
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
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        if !self.visible {
            return;
        }
        match self.icon_type.to_draw_type() {
            Ok(ty) => {
                self.draw_type.replace(ty);
            }
            Err(e) => {
                cx.apply_error(live_error_origin!(), index, nodes, e.to_string());
            }
        }
        self.render(cx);
        self.draw_icon.redraw(cx);
    }
}

impl GIcon {
    set_scope_path!();
    play_animation!();
    widget_area! {
        area, draw_icon
    }
    event_option! {
        clicked: GIconEvent::Clicked => GIconClickedParam,
        hover_in: GIconEvent::HoverIn => GIconHoverParam,
        hover_out: GIconEvent::HoverOut => GIconHoverParam,
        focus: GIconEvent::Focus => GIconFocusParam,
        focus_lost: GIconEvent::FocusLost => GIconFocusLostParam
    }
    active_event! {
        active_hover_in: GIconEvent::HoverIn |e: FingerHoverEvent| => GIconHoverParam{ e },
        active_hover_out: GIconEvent::HoverOut |e: FingerHoverEvent| => GIconHoverParam{ e },
        active_focus: GIconEvent::Focus |e: FingerDownEvent| => GIconFocusParam{ e },
        active_focus_lost: GIconEvent::FocusLost |e: FingerUpEvent| => GIconFocusLostParam{ e },
        active_clicked: GIconEvent::Clicked |e: FingerUpEvent| => GIconClickedParam{ e }
    }
    pub fn render(&mut self, cx: &mut Cx) {
        fn handle<T>(
            target: &mut Option<T>,
            cx: &mut Cx,
            colors: [Vec4; 3],
            stroke_width: f32,
            icon_type: &IconType,
        ) -> ()
        where
            T: LiveApply + ApplyIconType,
        {
            target.as_mut().unwrap().apply_over(
                cx,
                live! {
                    color: (colors[0]),
                    stroke_width: (stroke_width),
                    stroke_hover_color: (colors[1]),
                    stroke_focus_color: (colors[2]),
                },
            );

            let _ = target.as_mut().unwrap().apply_type(icon_type);
        }

        // let color = self.color.get(self.theme, 25);
        // let stroke_hover_color = self.stroke_hover_color.get(self.theme, 100);
        // let stroke_focus_color = self.stroke_focus_color.get(self.theme, 100);

        let colors = [
            self.color.get(self.theme, 25),
            self.stroke_hover_color.get(self.theme, 100),
            self.stroke_focus_color.get(self.theme, 100),
        ];

        match self.draw_type.as_ref().unwrap() {
            crate::shader::icon_lib::types::DrawGIconType::Base => {
                handle(
                    &mut self.icon_base,
                    cx,
                    colors,
                    self.stroke_width,
                    &self.icon_type,
                );
                // self.icon_base.as_mut().unwrap().apply_over(
                //     cx,
                //     live! {
                //         stroke_color: (color),
                //         stroke_width: (self.stroke_width),
                //         stroke_hover_color: (stroke_hover_color),
                //     },
                // );
                // self.icon_base
                //     .as_mut()
                //     .unwrap()
                //     .apply_type(Base::try_from(&self.icon_type).unwrap());
                // self.icon_base.as_mut().unwrap().apply_type(&self.icon_type);
            }
            crate::shader::icon_lib::types::DrawGIconType::Code => {
                // self.icon_code.as_mut().unwrap().apply_over(
                //     cx,
                //     live! {
                //         stroke_color: (color),
                //         stroke_width: (self.stroke_width),
                //         stroke_hover_color: (stroke_hover_color),
                //     },
                // );
                // self.icon_code
                //     .as_mut()
                //     .unwrap()
                //     .apply_type(Code::try_from(&self.icon_type).unwrap());
                handle(
                    &mut self.icon_code,
                    cx,
                    colors,
                    self.stroke_width,
                    &self.icon_type,
                );
            }
            crate::shader::icon_lib::types::DrawGIconType::Arrow => {
                // self.icon_arrow.as_mut().unwrap().apply_over(
                //     cx,
                //     live! {
                //         stroke_color: (color),
                //         stroke_width: (self.stroke_width),
                //         stroke_hover_color: (stroke_hover_color),
                //     },
                // );
                // self.icon_arrow
                //     .as_mut()
                //     .unwrap()
                //     .apply_type(Arrow::try_from(&self.icon_type).unwrap());
                handle(
                    &mut self.icon_arrow,
                    cx,
                    colors,
                    self.stroke_width,
                    &self.icon_type,
                );
            }
            crate::shader::icon_lib::types::DrawGIconType::Emoji => {
                // self.icon_emoji.as_mut().unwrap().apply_over(
                //     cx,
                //     live! {
                //         stroke_color: (color),
                //         stroke_width: (self.stroke_width),
                //         stroke_hover_color: (stroke_hover_color),
                //     },
                // );
                // self.icon_emoji
                //     .as_mut()
                //     .unwrap()
                //     .apply_type(Emoji::try_from(&self.icon_type).unwrap());
                handle(
                    &mut self.icon_emoji,
                    cx,
                    colors,
                    self.stroke_width,
                    &self.icon_type,
                );
            }
            crate::shader::icon_lib::types::DrawGIconType::Fs => {
                // self.icon_fs.as_mut().unwrap().apply_over(
                //     cx,
                //     live! {
                //         stroke_color: (color),
                //         stroke_width: (self.stroke_width),
                //         stroke_hover_color: (stroke_hover_color),
                //     },
                // );
                // self.icon_fs
                //     .as_mut()
                //     .unwrap()
                //     .apply_type(Fs::try_from(&self.icon_type).unwrap());
                handle(
                    &mut self.icon_fs,
                    cx,
                    colors,
                    self.stroke_width,
                    &self.icon_type,
                );
            }
            crate::shader::icon_lib::types::DrawGIconType::UI => {
                // self.icon_ui.as_mut().unwrap().apply_over(
                //     cx,
                //     live! {
                //         stroke_color: (color),
                //         stroke_width: (self.stroke_width),
                //         stroke_hover_color: (stroke_hover_color),
                //     },
                // );
                // self.icon_ui
                //     .as_mut()
                //     .unwrap()
                //     .apply_type(UI::try_from(&self.icon_type).unwrap());
                handle(
                    &mut self.icon_ui,
                    cx,
                    colors,
                    self.stroke_width,
                    &self.icon_type,
                );
            }
            crate::shader::icon_lib::types::DrawGIconType::Person => {
                // self.icon_person.as_mut().unwrap().apply_over(
                //     cx,
                //     live! {
                //         stroke_color: (color),
                //         stroke_width: (self.stroke_width),
                //         stroke_hover_color: (stroke_hover_color),
                //     },
                // );
                // self.icon_person
                //     .as_mut()
                //     .unwrap()
                //     .apply_type(Person::try_from(&self.icon_type).unwrap());
                handle(
                    &mut self.icon_person,
                    cx,
                    colors,
                    self.stroke_width,
                    &self.icon_type,
                );
            }
            crate::shader::icon_lib::types::DrawGIconType::Relation => {
                // self.icon_relation.as_mut().unwrap().apply_over(
                //     cx,
                //     live! {
                //         stroke_color: (color),
                //         stroke_width: (self.stroke_width),
                //         stroke_hover_color: (stroke_hover_color),
                //     },
                // );
                // self.icon_relation
                //     .as_mut()
                //     .unwrap()
                //     .apply_type(Relation::try_from(&self.icon_type).unwrap());
                handle(
                    &mut self.icon_relation,
                    cx,
                    colors,
                    self.stroke_width,
                    &self.icon_type,
                );
            }
            crate::shader::icon_lib::types::DrawGIconType::State => {
                // self.icon_state.as_mut().unwrap().apply_over(
                //     cx,
                //     live! {
                //         stroke_color: (color),
                //         stroke_width: (self.stroke_width),
                //         stroke_hover_color: (stroke_hover_color),
                //     },
                // );
                // self.icon_state
                //     .as_mut()
                //     .unwrap()
                //     .apply_type(State::try_from(&self.icon_type).unwrap());
                handle(
                    &mut self.icon_state,
                    cx,
                    colors,
                    self.stroke_width,
                    &self.icon_type,
                );
            }
            crate::shader::icon_lib::types::DrawGIconType::Time => {
                // self.icon_time.as_mut().unwrap().apply_over(
                //     cx,
                //     live! {
                //         stroke_color: (color),
                //         stroke_width: (self.stroke_width),
                //         stroke_hover_color: (stroke_hover_color),
                //     },
                // );
                // self.icon_time
                //     .as_mut()
                //     .unwrap()
                //     .apply_type(Time::try_from(&self.icon_type).unwrap());
                handle(
                    &mut self.icon_time,
                    cx,
                    colors,
                    self.stroke_width,
                    &self.icon_type,
                );
            }
            crate::shader::icon_lib::types::DrawGIconType::Tool => {
                // self.icon_tool.as_mut().unwrap().apply_over(
                //     cx,
                //     live! {
                //         stroke_color: (color),
                //         stroke_width: (self.stroke_width),
                //         stroke_hover_color: (stroke_hover_color),
                //     },
                // );
                // self.icon_tool
                //     .as_mut()
                //     .unwrap()
                //     .apply_type(Tool::try_from(&self.icon_type).unwrap());
                handle(
                    &mut self.icon_tool,
                    cx,
                    colors,
                    self.stroke_width,
                    &self.icon_type,
                );
            }
        }
    }
    pub fn redraw(&self, cx: &mut Cx) {
        self.draw_icon.redraw(cx);
        match self.draw_type.as_ref().unwrap() {
            DrawGIconType::Base => {
                self.icon_base.as_ref().unwrap().redraw(cx);
            }
            DrawGIconType::Code => {
                self.icon_code.as_ref().unwrap().redraw(cx);
            }
            DrawGIconType::Arrow => {
                self.icon_arrow.as_ref().unwrap().redraw(cx);
            }
            DrawGIconType::Emoji => {
                self.icon_emoji.as_ref().unwrap().redraw(cx);
            }
            DrawGIconType::Fs => {
                self.icon_fs.as_ref().unwrap().redraw(cx);
            }
            DrawGIconType::UI => {
                self.icon_ui.as_ref().unwrap().redraw(cx);
            }
            DrawGIconType::Person => {
                self.icon_person.as_ref().unwrap().redraw(cx);
            }
            DrawGIconType::Relation => {
                self.icon_relation.as_ref().unwrap().redraw(cx);
            }
            DrawGIconType::State => {
                self.icon_state.as_ref().unwrap().redraw(cx);
            }
            DrawGIconType::Time => {
                self.icon_time.as_ref().unwrap().redraw(cx);
            }
            DrawGIconType::Tool => {
                self.icon_tool.as_ref().unwrap().redraw(cx);
            }
        }
    }
    pub fn clear_animation(&mut self, cx: &mut Cx) {
        self.draw_icon.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0,
            },
        );
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
    pub fn animate_focus_on(&mut self, cx: &mut Cx) -> () {
        self.draw_icon.apply_over(
            cx,
            live! {
                focus: 1.0,
            },
        );
    }
    pub fn animate_focus_off(&mut self, cx: &mut Cx) -> () {
        self.draw_icon.apply_over(
            cx,
            live! {
                focus: 0.0,
            },
        );
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
            Hit::FingerDown(e) => {
                default_hit_finger_down!(self, cx, focus_area, e);
            }
            Hit::FingerHoverIn(e) => {
                default_hit_hover_in!(self, cx, e);
            }
            Hit::FingerHoverOut(e) => {
                default_hit_hover_out!(self, cx, e);
            }
            Hit::FingerUp(e) => {
                default_hit_finger_up!(self, cx, e);
            }
            _ => (),
        }
    }
}

impl GIconRef {
    ref_event_option! {
        hover_in => GIconHoverParam,
        hover_out => GIconHoverParam,
        focus => GIconFocusParam,
        focus_lost => GIconFocusLostParam,
        clicked => GIconClickedParam
    }
    animatie_fn! {
        clear_animation,
        animate_hover_on,
        animate_hover_off,
        animate_focus_on,
        animate_focus_off
    }
}

impl GIconSet {
    set_event! {
        hover_in => GIconHoverParam,
        hover_out => GIconHoverParam,
        focus => GIconFocusParam,
        focus_lost => GIconFocusLostParam,
        clicked => GIconClickedParam
    }
}
