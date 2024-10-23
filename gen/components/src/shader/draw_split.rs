use makepad_widgets::*;
live_design! {
    import makepad_draw::shader::std::*;

    DrawGSplit = {{DrawGSplit}}{
        fn pixel(self) -> vec4{

            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let offset = 0.5;
            let center = vec2(self.rect_size.x * 0.5, self.rect_size.y * 0.5);
            let q_size = self.rect_size.x * 0.25;
            match self.split_type {
                GSplitType::Spliter => {
                    // draw a `/` icon as a split
                    sdf.move_to(center.x + q_size, self.pos.y);
                    sdf.line_to(center.x - q_size, self.pos.y + self.rect_size.y);
                    sdf.stroke(self.color, 1.2);
                }
                GSplitType::Arrow => {
                    // draw a `>` icon as a split
                    sdf.move_to(q_size, center.y - q_size * 1.2);
                    sdf.line_to(self.rect_size.x - q_size * 1.2, center.y);
                    sdf.line_to(q_size, center.y + q_size * 1.2);
                    sdf.stroke(self.color, 1.2);
                }
                GSplitType::Line => {
                    // draw a `|` icon as a split
                    sdf.move_to(center.x, 0.0);
                    sdf.line_to(center.x, self.pos.y + self.rect_size.y);
                    sdf.stroke(self.color, 1.2);
                }
            }
            return sdf.result;
        }
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawGSplit {
    #[deref]
    pub draw_super: DrawQuad,
    // #[live]
    // pub hover_color: Vec4,
    // #[live(0.0)]
    // pub hover: f32,
    #[live(1.0)]
    pub brightness: f32,
    #[live(0.6)]
    pub curve: f32,
    #[live(0.5)]
    pub linearize: f32,
    #[live]
    pub color: Vec4,
    #[live]
    pub split_type: GSplitType,
}

impl DrawGSplit {
    pub fn apply_split_type(&mut self, split_type: GSplitType) {
        self.split_type = split_type;
    }
}

#[derive(Live, LiveHook, Clone, Debug)]
#[live_ignore]
#[repr(u32)]
pub enum GSplitType {
    #[pick]
    /// "/" (斜杠)
    Spliter = shader_enum(1),
    /// ">" (大于号)
    Arrow = shader_enum(2),
    /// "|" (竖线)
    Line = shader_enum(3),
}

impl ToLiveValue for GSplitType {
    fn to_live_value(&self) -> LiveValue {
        match self {
            GSplitType::Spliter => LiveValue::BareEnum(LiveId(1_u64)),
            GSplitType::Arrow => LiveValue::BareEnum(LiveId(2_u64)),
            GSplitType::Line => LiveValue::BareEnum(LiveId(3_u64)),
        }
    }
}
