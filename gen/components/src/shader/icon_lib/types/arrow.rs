use makepad_widgets::*;

use super::IconType;

#[derive(Live, LiveHook, Clone, Debug)]
#[live_ignore]
#[repr(u32)]
pub enum Arrow {
    #[pick]
    /// `<` (左箭头)
    Left,
    /// `>` (右箭头)
    Right,
    /// `︿` (向上)
    Up,
    /// `﹀` (向下)
    Down,
    /// `⇆` (切换)
    Switch,
}

impl TryFrom<&IconType> for Arrow {
    type Error = ();

    fn try_from(value: &IconType) -> Result<Self, Self::Error> {
        match value {
            IconType::Left => Ok(Self::Left),
            IconType::Right => Ok(Self::Right),
            IconType::Up => Ok(Self::Up),
            IconType::Down => Ok(Self::Down),
            IconType::Switch => Ok(Self::Switch),
            _ => Err(()),
        }
    }
}