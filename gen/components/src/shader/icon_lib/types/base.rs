use makepad_widgets::*;

use crate::error::GError;

use super::IconType;

#[derive(Live, LiveHook, Clone, Debug, Copy)]
#[live_ignore]
#[repr(u32)]
pub enum Base {
    #[pick]
    /// `-` (减号, 缩小, 最小化)
    Min = shader_enum(1),
    /// `▢` (最大化)
    Max,
    /// `▣` (全屏)
    FullScreen,
    /// `⇱` (全屏展开)
    FullScreenExpand,
    /// `⋯` (更多)
    More,
    /// `×` (关闭)
    Close,
    /// 上传
    Upload,
    /// 下载
    Download,
    /// `+` (加号)
    Add,
    /// 删除
    Delete,
    DeleteKey,
    Correct,
    Fresh,
    Play,
    Stop,
    GoOn,
    Setting,
    Setting2,
    Setting3,
    Home,
    System,
    Picture,
    Eye,
    EyeClose,
    Phone,
    Light,
    Menu,
}

impl TryFrom<&IconType> for Base {
    type Error = GError;

    fn try_from(value: &IconType) -> Result<Self, Self::Error> {
        match value {
            IconType::Min => Ok(Self::Min),
            IconType::Max => Ok(Self::Max),
            IconType::FullScreen => Ok(Self::FullScreen),
            IconType::FullScreenExpand => Ok(Self::FullScreenExpand),
            IconType::More => Ok(Self::More),
            IconType::Close => Ok(Self::Close),
            IconType::Upload => Ok(Self::Upload),
            IconType::Download => Ok(Self::Download),
            IconType::Add => Ok(Self::Add),
            IconType::Delete => Ok(Self::Delete),
            IconType::DeleteKey => Ok(Self::DeleteKey),
            IconType::Correct => Ok(Self::Correct),
            IconType::Fresh => Ok(Self::Fresh),
            IconType::Play => Ok(Self::Play),
            IconType::Stop => Ok(Self::Stop),
            IconType::GoOn => Ok(Self::GoOn),
            IconType::Setting => Ok(Self::Setting),
            IconType::Setting2 => Ok(Self::Setting2),
            IconType::Setting3 => Ok(Self::Setting3),
            IconType::Home => Ok(Self::Home),
            IconType::System => Ok(Self::System),
            IconType::Picture => Ok(Self::Picture),
            IconType::Eye => Ok(Self::Eye),
            IconType::EyeClose => Ok(Self::EyeClose),
            IconType::Phone => Ok(Self::Phone),
            IconType::Light => Ok(Self::Light),
            IconType::Menu => Ok(Self::Menu),
            _ => Err(GError::IconTypeTransfom),
        }
    }
}
