use makepad_widgets::*;

use crate::shader::icon_lib::{
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
        arrow::Arrow, base::Base, code::Code, emoji::Emoji, fs::Fs, person::Person, relation::Relation, state::State, time::Time, tool::Tool, ui::UI, DrawGIconType, IconType
    },
    ui::DrawGIconUI,
};

live_design! {
    import makepad_draw::shader::std::*;

    GIconBase = {{GIcon}}{
        draw_icon: {
            fn pixel(self) -> vec4{
                return vec4(0.0);
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct GIcon {
    #[live]
    pub icon_type: IconType,
    #[redraw]
    #[live]
    pub draw_icon: DrawQuad,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
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

    #[rust]
    pub draw_type: Option<DrawGIconType>,
}

impl Widget for GIcon {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
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
}

impl LiveHook for GIcon {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        let color = vec4(1.0, 1.0, 1.0, 1.0);

        self.draw_type.replace(self.icon_type.to_draw_type());
        match self.draw_type.as_ref().unwrap() {
            crate::shader::icon_lib::types::DrawGIconType::Base => {
                self.icon_base.as_mut().unwrap().apply_over(
                    cx,
                    live! {
                        stroke_color: (color),
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
