use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;
    
    DrawGToolButton = {{DrawGToolButton}}{
        /// draw bezier curve (2)
        fn bezier2(start: vec2, control: vec2, end: vec2, t: float) -> vec2 {
            let t1 = 1.0 - t;
            return t1 * t1 * start + 2.0 * t1 * t * control + t * t * end;
        }
        /// draw bezier curve (3)
        fn bezier3(start: vec2, control1: vec2, control2: vec2, end: vec2, t: float) -> vec2 {
            let t1 = 1.0 - t;
            return t1 * t1 * t1 * start + 3.0 * t1 * t1 * t * control1 + 3.0 * t1 * t * t * control2 + t * t * t * end;
        }

        fn get_color() -> vec4 {
            return mix(
                self.color,
                self.hover_color,
                self.hover
            );
        }
        fn pixel(self) -> vec4{
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            // use offset to control not overlap with border
            let offset = 0.5;
            let start_pos = vec2(self.pos.x - self.border_width + offset, self.pos.y + self.border_width + offset);
            let end_pos = vec2(self.pos.x + self.rect_size.x - self.border_width - offset, self.pos.y + self.rect_size.y - self.border_width - offset);
            let size = end_pos - start_pos;
            let stroke_width = 1.2;
            let half_size = size * 0.5;
            match self.button_type{
                GToolButtonType::Min => {
                    // draw a `-` icon as a button
                    sdf.move_to(start_pos.x, start_pos.y + size.y * 0.5);
                    sdf.line_to(end_pos.x, start_pos.y + size.y * 0.5);
                    sdf.stroke(self.get_color(), stroke_width);
                }
                GToolButtonType::Max => {
                    // draw a `▢` icon as a button
                    sdf.rect(start_pos.x, start_pos.y, size.x, size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                }
                GToolButtonType::FullScreen => {
                    // draw a `▣` icon as a button
                    sdf.rect(start_pos.x, start_pos.y, size.x, size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                }
                GToolButtonType::Left => {
                    // draw a `<` icon as a button
                    sdf.move_to(start_pos.x + size.x * 0.5, start_pos.y);
                    sdf.line_to(start_pos.x + size.x * 0.5, end_pos.y);
                    sdf.stroke(self.get_color(), stroke_width);
                    sdf.move_to(start_pos.x + size.x * 0.5, start_pos.y);
                    sdf.line_to(start_pos.x + size.x * 0.25, start_pos.y + size.y * 0.5);
                    sdf.line_to(start_pos.x + size.x * 0.5, end_pos.y);
                    sdf.fill(self.get_color());
                }
                GToolButtonType::Right => {
                    // draw a `>` icon as a button
                    sdf.move_to(start_pos.x + size.x * 0.5, start_pos.y);
                    sdf.line_to(start_pos.x + size.x * 0.5, end_pos.y);
                    sdf.stroke(self.get_color(), stroke_width);
                    sdf.move_to(start_pos.x + size.x * 0.5, start_pos.y);
                    sdf.line_to(start_pos.x + size.x * 0.75, start_pos.y + size.y * 0.5);
                    sdf.line_to(start_pos.x + size.x * 0.5, end_pos.y);
                    sdf.fill(self.get_color());
                }
                GToolButtonType::More => {
                    // draw a `⋯` icon as a button
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    sdf.move_to(start_pos.x + quarter_size.x, start_pos.y + half_size.y);
                    sdf.line_to(end_pos.x - quarter_size.x, start_pos.y + half_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                    sdf.move_to(start_pos.x + quarter_size.x, start_pos.y + half_size.y);
                    sdf.line_to(start_pos.x + quarter_size.x, end_pos.y - quarter_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                    sdf.move_to(end_pos.x - quarter_size.x, start_pos.y + half_size.y);
                    sdf.line_to(end_pos.x - quarter_size.x, end_pos.y - quarter_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                }
                GToolButtonType::Close => {
                    // draw a `×` icon as a button
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    sdf.move_to(start_pos.x + quarter_size.x, start_pos.y + quarter_size.y);
                    sdf.line_to(end_pos.x - quarter_size.x, end_pos.y - quarter_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                    sdf.move_to(start_pos.x + quarter_size.x, end_pos.y - quarter_size.y);
                    sdf.line_to(end_pos.x - quarter_size.x, start_pos.y + quarter_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                }
                GToolButtonType::Up => {
                    // draw a `︿` icon as a button
                    sdf.move_to(start_pos.x, end_pos.y);
                    sdf.line_to(start_pos.x + size.x * 0.5, start_pos.y);
                    sdf.line_to(end_pos.x, end_pos.y);
                    sdf.fill(self.get_color());
                }
                GToolButtonType::Down => {
                    // draw a `﹀` icon as a button
                    sdf.move_to(start_pos.x, start_pos.y);
                    sdf.line_to(start_pos.x + size.x * 0.5, end_pos.y);
                    sdf.line_to(end_pos.x, start_pos.y);
                    sdf.fill(self.get_color());
                }
                GToolButtonType::Switch => {
                    // draw a `⇆` icon as a button
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    sdf.move_to(start_pos.x + quarter_size.x, start_pos.y + half_size.y);
                    sdf.line_to(end_pos.x - quarter_size.x, start_pos.y + half_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                    sdf.move_to(start_pos.x + quarter_size.x, start_pos.y + half_size.y);
                    sdf.line_to(start_pos.x + quarter_size.x, end_pos.y - quarter_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                    sdf.move_to(end_pos.x - quarter_size.x, start_pos.y + half_size.y);
                    sdf.line_to(end_pos.x - quarter_size.x, end_pos.y - quarter_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                }
                GToolButtonType::Exit => {
                    // draw a `---` icon as a button
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    sdf.move_to(start_pos.x, start_pos.y + half_size.y);
                    sdf.line_to(end_pos.x, start_pos.y + half_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                    sdf.move_to(start_pos.x, start_pos.y + half_size.y);
                    sdf.line_to(start_pos.x + quarter_size.x, start_pos.y);
                    sdf.line_to(start_pos.x + quarter_size.x, end_pos.y);
                    sdf.fill(self.get_color());
                }
                GToolButtonType::Expand => {
                    // draw a `<>` icon as a button
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    sdf.move_to(start_pos.x + quarter_size.x, start_pos.y + half_size.y);
                    sdf.line_to(end_pos.x - quarter_size.x, start_pos.y + half_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                    sdf.move_to(start_pos.x + quarter_size.x, start_pos.y + half_size.y);
                    sdf.line_to(start_pos.x + half_size.x, start_pos.y + quarter_size.y);
                    sdf.line_to(start_pos.x + half_size.x, end_pos.y - quarter_size.y);
                    sdf.fill(self.get_color());
                    sdf.move_to(end_pos.x - quarter_size.x, start_pos.y + half_size.y);
                    sdf.line_to(end_pos.x - half_size.x, start_pos.y + quarter_size.y);
                    sdf.line_to(end_pos.x - half_size.x, end_pos.y - quarter_size.y);
                    sdf.fill(self.get_color());
                }
                GToolButtonType::ExpandTop => {
                    // draw a `|-----|` icon as a button
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    sdf.move_to(start_pos.x, start_pos.y + half_size.y);
                    sdf.line_to(end_pos.x, start_pos.y + half_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                    sdf.move_to(start_pos.x, start_pos.y + half_size.y);
                    sdf.line_to(start_pos.x + quarter_size.x, start_pos.y);
                    sdf.line_to(start_pos.x + quarter_size.x, end_pos.y);
                    sdf.fill(self.get_color());
                    sdf.move_to(end_pos.x, start_pos.y + half_size.y);
                    sdf.line_to(end_pos.x - quarter_size.x, start_pos.y);
                    sdf.line_to(end_pos.x - quarter_size.x, end_pos.y);
                    sdf.fill(self.get_color());
                }
                GToolButtonType::ExpandBottom => {
                    // draw a `|-----|` icon as a button
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    sdf.move_to(start_pos.x, start_pos.y + half_size.y);
                    sdf.line_to(end_pos.x, start_pos.y + half_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                    sdf.move_to(start_pos.x, start_pos.y + half_size.y);
                    sdf.line_to(start_pos.x + quarter_size.x, start_pos.y);
                    sdf.line_to(start_pos.x + quarter_size.x, end_pos.y);
                    sdf.fill(self.get_color());
                    sdf.move_to(end_pos.x, start_pos.y + half_size.y);
                    sdf.line_to(end_pos.x - quarter_size.x, start_pos.y);
                    sdf.line_to(end_pos.x - quarter_size.x, end_pos.y);
                    sdf.fill(self.get_color());
                }
                GToolButtonType::ExpandLeft => {
                    // draw a `|-----|` icon as a button
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    sdf.move_to(start_pos.x + half_size.x, start_pos.y);
                    sdf.line_to(start_pos.x + half_size.x, end_pos.y);
                    sdf.stroke(self.get_color(), stroke_width);
                    sdf.move_to(start_pos.x + half_size.x, start_pos.y);
                    sdf.line_to(start_pos.x, start_pos.y + quarter_size.y);
                    sdf.line_to(end_pos.x, start_pos.y + quarter_size.y);
                    sdf.fill(self.get_color());
                    sdf.move_to(start_pos.x + half_size.x, end_pos.y);
                    sdf.line_to(start_pos.x, end_pos.y - quarter_size.y);
                    sdf.line_to(end_pos.x, end_pos.y - quarter_size.y);
                    sdf.fill(self.get_color());
                }
                GToolButtonType::ExpandRight => {
                    // draw a `|-----|` icon as a button
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    sdf.move_to(start_pos.x + half_size.x, start_pos.y);
                    sdf.line_to(start_pos.x + half_size.x, end_pos.y);
                    sdf.stroke(self.get_color(), stroke_width);
                    sdf.move_to(start_pos.x + half_size.x, start_pos.y);
                    sdf.line_to(end_pos.x, start_pos.y + quarter_size.y);
                    sdf.line_to(end_pos.x, end_pos.y - quarter_size.y);
                    sdf.fill(self.get_color());
                    sdf.move_to(start_pos.x + half_size.x, end_pos.y);
                    sdf.line_to(end_pos.x, end_pos.y - quarter_size.y);
                    sdf.line_to(end_pos.x, start_pos.y + quarter_size.y);
                    sdf.fill(self.get_color());
                }
                GToolButtonType::Upload =>{
                    let cloud_size = size * 0.7;
                    let arrow_size = size * 0.3;
                    // draw 3 half circle as a cloud
                    // todo! wait to finish bezier curve and then finish upload , download
                }
                GToolButtonType::Download => {

                }
                GToolButtonType::Add => {
                    sdf.move_to(start_pos.x + half_size.x, start_pos.y);
                    sdf.line_to(start_pos.x + half_size.x, end_pos.y);
                    sdf.stroke(self.get_color(), stroke_width);
                    sdf.move_to(start_pos.x, start_pos.y + half_size.y);
                    sdf.line_to(end_pos.x, start_pos.y + half_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                }
                GToolButtonType::Delete => {
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    sdf.move_to(start_pos.x + quarter_size.x, start_pos.y + quarter_size.y);
                    sdf.line_to(end_pos.x - quarter_size.x, end_pos.y - quarter_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                    sdf.move_to(start_pos.x + quarter_size.x, end_pos.y - quarter_size.y);
                    sdf.line_to(end_pos.x - quarter_size.x, start_pos.y + quarter_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                }
                GToolButtonType::Correct => {
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    sdf.move_to(start_pos.x + quarter_size.x, start_pos.y + half_size.y);
                    sdf.line_to(start_pos.x + half_size.x, end_pos.y - quarter_size.y);
                    sdf.line_to(end_pos.x - quarter_size.x, start_pos.y + quarter_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                }
                GToolButtonType::Fresh => {
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    sdf.move_to(start_pos.x + half_size.x, start_pos.y + quarter_size.y);
                    sdf.line_to(start_pos.x + half_size.x, end_pos.y - quarter_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                    sdf.move_to(start_pos.x + quarter_size.x, start_pos.y + half_size.y);
                    sdf.line_to(end_pos.x - quarter_size.x, start_pos.y + half_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                    sdf.move_to(start_pos.x + half_size.x, start_pos.y + quarter_size.y);
                    sdf.line_to(end_pos.x - quarter_size.x, end_pos.y - quarter_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                }
                GToolButtonType::Play => {
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    sdf.circle(start_pos.x + half_size.x, start_pos.y + half_size.y, half_size.x);
                    sdf.fill(self.get_color());
                    sdf.move_to(start_pos.x + half_size.x, start_pos.y + quarter_size.y);
                    sdf.line_to(start_pos.x + half_size.x, end_pos.y - quarter_size.y);
                    sdf.line_to(end_pos.x - quarter_size.x, start_pos.y + half_size.y);
                    sdf.fill(self.get_color());
                }
                GToolButtonType::Stop => {
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    sdf.circle(start_pos.x + half_size.x, start_pos.y + half_size.y, half_size.x);
                    sdf.fill(self.get_color());
                    sdf.move_to(start_pos.x + quarter_size.x, start_pos.y + quarter_size.y);
                    sdf.line_to(end_pos.x - quarter_size.x, end_pos.y - quarter_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                }
                GToolButtonType::Setting => {
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    sdf.circle(start_pos.x + half_size.x, start_pos.y + half_size.y, half_size.x);
                    sdf.fill(self.get_color());
                    sdf.move_to(start_pos.x + half_size.x, start_pos.y + quarter_size.y);
                    sdf.line_to(start_pos.x + half_size.x, end_pos.y - quarter_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                    sdf.move_to(start_pos.x + quarter_size.x, start_pos.y + half_size.y);
                    sdf.line_to(end_pos.x - quarter_size.x, start_pos.y + half_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                }
                GToolButtonType::Bind => {
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    sdf.circle(start_pos.x + half_size.x, start_pos.y + half_size.y, half_size.x);
                    sdf.fill(self.get_color());
                    sdf.move_to(start_pos.x + half_size.x, start_pos.y + quarter_size.y);
                    sdf.line_to(start_pos.x + half_size.x, end_pos.y - quarter_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                    sdf.move_to(start_pos.x + quarter_size.x, start_pos.y + half_size.y);
                    sdf.line_to(end_pos.x - quarter_size.x, start_pos.y + half_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                    sdf.move_to(start_pos.x + half_size.x, start_pos.y + quarter_size.y);
                    sdf.line_to(start_pos.x + quarter_size.x, start_pos.y + half_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                    sdf.move_to(start_pos.x + half_size.x, end_pos.y - quarter_size.y);
                    sdf.line_to(start_pos.x + quarter_size.x, start_pos.y + half_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                }
                GToolButtonType::Menu => {
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    sdf.move_to(start_pos.x + quarter_size.x, start_pos.y + half_size.y);
                    sdf.line_to(end_pos.x - quarter_size.x, start_pos.y + half_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                    sdf.move_to(start_pos.x + quarter_size.x, start_pos.y + half_size.y);
                    sdf.line_to(start_pos.x + quarter_size.x, end_pos.y - quarter_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                    sdf.move_to(end_pos.x - quarter_size.x, start_pos.y + half_size.y);
                    sdf.line_to(end_pos.x - quarter_size.x, end_pos.y - quarter_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                }
                GToolButtonType::Emoji => {
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    sdf.circle(start_pos.x + half_size.x, start_pos.y + half_size.y, half_size.x);
                    sdf.fill(self.get_color());
                    sdf.circle(start_pos.x + half_size.x - quarter_size.x, start_pos.y + half_size.y - quarter_size.y, quarter_size.x);
                    sdf.fill(self.get_color);
                    sdf.circle(start_pos.x + half_size.x + quarter_size.x, start_pos.y + half_size.y - quarter_size.y, quarter_size.x);
                    sdf.fill(self.get_color);
                    sdf.move_to(start_pos.x + half_size.x - quarter_size.x, start_pos.y + half_size.y + quarter_size.y);
                    sdf.line_to(start_pos.x + half_size.x + quarter_size.x, start_pos.y + half_size.y + quarter_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                }
                GToolButtonType::Phone => {
                    let half_size = size * 0.5;
                    let quarter_size = size * 0.25;
                    sdf.circle(start_pos.x + half_size.x, start_pos.y + half_size.y, half_size.x);
                    sdf.fill(self.get_color());
                    sdf.move_to(start_pos.x + half_size.x, start_pos.y + quarter_size.y);
                    sdf.line_to(start_pos.x + half_size.x, end_pos.y - quarter_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                    sdf.move_to(start_pos.x + quarter_size.x, start_pos.y + half_size.y);
                    sdf.line_to(end_pos.x - quarter_size.x, start_pos.y + half_size.y);
                    sdf.stroke(self.get_color(), stroke_width);
                }
                GToolButtonType::None => {
                    sdf.rect(start_pos.x, start_pos.y, size.x, size.y);
                    sdf.fill(self.get_color());
                }
            }
            return sdf.result;
        }

    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawGToolButton {
    #[live]
    pub button_type: GToolButtonType,
    #[deref]
    draw_super: DrawQuad,
    #[live]
    pub color: Vec4,
    #[live]
    pub hover_color: Vec4,
    #[live]
    pub border_width: f32,
    #[live]
    pub border_color: Vec4,
}

impl DrawGToolButton {
    pub fn apply_button_type(&mut self, button_type: GToolButtonType) {
        self.button_type = button_type;
    }
}

#[derive(Live, LiveHook, Clone)]
#[live_ignore]
#[repr(u32)]
pub enum GToolButtonType {
    /// `-` (减号, 缩小, 最小化)
    Min = shader_enum(1),
    /// `▢` (最大化)
    Max = shader_enum(2),
    /// `▣` (全屏)
    FullScreen = shader_enum(3),
    /// `<` (左箭头)
    Left = shader_enum(4),
    /// `>` (右箭头)
    Right = shader_enum(5),
    /// `⋯` (更多)
    More = shader_enum(6),
    /// `×` (关闭)
    Close = shader_enum(7),
    /// `︿` (向上)
    Up = shader_enum(8),
    /// `﹀` (向下)
    Down = shader_enum(9),
    /// `⇆` (切换)
    Switch = shader_enum(10),
    /// ```
    /// ---
    /// |  > (退出)
    /// ---
    /// ``` 
    Exit = shader_enum(11),
    /// `<>` (展开)
    Expand = shader_enum(12),
    /// ```
    /// -------
    /// |-----|  (展开上边)
    /// |     |
    /// -------
    /// ```
    ExpandTop = shader_enum(13),
    /// ```
    /// -------
    /// |     |
    /// |-----|  (展开下边)
    /// -------
    /// ```
    ExpandBottom = shader_enum(14),
    /// ```
    /// ---------
    /// |  |    |  (展开左边)
    /// |  |    |
    /// ---------
    /// ```
    ExpandLeft = shader_enum(15),
    /// ```
    /// ---------
    /// |    |  |  (展开右边)
    /// |    |  |
    /// ---------
    ExpandRight = shader_enum(16),
    /// 上面有一朵云下面有个向上的箭头
    /// a cloud with an arrow pointing up below
    Upload = shader_enum(17),
    /// 上面有一朵云下面有个向下的箭头
    /// a cloud with an arrow pointing down below
    Download = shader_enum(18),
    /// `+` (加号)
    Add = shader_enum(19),
    /// 一个垃圾桶
    /// a trash can
    Delete = shader_enum(20),
    /// `✓` (勾)
    Correct = shader_enum(21),
    /// `↺` (刷新)
    Fresh = shader_enum(22),
    /// 一个圆其中有一个 ▶ (播放)
    /// a circle with a ▶ (play)
    Play = shader_enum(23),
    /// 一个圆其中有一个斜向下的横线 (停止)
    /// a circle with a diagonal line down (stop)
    Stop = shader_enum(24),
    /// 一个设置图标
    Setting = shader_enum(25),
    /// 一个类似📌图标
    /// a similar 📌 icon
    Bind = shader_enum(26),
    /// `≡` (菜单)
    Menu = shader_enum(27),
    /// 一个笑脸, 类似😀
    /// a smiley face, similar to 😀
    Emoji = shader_enum(28),
    /// 一个电话, 类似📱
    /// a phone, similar to 📱
    Phone = shader_enum(29),
    #[pick]
    /// 空白
    None = shader_enum(30),
}
