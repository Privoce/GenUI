use makepad_widgets::*;

use super::IconType;

#[derive(Live, LiveHook, Clone, Debug)]
#[live_ignore]
#[repr(u32)]
pub enum Code {
    #[pick]
    /// `</>` 
    Code,
    /// 试管图标
    Test,
}

impl TryFrom<&IconType> for Code {
    type Error = ();

    fn try_from(value: &IconType) -> Result<Self, Self::Error> {
        match value {
            IconType::Code => Ok(Self::Code),
            IconType::Test => Ok(Self::Test),
            _ => Err(()),
        }
    }
}