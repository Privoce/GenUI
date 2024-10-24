use makepad_widgets::*;
use crate::error::GError;

use super::IconType;

#[derive(Live, LiveHook, Clone, Debug)]
#[live_ignore]
#[repr(u32)]
pub enum UI {
    #[pick]
    /// ```
    /// ---
    /// |  > (退出)
    /// ---
    /// ```
    Exit = shader_enum(1),
    /// `<>` (展开)
    Expand,
    /// ```
    /// -------
    /// |-----|  (展开上边)
    /// |     |
    /// -------
    /// ```
    ExpandTop,
    /// ```
    /// -------
    /// |     |
    /// |-----|  (展开下边)
    /// -------
    /// ```
    ExpandBottom,
    /// ```
    /// ---------
    /// |  |    |  (展开左边)
    /// |  |    |
    /// ---------
    /// ```
    ExpandLeft,
    /// ```
    /// ---------
    /// |    |  |  (展开右边)
    /// |    |  |
    /// ---------
    ExpandRight,
    Open,
    OpenLeft,
    OpenRight,
    OpenTop,
    OpenBottom,
    Split,
    Split2,
    Poweroff,
}

impl TryFrom<&IconType> for UI {
    type Error = GError;

    fn try_from(value: &IconType) -> Result<Self, Self::Error> {
        match value {
            IconType::Exit => Ok(Self::Exit),
            IconType::Expand => Ok(Self::Expand),
            IconType::ExpandTop => Ok(Self::ExpandTop),
            IconType::ExpandBottom => Ok(Self::ExpandBottom),
            IconType::ExpandLeft => Ok(Self::ExpandLeft),
            IconType::ExpandRight => Ok(Self::ExpandRight),
            IconType::Open => Ok(Self::Open),
            IconType::OpenLeft => Ok(Self::OpenLeft),
            IconType::OpenRight => Ok(Self::OpenRight),
            IconType::OpenTop => Ok(Self::OpenTop),
            IconType::OpenBottom => Ok(Self::OpenBottom),
            IconType::Split => Ok(Self::Split),
            IconType::Split2 => Ok(Self::Split2),
            IconType::Poweroff => Ok(Self::Poweroff),
            _ => Err(GError::IconTypeTransfom),
        }
    }
}