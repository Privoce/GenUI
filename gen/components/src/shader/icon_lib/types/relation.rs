use makepad_widgets::*;

use crate::error::GError;

use super::IconType;

#[derive(Live, LiveHook, Clone, Debug)]
#[live_ignore]
#[repr(u32)]
pub enum Relation {
    #[pick]
    Connect = shader_enum(1),
    Disconnect,
}

impl TryFrom<&IconType> for Relation {
    type Error = GError;

    fn try_from(value: &IconType) -> Result<Self, Self::Error> {
        match value {
            IconType::Connect => Ok(Self::Connect),
            IconType::Disconnect => Ok(Self::Disconnect),
            _ => Err(GError::IconTypeTransfom),
        }
    }
}