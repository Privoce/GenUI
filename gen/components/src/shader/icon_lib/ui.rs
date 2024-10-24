use makepad_widgets::*;

use super::{types::ui::UI, ApplyIconType, DrawGIcon};

live_design! {
    import makepad_draw::shader::std::*;
    DrawGIconUI = {{DrawGIconUI}}{
        fn pixel(self) -> vec4{
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            // use offset to control not overlap with border
            let stroke_width = self.stroke_width;
            let offset = stroke_width * 1.25;
            let start_pos = vec2(self.pos.x + offset, self.pos.y + offset);
            let end_pos = vec2(self.pos.x + self.rect_size.x - offset * 1.0 - 1.0, self.pos.y + self.rect_size.y - offset * 1.0);
            let size = end_pos - start_pos;
            let center_y = self.rect_size.y * 0.5;
            let center_x = self.rect_size.x * 0.5;
            let half_size = size * 0.5;

            match self.icon_type{
                UI::Exit => {
                    let offset_smooth = 0.7;
                    sdf.move_to(end_pos.x * offset_smooth, start_pos.y);
                    sdf.line_to(start_pos.x, start_pos.y);
                    sdf.line_to(start_pos.x, end_pos.y);
                    sdf.line_to(end_pos.x * offset_smooth, end_pos.y);
                    sdf.move_to(end_pos.x - end_pos.x * (1.0 - offset_smooth) + size.x * 0.1, self.rect_size.y * 0.5 - size.y * 0.3);
                    sdf.line_to(end_pos.x , self.rect_size.y * 0.5);
                    sdf.line_to(end_pos.x - end_pos.x * (1.0 - offset_smooth) + size.x * 0.1, self.rect_size.y * 0.5 + size.y * 0.3);
                    sdf.move_to(end_pos.x, self.rect_size.y * 0.5);
                    sdf.line_to(end_pos.x - size.x * 0.5, self.rect_size.y * 0.5);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                UI::Expand => {
                    // draw a `<>` icon as a button
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    // first draw left `<`
                    sdf.move_to(start_pos.x + quarter_size.x, start_pos.y + quarter_size.y);
                    sdf.line_to(start_pos.x, start_pos.y + half_size.y);
                    sdf.line_to(start_pos.x + quarter_size.x, end_pos.y - quarter_size.y);
                    // then draw right `>`
                    sdf.move_to(end_pos.x - quarter_size.y, start_pos.y + quarter_size.y);
                    sdf.line_to(end_pos.x, start_pos.y + half_size.y);
                    sdf.line_to(end_pos.x - quarter_size.x, end_pos.y - quarter_size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                UI::ExpandTop => {
                    let quarter_size = size * 0.25;
                    sdf.rect(start_pos.x, start_pos.y, size.x, size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(start_pos.x, start_pos.y + quarter_size.y);
                    sdf.line_to(end_pos.x, start_pos.y + quarter_size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                UI::ExpandBottom => {
                    let quarter_size = size * 0.25;
                    sdf.rect(start_pos.x, start_pos.y, size.x, size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(start_pos.x, end_pos.y - quarter_size.y);
                    sdf.line_to(end_pos.x, end_pos.y - quarter_size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                UI::ExpandLeft => {
                    let quarter_size = size * 0.25;
                    sdf.rect(start_pos.x, start_pos.y, size.x, size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(start_pos.x + quarter_size.y, start_pos.y);
                    sdf.line_to(start_pos.x + quarter_size.y, end_pos.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                UI::ExpandRight => {
                    let quarter_size = size * 0.25;
                    sdf.rect(start_pos.x, start_pos.y, size.x, size.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(end_pos.x - quarter_size.y, start_pos.y);
                    sdf.line_to(end_pos.x - quarter_size.y, end_pos.y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                UI::Open => {
                    let quarter_size = size * 0.3;
                    sdf.circle(center_x, center_y, quarter_size.x);
                    sdf.move_to(center_x - quarter_size.x, center_y);
                    sdf.line_to(start_pos.x + quarter_size.x * 0.1, center_y);
                    sdf.move_to(center_x + quarter_size.x, center_y);
                    sdf.line_to(end_pos.x - quarter_size.x * 0.1, center_y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                UI::OpenLeft => {
                    let quarter_size = size * 0.3;
                    sdf.circle(center_x + quarter_size.x * 0.4, center_y, quarter_size.x);
                    sdf.move_to(center_x, center_y);
                    sdf.line_to(start_pos.x + quarter_size.x * 0.2, center_y);
                    sdf.line_to(center_x - quarter_size.x * 0.8, center_y - quarter_size.y * 0.6);
                    sdf.move_to(start_pos.x + quarter_size.x * 0.2, center_y);
                    sdf.line_to(center_x - quarter_size.x * 0.8, center_y + quarter_size.y * 0.6);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                UI::OpenRight => {
                    let quarter_size = size * 0.3;
                    sdf.circle(center_x - quarter_size.x * 0.4, center_y, quarter_size.x);
                    sdf.move_to(center_x, center_y);
                    sdf.line_to(end_pos.x - quarter_size.x * 0.2, center_y);
                    sdf.line_to(center_x + quarter_size.x * 0.8, center_y - quarter_size.y * 0.6);
                    sdf.move_to(end_pos.x - quarter_size.x * 0.2, center_y);
                    sdf.line_to(center_x + quarter_size.x * 0.8, center_y + quarter_size.y * 0.6);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                UI::OpenTop => {
                    let quarter_size = size * 0.3;
                    sdf.circle(center_x, center_y + quarter_size.y * 0.4, quarter_size.x);
                    sdf.move_to(center_x, center_y);
                    sdf.line_to(center_x, start_pos.y + quarter_size.y * 0.2);
                    sdf.line_to(center_x - quarter_size.x * 0.6, center_y - quarter_size.y * 0.8);
                    sdf.move_to(center_x, start_pos.y + quarter_size.y * 0.2);
                    sdf.line_to(center_x + quarter_size.x * 0.6, center_y - quarter_size.y * 0.8);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                UI::OpenBottom => {
                    let quarter_size = size * 0.3;
                    sdf.circle(center_x, center_y - quarter_size.y * 0.4, quarter_size.x);
                    sdf.move_to(center_x, center_y);
                    sdf.line_to(center_x, end_pos.y - quarter_size.y * 0.2);
                    sdf.line_to(center_x - quarter_size.x * 0.6, center_y + quarter_size.y * 0.8);
                    sdf.move_to(center_x, end_pos.y - quarter_size.y * 0.2);
                    sdf.line_to(center_x + quarter_size.x * 0.6, center_y + quarter_size.y * 0.8);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                UI::Split => {
                    let quarter_size = size * 0.25;
                    sdf.rect(start_pos.x + quarter_size.x / 2.0, start_pos.y + quarter_size.x / 2.0, size.x - quarter_size.x, size.y- quarter_size.x);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(center_x, start_pos.y + quarter_size.y * 0.5);
                    sdf.line_to(center_x, end_pos.y - quarter_size.y * 0.5);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                UI::Split2 => {
                    let quarter_size = size * 0.25;
                    sdf.rect(start_pos.x + quarter_size.x / 2.0, start_pos.y + quarter_size.x / 2.0, size.x - quarter_size.x, size.y- quarter_size.x);
                    sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.move_to(start_pos.x + quarter_size.x * 0.5, center_y);
                    sdf.line_to(end_pos.x - quarter_size.x * 0.5, center_y);
                    sdf.stroke(self.stroke_color(), stroke_width);
                }
                UI::Poweroff => {
                    let half_size = size * 0.5;
                    let quarter_size = half_size * 0.5;
                    let w = stroke_width * 2.0;
                    sdf.circle(center_x, center_y + quarter_size.y * 0.1, half_size.x * 1.0);
                    sdf.circle(center_x, center_y + quarter_size.y * 0.1, half_size.x - stroke_width * 2.0);
                    sdf.subtract();
                    sdf.rect(center_x - w * 1.8 * 0.5, center_y - half_size.x, w * 1.8, half_size.y);
                    sdf.subtract();
                    // sdf.stroke(self.stroke_color(), stroke_width);
                    sdf.fill(self.stroke_color());
                    sdf.rect(center_x - w * 0.5, center_y - half_size.x, w, half_size.y);
                    sdf.fill(self.stroke_color());
                }
            }
            return sdf.result;
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawGIconUI {
    #[deref]
    pub deref_draw: DrawGIcon,
    #[live]
    pub icon_type: UI,
}

impl DrawGIconUI {
    pub fn apply_type(&mut self, ty: UI) {
        self.icon_type = ty;
    }
}

impl ApplyIconType for DrawGIconUI {
    fn apply_type(&mut self, ty: &super::types::IconType) -> Result<(), Box<dyn std::error::Error>> {
        self.icon_type = ty.try_into()?;
        Ok(())
    }
}