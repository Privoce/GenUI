use makepad_widgets::*;

use crate::error::GError;

use super::IconType;

#[derive(Live, LiveHook, Clone, Debug)]
#[live_ignore]
#[repr(u32)]
pub enum Fs {
    #[pick]
    Note = shader_enum(1),
}

impl TryFrom<&IconType> for Fs {
    type Error = GError;

    fn try_from(value: &IconType) -> Result<Self, Self::Error> {
        match value {
            IconType::Note => Ok(Self::Note),
            _ => Err(GError::IconTypeTransfom),
        }
    }
}