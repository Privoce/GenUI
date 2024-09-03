use makepad_widgets::*;

use super::IconType;

#[derive(Live, LiveHook, Clone, Debug)]
#[live_ignore]
#[repr(u32)]
pub enum State {
    #[pick]
    /// `i` (成功)
    Info,
    /// `?` (帮助)
    Help,
    /// `⚠` (警告)
    Warn,
    Wifi,
    WifiNone,
}

impl TryFrom<&IconType> for State {
    type Error = ();

    fn try_from(value: &IconType) -> Result<Self, Self::Error> {
        match value {
            IconType::Info => Ok(Self::Info),
            IconType::Help => Ok(Self::Help),
            IconType::Warn => Ok(Self::Warn),
            IconType::Wifi => Ok(Self::Wifi),
            IconType::WifiNone => Ok(Self::WifiNone),
            _ => Err(()),
        }
    }
}