use makepad_widgets::*;

use super::IconType;

#[derive(Live, LiveHook, Clone, Debug)]
#[live_ignore]
#[repr(u32)]
pub enum Time {
    #[pick]
    Clock
}

impl TryFrom<&IconType> for Time {
    type Error = ();

    fn try_from(value: &IconType) -> Result<Self, Self::Error> {
        match value {
            IconType::Clock => Ok(Self::Clock),
            _ => Err(()),
        }
    }
}