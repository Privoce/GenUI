use makepad_widgets::*;

use crate::error::GError;

use super::IconType;

#[derive(Live, LiveHook, Clone, Debug)]
#[live_ignore]
#[repr(u32)]
pub enum Tool {
    #[pick]
    Search = shader_enum(1),
    ZoomIn,
    ZoomOut,
    Share,
    Rss,
    AI,
    VR,
    Notice,
    NoticeNone,
    Bind
}

impl TryFrom<&IconType> for Tool {
    type Error = GError;

    fn try_from(value: &IconType) -> Result<Self, Self::Error> {
        match value {
            IconType::Search => Ok(Self::Search),
            IconType::ZoomIn => Ok(Self::ZoomIn),
            IconType::ZoomOut => Ok(Self::ZoomOut),
            IconType::Share => Ok(Self::Share),
            IconType::Rss => Ok(Self::Rss),
            IconType::AI => Ok(Self::AI),
            IconType::VR => Ok(Self::VR),
            IconType::Notice => Ok(Self::Notice),
            IconType::NoticeNone => Ok(Self::NoticeNone),
            IconType::Bind => Ok(Self::Bind),
            _ => Err(GError::IconTypeTransfom),
        }
    }
}