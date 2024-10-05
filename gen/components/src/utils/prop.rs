use std::{path::PathBuf, str::FromStr};

use makepad_widgets::{DVec2, HeapLiveIdPath, LiveDependency, LiveId, Rect, Vec2, Vec4};

use crate::themes::{get_color, hex_to_vec4, Themes};

use super::from_str_unchecked;

// -------------------------------------------------------------------------------------------------
/// This trait is used to get the color of the theme
pub trait ThemeColor {
    /// Get the color of the theme. if color is none, return the default color
    fn get(&self, theme: Themes, default: u32) -> Vec4;
    fn use_or(&self, hex: &str) -> Vec4;
}

impl ThemeColor for Option<Vec4> {
    fn get(&self, theme: Themes, default: u32) -> Vec4 {
        get_color(theme, self.as_ref(), default)
    }
    fn use_or(&self, hex: &str) -> Vec4 {
        self.unwrap_or_else(|| hex_to_vec4(hex))
    }
}

#[macro_export]
macro_rules! color_v_trait {
    ($T:ty) => {
        impl ThemeColorValue for $T {
            fn v(target: u32) -> Vec4 {
                hex_to_vec4(match target {
                    25 => Self::_25,
                    50 => Self::_50,
                    100 => Self::_100,
                    200 => Self::_200,
                    300 => Self::_300,
                    400 => Self::_400,
                    500 => Self::_500,
                    600 => Self::_600,
                    700 => Self::_700,
                    800 => Self::_800,
                    900 => Self::_900,
                    _ => panic!("invalid target"),
                })
            }

            fn get(&self) -> Vec4 {
                self.0
            }
        }
    };
}
// -------------------------------------------------------------------------------------------------
/// This trait is used to transform f32/f64 to bool
pub trait ToBool {
    /// Transform f32/f64 to bool
    fn to_bool(&self) -> bool;
}

impl ToBool for f32 {
    fn to_bool(&self) -> bool {
        *self != 0.0
    }
}

pub trait BoolToF32 {
    /// Transform bool to f32/f64
    fn to_f32(&self) -> f32;
}

impl BoolToF32 for bool {
    fn to_f32(&self) -> f32 {
        *self as u8 as f32
    }
}

// --------------------------------------------------------------------------------------------------
pub trait Render {
    fn render(&mut self);
}

// ---------------------------------------------------------------------------------------------------
pub trait ToPath {
    fn to_pathbuf(&self) -> PathBuf;
}

impl ToPath for LiveDependency {
    fn to_pathbuf(&self) -> PathBuf {
        PathBuf::from_str(self.as_str()).unwrap()
    }
}

// ----------------------------------------------------------------------------------------------------
pub trait RectExpand {
    fn abs_start(&mut self, by: &Rect, offset: Option<DVec2>) -> ();
    fn abs_start_center(&mut self, by: &Rect, offset: Option<DVec2>) -> ();
    fn abs_end(&mut self, by: &Rect, offset: Option<DVec2>) -> ();
    fn abs_end_center(&mut self, by: &Rect, offset: Option<DVec2>) -> ();
}

impl RectExpand for Rect {
    fn abs_start(&mut self, by: &Rect, offset: Option<DVec2>) -> () {
        let Rect { pos, .. } = by;

        if let Some(v) = offset {
            self.pos.x = pos.x + v.x;
            self.pos.y = pos.y + v.y;
        } else {
            self.pos.x = pos.x;
            self.pos.y = pos.y;
        };
    }

    fn abs_start_center(&mut self, by: &Rect, offset: Option<DVec2>) -> () {
        let Rect { pos, size } = by;

        if let Some(v) = offset {
            self.pos.x = pos.x + v.x;
            self.pos.y = pos.y + v.y + size.y * 0.5 - self.size.y * 0.5;
        } else {
            self.pos.x = pos.x;
            self.pos.y = pos.y + size.y * 0.5 - self.size.y * 0.5;
        };
    }

    fn abs_end(&mut self, by: &Rect, offset: Option<DVec2>) -> () {
        let Rect { pos, size } = by;

        if let Some(v) = offset {
            self.pos.x = pos.x + v.x + size.x;
            self.pos.y = pos.y + v.y;
        } else {
            self.pos.x = pos.x;
            self.pos.y = pos.y;
        };
    }

    fn abs_end_center(&mut self, by: &Rect, offset: Option<DVec2>) -> () {
        let Rect { pos, size } = by;

        if let Some(v) = offset {
            self.pos.x = pos.x + v.x + size.x;
            self.pos.y = pos.y + v.y + size.y * 0.5 - self.size.y * 0.5;
        } else {
            self.pos.x = pos.x + size.x;
            self.pos.y = pos.y + size.y * 0.5 - self.size.y * 0.5;
        };
    }
}

// ------------------------------------------------------------------------------------------------------------

pub trait ToDVec {
    fn to_dvec2(self) -> DVec2;
}

impl ToDVec for Vec2 {
    fn to_dvec2(self) -> DVec2 {
        DVec2 {
            x: self.x as f64,
            y: self.y as f64,
        }
    }
}

pub trait HeapLiveIdPathExp {
    // body.navigation.application_pages.upload_frame.UniqueId 3.s3_list.UniqueId 3.UniqueId 1.share_wrap
    fn contains(&self, child: &HeapLiveIdPath) -> Result<bool, String>;
    fn contains_id(&self, id: &LiveId) -> bool;
    fn to_live_id(&self) -> Vec<LiveId>;
    fn trim_matches(&self, target: &HeapLiveIdPath) -> Vec<LiveId>;
    fn eq(&self, target: &HeapLiveIdPath) -> bool;
    fn is_empty(&self) -> bool;
    fn to_vec_str(&self) -> Vec<String>;
    fn to_string(&self) -> String;
}

impl HeapLiveIdPathExp for HeapLiveIdPath {
    fn to_vec_str(&self) -> Vec<String> {
        format!("{:?}", self)
            .split(".")
            .map(|x| x.to_string())
            .collect()
    }
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
    fn contains(&self, child: &HeapLiveIdPath) -> Result<bool, String> {
        // do format then split by `.`
        let father = format!("{:?}", self);
        let child = format!("{:?}", child);

        let father = father.split('.').collect::<Vec<&str>>();
        let child = child.split('.').collect::<Vec<&str>>();
        // eat one by one till `UniqueId`

        if father.len() < child.len() {
            return Err("father LiveIdPath length smaller than child".to_string());
        }

        let mut flag = true;
        for (index, c_p) in child.iter().enumerate() {
            // let f_p = if father[index].starts_with("UniqueId") {
            //     father[index].trim_start_matches("UniqueId ")
            // } else {
            //     father[index]
            // };
            // dbg!(c_p, f_p);

            if *c_p != father[index] {
                flag = false;
                break;
            }
        }
        Ok(flag)
    }

    /// not complete!!!
    fn to_live_id(&self) -> Vec<LiveId> {
        let path = format!("{:?}", self);
        path.split('.')
            .map(|x| LiveId(from_str_unchecked(x)))
            .collect()
    }

    fn trim_matches(&self, target: &HeapLiveIdPath) -> Vec<LiveId> {
        format!("{:?}", self)
            .trim_start_matches(&format!("{:?}", target))
            .split('.')
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| LiveId(from_str_unchecked(x.trim_matches('.'))))
            .collect()
    }

    fn eq(&self, target: &HeapLiveIdPath) -> bool {
        format!("{:?}", self) == format!("{:?}", target)
    }

    fn contains_id(&self, id: &LiveId) -> bool {
        format!("{:?}", self).contains(&id.to_string())
    }

    fn is_empty(&self) -> bool {
        format!("{:?}", self).is_empty()
    }
}

pub trait LiveIdExp {
    fn as_slice(&self) -> &[LiveId];
}

impl LiveIdExp for LiveId {
    fn as_slice(&self) -> &[LiveId] {
        std::slice::from_ref(self)
    }
}
