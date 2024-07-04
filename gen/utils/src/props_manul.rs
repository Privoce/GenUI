pub struct Font;

impl Font {
    /// 字体类型
    pub const FONT_FAMILY: &'static str = "font_family";
    /// 字体大小
    pub const FONT_SIZE: &'static str = "font_size";
    /// 字体粗细
    pub const FONT_WEIGHT: &'static str = "font_weight";
    /// 字体缩放
    pub const FONT_SCALE: &'static str = "font_scale";
    /// 字体亮度
    pub const BRIGHTNESS: &'static str = "brightness";
    /// 字体曲线
    pub const CURVE: &'static str = "curve";
    /// 字体行间距
    pub const LINE_SPACING: &'static str = "line_spacing";
    /// 起始字符高度
    pub const TOP_DROP: &'static str = "top_drop";
    /// 高度因子
    pub const HEIGHT_FACTOR: &'static str = "height_factor";
}

pub struct Text;

impl Text {
    /// 文本内容
    pub const TEXT: &'static str = "text";
    /// 文本深度
    pub const DRAW_DEPTH: &'static str = "draw_depth";
    /// 忽略换行
    pub const IGNORE_NEWLINES: &'static str = "ignore_newlines";
    /// 合并空格
    pub const COMBINE_SPACES: &'static str = "combine_spaces";
    /// 文本换行行为
    pub const TEXT_WRAP: &'static str = "text_wrap";
    /// 文本颜色
    pub const COLOR: &'static str = "color";
    /// 文本对齐
    pub const TEXT_ALIGN: &'static str = "text_align";
    /// 空白
    pub const EMPTY: &'static str = "empty";
    /// 空消息
    pub const EMPTY_MESSAGE: &'static str = "empty_message";
}

pub struct Size;

impl Size {
    /// 宽度
    pub const WIDTH: &'static str = "width";
    /// 高度
    pub const HEIGHT: &'static str = "height";
    /// 最小宽度
    pub const MIN_WIDTH: &'static str = "min_width";
    /// 最小高度
    pub const MIN_HEIGHT: &'static str = "min_height";
    /// 最大宽度
    pub const MAX_WIDTH: &'static str = "max_width";
    /// 最大高度
    pub const MAX_HEIGHT: &'static str = "max_height";
    /// 外边距
    pub const MARGIN: &'static str = "margin";
    /// 内边距
    pub const PADDING: &'static str = "padding";
    pub const CLIP_X: &'static str = "clip_x";
    pub const CLIP_Y: &'static str = "clip_y";
    /// 窗口大小
    pub const WINDOW_SIZE: &'static str = "window_size";
    pub const WIDTH_SCALE: &'static str = "width_scale";
}

pub struct Position;

impl Position {
    /// 定位
    pub const ABS_POS: &'static str = "abs_pos";
    /// 子元素定位
    pub const ALIGN: &'static str = "align";
    /// 排序
    pub const FLOW: &'static str = "flow";
    /// 间距
    pub const SPACING: &'static str = "spacing";
    /// 窗口位置
    pub const WINDOW_POSITION: &'static str = "window_position";
}

pub struct Background;

impl Background {
    /// 背景颜色
    pub const BACKGROUND_COLOR: &'static str = "background_color";
    /// 显示背景
    pub const BACKGROUND_VISIBLE: &'static str = "background_visible";
}

pub struct Border;

impl Border {
    /// 边框颜色
    pub const BORDER_COLOR: &'static str = "border_color";
    /// 边框宽度
    pub const BORDER_WIDTH: &'static str = "border_width";
    /// 边框圆角
    pub const BORDER_RADIUS: &'static str = "border_radius";
}

pub struct Others;

impl Others {
    /// 可见性
    pub const VISIBLE: &'static str = "visible";
    pub const SCROLL: &'static str = "scroll";
    /// 优化方案
    pub const OPTIMIZE: &'static str = "optimize";
    pub const SELECT_PAD_EDGES: &'static str = "select_pad_edges";
    pub const ON_FOCUS_SELECT_ALL: &'static str = "on_focus_select_all";
}

pub struct Resource;

impl Resource {
    /// 资源
    pub const SOURCE: &'static str = "source";
    /// 适应(用于图片)
    pub const FIT: &'static str = "fit";
    /// 媒体
    pub const MEDIA: &'static str = "media";
}

pub struct Event;

impl Event {
    /// 点击
    pub const CLICKED: &'static str = "clicked";
    /// 悬停
    pub const HOVER: &'static str = "hover";
    /// 聚焦
    pub const FOCUS: &'static str = "focus";
    /// 选择
    pub const SELECTED: &'static str = "selected";
    /// 绑定
    pub const BIND: &'static str = "bind";
    /// 事件顺序
    pub const EVENT_ORDER: &'static str = "event_order";
    /// 事件透传
    pub const GRAB_KEY_FOCUS: &'static str = "grab_key_focus";
    /// 阻止事件
    pub const BLOCK_SIGNAL_EVENT: &'static str = "block_signal_event";
}

pub struct Cursor;

impl Cursor {
    /// 鼠标样式
    pub const CURSOR: &'static str = "cursor";
    /// 鼠标(光标)大小
    pub const CURSOR_SIZE: &'static str = "cursor_size";
    /// 鼠标(光标) margin bottom
    pub const CURSOR_MARGIN_BOTTOM: &'static str = "cursor_margin_bottom";
    /// 鼠标(光标) margin top
    pub const CURSOR_MARGIN_TOP: &'static str = "cursor_margin_top";
}

pub struct State;

impl State {
    /// 仅数字
    pub const NUMERIC_ONLY: &'static str = "numeric_only";
    /// 仅ASCII
    pub const ASCII_ONLY: &'static str = "ascii_only";
    /// 只读
    pub const READ_ONLY: &'static str = "read_only";
    pub const SECRET: &'static str = "secret";
}