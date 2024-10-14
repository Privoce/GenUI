use makepad_widgets::*;

use crate::error::GError;

use super::IconType;

#[derive(Live, LiveHook, Clone, Debug)]
#[live_ignore]
#[repr(u32)]
pub enum Time {
    #[pick]
    Clock = shader_enum(1)
}

impl TryFrom<&IconType> for Time {
    type Error = GError;

    fn try_from(value: &IconType) -> Result<Self, Self::Error> {
        match value {
            IconType::Clock => Ok(Self::Clock),
            _ => Err(GError::IconTypeTransfom),
        }
    }
}