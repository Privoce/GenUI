use makepad_widgets::*;

use crate::error::GError;

use super::IconType;

#[derive(Live, LiveHook, Clone, Debug)]
#[live_ignore]
#[repr(u32)]
pub enum Code {
    #[pick]
    /// `</>` 
    Code = shader_enum(1),
    /// 试管图标
    Test,
    Debug,
}

impl TryFrom<&IconType> for Code {
    type Error = GError;

    fn try_from(value: &IconType) -> Result<Self, Self::Error> {
        match value {
            IconType::Code => Ok(Self::Code),
            IconType::Test => Ok(Self::Test),
            IconType::Debug => Ok(Self::Debug),
            _ => Err(GError::IconTypeTransfom),
        }
    }
}