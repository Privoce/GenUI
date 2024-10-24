use makepad_widgets::*;

use super::draw_radio::GChooseType;

live_design! {
    import makepad_draw::shader::std::*;

    DrawGCheckBox = {{DrawGCheckBox}} {

        fn get_background_color(self) -> vec4 {
            return mix(
                mix(
                    self.background_color,
                    self.hover_color,
                    self.hover
                ),
                self.selected_color,
                self.selected
            );
        }

        fn get_border_color(self) -> vec4{
            return self.border_color;
        }

        fn get_stroke_color(self) -> vec4 {
            return mix(
                mix(
                    self.stroke_color,
                    self.stroke_hover_color,
                    self.hover
                ),
                self.stroke_selected_color,
                self.selected
            );
        }

        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size)
            let sz = self.size;
            let center = sz + self.border_width;
            sdf.box(self.border_width, self.border_width, sz * 2.0, sz * 2.0, 1.6);
            if self.background_visible == 1.0{
                sdf.fill_keep(self.get_background_color())
            }
            sdf.stroke(self.get_border_color(), self.border_width);
            match self.check_type {
                GChooseType::Round => {
                    let isz = sz * self.scale;
                    let i_point = vec2(sz - isz + self.border_width);
                    sdf.box(i_point.x, i_point.y , isz * 2.0, isz * 2.0, 1.0);
                    sdf.fill(
                        self.get_stroke_color()
                    );
                }
                GChooseType::Tick => {
                    let stroke_width = self.size * 0.16;
                    let start = (sz + self.border_width) * 0.5;
                    let end = (sz + self.border_width) * 2.0 - start;
                    sdf.move_to(center * 0.5, center);
                    sdf.line_to(center * 0.85, end * 0.9);
                    sdf.line_to(end, center * 0.65);
                    sdf.stroke(self.get_stroke_color(), stroke_width);
                }
                GChooseType::Cross => {
                    let stroke_width = self.size * pow(self.scale / 1.4, 1.86);
                    let start = (sz + self.border_width) * 0.5;
                    let end = (sz + self.border_width) * 2.0 - start;
                    sdf.move_to(start, self.rect_size.y * 0.5 - stroke_width * 0.5);
                    sdf.line_to(end , self.rect_size.y * 0.5 - stroke_width * 0.5);
                    sdf.stroke(self.get_stroke_color(), stroke_width);
                }
            }
            return sdf.result
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawGCheckBox {
    #[deref]
    pub draw_super: DrawQuad,
    // ---- event state
    #[live]
    pub hover: f32, // 盒子的hover状态
    #[live]
    pub selected: f32, // 盒子的选中状态
    // ---- colors
    #[live]
    pub background_color: Vec4,
    #[live]
    pub stroke_color: Vec4,
    #[live(1.0)]
    pub background_visible: f32,
    #[live]
    pub stroke_hover_color: Vec4,
    #[live]
    pub stroke_selected_color: Vec4,
    #[live]
    pub hover_color: Vec4,
    #[live]
    pub selected_color: Vec4,
    #[live]
    pub border_color: Vec4,
    // ---- size
    #[live(8.0)]
    pub size: f32, // 盒子的大小
    #[live(1.0)]
    pub border_width: f32, // 盒子的边框宽度
    #[live(0.64)]
    pub scale: f32, // 盒子内部绘制的缩放比例
    // ---- type
    #[live]
    pub check_type: GChooseType,
}

impl DrawGCheckBox {
    pub fn apply_type(&mut self, check_type: GChooseType) {
        self.check_type = check_type;
    }
}
