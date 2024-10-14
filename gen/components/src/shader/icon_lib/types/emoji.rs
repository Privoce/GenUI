use makepad_widgets::*;

use crate::error::GError;

use super::IconType;

#[derive(Live, LiveHook, Clone, Debug)]
#[live_ignore]
#[repr(u32)]
pub enum Emoji {
    #[pick]
    Emoji = shader_enum(1),
    Hot,
    Heart,
    HeartBroken,
    Dislike,
}

impl TryFrom<&IconType> for Emoji {
    type Error = GError;

    fn try_from(value: &IconType) -> Result<Self, Self::Error> {
        match value {
            IconType::Emoji => Ok(Self::Emoji),
            IconType::Hot => Ok(Self::Hot),
            IconType::Heart => Ok(Self::Heart),
            IconType::HeartBroken => Ok(Self::HeartBroken),
            IconType::Dislike => Ok(Self::Dislike),
            _ => Err(GError::IconTypeTransfom),
        }
    }
}