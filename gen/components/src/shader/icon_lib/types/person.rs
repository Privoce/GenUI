use makepad_widgets::*;

use crate::error::GError;

use super::IconType;

#[derive(Live, LiveHook, Clone, Debug)]
#[live_ignore]
#[repr(u32)]
pub enum Person {
    #[pick]
    Male = shader_enum(1),
    Female,
}

impl TryFrom<&IconType> for Person {
    type Error = GError;

    fn try_from(value: &IconType) -> Result<Self, Self::Error> {
        match value {
            IconType::Male => Ok(Self::Male),
            IconType::Female => Ok(Self::Female),
            _ => Err(GError::IconTypeTransfom),
        }
    }
}
