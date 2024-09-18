pub mod arrow;
pub mod base;
pub mod code;
pub mod emoji;
pub mod fs;
pub mod person;
pub mod relation;
pub mod state;
pub mod time;
pub mod tool;
pub mod ui;

use arrow::Arrow;
use base::Base;
use code::Code;
use emoji::Emoji;
use fs::Fs;

use makepad_widgets::*;
use person::Person;
use relation::Relation;
use state::State;
use time::Time;
use tool::Tool;
use ui::UI;

#[derive(Live, LiveHook, Clone, Debug)]
#[live_ignore]
#[repr(u32)]
pub enum IconType {
    #[pick]
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
    // Default = shader_enum(30),
    DeleteKey = shader_enum(30),
    FullScreenExpand,
    Setting2,
    Setting3,
    Hot,
    Heart,
    HeartBroken,
    Dislike,
    Rss,
    Share,
    ZoomIn,
    ZoomOut,
    Eye,
    EyeClose,
    Search,
    Connect,
    Disconnect,
    Debug,
    Code,
    Test,
    Open,
    OpenLeft,
    OpenRight,
    OpenTop,
    OpenBottom,
    Split,
    Split2,
    Wifi,
    WifiNone,
    AI,
    VR,
    Note,
    Notice,
    NoticeNone,
    Clock,
    /// i
    Info,
    /// ?
    Help,
    /// !
    Warn,
    Poweroff,
    Light,
    Male,
    Female,
    Home,
    System,
    Picture,
    GoOn,
}

impl IconType {
    pub fn to_draw_type(&self) -> DrawGIconType {
        Base::try_from(self)
            .is_ok()
            .then(|| DrawGIconType::Base)
            .or_else(|| Code::try_from(self).is_ok().then(|| DrawGIconType::Code))
            .or_else(|| Arrow::try_from(self).is_ok().then(|| DrawGIconType::Arrow))
            .or_else(|| Emoji::try_from(self).is_ok().then(|| DrawGIconType::Emoji))
            .or_else(|| Fs::try_from(self).is_ok().then(|| DrawGIconType::Fs))
            .or_else(|| UI::try_from(self).is_ok().then(|| DrawGIconType::UI))
            .or_else(|| {
                Person::try_from(self)
                    .is_ok()
                    .then(|| DrawGIconType::Person)
            })
            .or_else(|| {
                Relation::try_from(self)
                    .is_ok()
                    .then(|| DrawGIconType::Relation)
            })
            .or_else(|| State::try_from(self).is_ok().then(|| DrawGIconType::State))
            .or_else(|| Time::try_from(self).is_ok().then(|| DrawGIconType::Time))
            .or_else(|| Tool::try_from(self).is_ok().then(|| DrawGIconType::Tool))
            .unwrap()
    }
}

impl From<Base> for IconType {
    fn from(value: Base) -> Self {
        match value {
            Base::Min => IconType::Min,
            Base::Max => IconType::Max,
            Base::FullScreen => IconType::FullScreen,
            Base::FullScreenExpand => IconType::FullScreenExpand,
            Base::More => IconType::More,
            Base::Close => IconType::Close,
            Base::Upload => IconType::Upload,
            Base::Download => IconType::Download,
            Base::Add => IconType::Add,
            Base::Delete => IconType::Delete,
            Base::DeleteKey => IconType::DeleteKey,
            Base::Correct => IconType::Correct,
            Base::Fresh => IconType::Fresh,
            Base::Play => IconType::Play,
            Base::Stop => IconType::Stop,
            Base::GoOn => IconType::GoOn,
            Base::Setting => IconType::Setting,
            Base::Setting2 => IconType::Setting2,
            Base::Setting3 => IconType::Setting3,
            Base::Home => IconType::Home,
            Base::System => IconType::System,
            Base::Picture => IconType::Picture,
            Base::Eye => IconType::Eye,
            Base::EyeClose => IconType::EyeClose,
            Base::Phone => IconType::Phone,
            Base::Light => IconType::Light,
            Base::Menu => IconType::Menu,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DrawGIconType {
    Base,
    Code,
    Arrow,
    Emoji,
    Fs,
    UI,
    Person,
    Relation,
    State,
    Time,
    Tool,
}

impl Default for DrawGIconType {
    fn default() -> Self {
        Self::Base
    }
}
