use makepad_widgets::*;

use crate::shader::icon_lib::{base::DrawGIconBase, types::{base::Base, DrawGIconType, IconType}};

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
    pub icon_base: DrawGIconBase,
    #[rust]
    pub draw_type: Option<DrawGIconType>, 
}

impl Widget for GIcon {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_icon.begin(cx, walk, self.layout);
        match self.draw_type.as_ref().unwrap() {
            crate::shader::icon_lib::types::DrawGIconType::Base => {
                self.icon_base.begin(cx, walk, self.layout);
                self.icon_base.end(cx);
            },
            crate::shader::icon_lib::types::DrawGIconType::Code => todo!(),
            crate::shader::icon_lib::types::DrawGIconType::Arrow => todo!(),
            crate::shader::icon_lib::types::DrawGIconType::Emoji => todo!(),
            crate::shader::icon_lib::types::DrawGIconType::Fs => todo!(),
            crate::shader::icon_lib::types::DrawGIconType::UI => todo!(),
            crate::shader::icon_lib::types::DrawGIconType::Person => todo!(),
            crate::shader::icon_lib::types::DrawGIconType::Relation => todo!(),
            crate::shader::icon_lib::types::DrawGIconType::State => todo!(),
            crate::shader::icon_lib::types::DrawGIconType::Time => todo!(),
            crate::shader::icon_lib::types::DrawGIconType::Tool => todo!(),
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
                self.icon_base.apply_over(
                    cx,
                    live! {
                        stroke_color: (color),
                    },
                );
                self.icon_base.apply_type(Base::try_from(&self.icon_type).unwrap());
            },
            crate::shader::icon_lib::types::DrawGIconType::Code => todo!(),
            crate::shader::icon_lib::types::DrawGIconType::Arrow => todo!(),
            crate::shader::icon_lib::types::DrawGIconType::Emoji => todo!(),
            crate::shader::icon_lib::types::DrawGIconType::Fs => todo!(),
            crate::shader::icon_lib::types::DrawGIconType::UI => todo!(),
            crate::shader::icon_lib::types::DrawGIconType::Person => todo!(),
            crate::shader::icon_lib::types::DrawGIconType::Relation => todo!(),
            crate::shader::icon_lib::types::DrawGIconType::State => todo!(),
            crate::shader::icon_lib::types::DrawGIconType::Time => todo!(),
            crate::shader::icon_lib::types::DrawGIconType::Tool => todo!(),
        }

        self.draw_icon.redraw(cx);
    }
}
