use makepad_widgets::*;

use super::IconType;

#[derive(Live, LiveHook, Clone, Debug)]
#[live_ignore]
#[repr(u32)]
pub enum Fs {
    #[pick]
    Note,
}

impl TryFrom<&IconType> for Fs {
    type Error = ();

    fn try_from(value: &IconType) -> Result<Self, Self::Error> {
        match value {
            IconType::Note => Ok(Self::Note),
            _ => Err(()),
        }
    }
}