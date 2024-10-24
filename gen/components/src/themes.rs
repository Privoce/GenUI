use std::fmt::Display;

use makepad_widgets::*;

use crate::color_v_trait;

#[derive(Copy, Clone, Debug, Live, LiveHook)]
#[live_ignore]
pub enum Themes {
    Dark,
    #[pick]
    Primary,
    Error,
    Warning,
    Success,
    Info,
}

impl Default for Themes {
    fn default() -> Self {
        Themes::Primary
    }
}

impl ToLiveValue for Themes {
    fn to_live_value(&self) -> LiveValue {
        match self {
            Themes::Dark => LiveValue::BareEnum(live_id!(Dark)),
            Themes::Primary => LiveValue::BareEnum(live_id!(Primary)),
            Themes::Error => LiveValue::BareEnum(live_id!(Error)),
            Themes::Warning => LiveValue::BareEnum(live_id!(Warning)),
            Themes::Success => LiveValue::BareEnum(live_id!(Success)),
            Themes::Info => LiveValue::BareEnum(live_id!(Info)),
        }
    }
}

impl Themes {
    pub fn get(&self, v: u32) -> Vec4 {
        match self {
            Themes::Dark => ThemeDark::v(v),
            Themes::Primary => ThemePrimary::v(v),
            Themes::Error => ThemeError::v(v),
            Themes::Warning => ThemeWarning::v(v),
            Themes::Success => ThemeSuccess::v(v),
            Themes::Info => ThemeInfo::v(v),
        }
    }
    pub fn hex(&self, v: u32) -> &'static str {
        match self {
            Themes::Dark => ThemeDark::hex(v),
            Themes::Primary => ThemePrimary::hex(v),
            Themes::Error => ThemeError::hex(v),
            Themes::Warning => ThemeWarning::hex(v),
            Themes::Success => ThemeSuccess::hex(v),
            Themes::Info => ThemeInfo::hex(v),
        }
    }
    pub fn to_vec(&self) -> Vec<Vec4> {
        let levels = vec![25, 50, 100, 200, 300, 400, 500, 600, 700, 800, 900];
        levels.iter().map(|v| self.get(*v)).collect()
    }
}

impl Display for Themes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Themes::Dark => write!(f, "Dark"),
            Themes::Primary => write!(f, "Primary"),
            Themes::Error => write!(f, "Error"),
            Themes::Warning => write!(f, "Warning"),
            Themes::Success => write!(f, "Success"),
            Themes::Info => write!(f, "Info"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ThemeColor {
    Dark(ThemeDark),
    Primary(ThemePrimary),
    Error(ThemeError),
    Warning(ThemeWarning),
    Success(ThemeSuccess),
    Info(ThemeInfo),
}

pub trait ThemeColorValue: Default {
    fn v(target: u32) -> Vec4;
    fn get(&self) -> Vec4;
    fn hex(target: u32) -> &'static str;
}

// -------- color-dark -----------------------------------------
// COLOR_DARK_25 = #6e7176;
// COLOR_DARK_50 = #5b5f64;
// COLOR_DARK_100 = #42464d;
// COLOR_DARK_200 = #3b4047;
// COLOR_DARK_300 = #2f333b;
// COLOR_DARK_400 = #282d35;
// COLOR_DARK_500 = #22272F;
// COLOR_DARK_600 = #1f242b;
// COLOR_DARK_700 = #1d2127;
// COLOR_DARK_800 = #1a1e24;
// COLOR_DARK_900 = #0f1115;
#[derive(Debug, Clone)]
pub struct ThemeDark(Vec4);

impl Default for ThemeDark {
    fn default() -> Self {
        Self(hex_to_vec4(Self::_500))
    }
}

impl ThemeDark {
    pub const _25: &'static str = "#6e7176";
    pub const _50: &'static str = "#5b5f64";
    pub const _100: &'static str = "#42464d";
    pub const _200: &'static str = "#3b4047";
    pub const _300: &'static str = "#2f333b";
    pub const _400: &'static str = "#282d35";
    pub const _500: &'static str = "#22272F";
    pub const _600: &'static str = "#1f242b";
    pub const _700: &'static str = "#1d2127";
    pub const _800: &'static str = "#1a1e24";
    pub const _900: &'static str = "#0f1115";
}

// -------- color-primary --------------------------------------
// COLOR_PRIMARY_25 = #F5FEFF;
// COLOR_PRIMARY_50 = #ECFDFF;
// COLOR_PRIMARY_100 = #CFF9FE;
// COLOR_PRIMARY_200 = #A5F0FC;
// COLOR_PRIMARY_300 = #67E3F9;
// COLOR_PRIMARY_400 = #22CCEE;
// COLOR_PRIMARY_500 = #06AED4;
// COLOR_PRIMARY_600 = #088AB2;
// COLOR_PRIMARY_700 = #0E6F90;
// COLOR_PRIMARY_800 = #155B75;
// COLOR_PRIMARY_900 = #164C63;
// default:  vec3(0.023529412,0.68235296,0.83137256)
#[derive(Debug, Clone)]
pub struct ThemePrimary(Vec4);

impl Default for ThemePrimary {
    fn default() -> Self {
        Self(hex_to_vec4(Self::_500))
    }
}

impl ThemePrimary {
    pub const _25: &'static str = "#F5FEFF";
    pub const _50: &'static str = "#ECFDFF";
    pub const _100: &'static str = "#CFF9FE";
    pub const _200: &'static str = "#A5F0FC";
    pub const _300: &'static str = "#67E3F9";
    pub const _400: &'static str = "#22CCEE";
    pub const _500: &'static str = "#06AED4";
    pub const _600: &'static str = "#088AB2";
    pub const _700: &'static str = "#0E6F90";
    pub const _800: &'static str = "#155B75";
    pub const _900: &'static str = "#164C63";
}

// -------- color-error ------------------------------------
// COLOR_ERROR_25 = #FFFBFA;
// COLOR_ERROR_50 = #FEF3F2;
// COLOR_ERROR_100 = #FEE4E2;
// COLOR_ERROR_200 = #FECDCA;
// COLOR_ERROR_300 = #FDA29B;
// COLOR_ERROR_400 = #F97066;
// COLOR_ERROR_500 = #F04438;
// COLOR_ERROR_600 = #D92D2D;
// COLOR_ERROR_700 = #B42318;
// COLOR_ERROR_800 = #912018;
// COLOR_ERROR_900 = #7A271A;
#[derive(Debug, Clone)]
pub struct ThemeError(Vec4);

impl Default for ThemeError {
    fn default() -> Self {
        Self(hex_to_vec4(Self::_500))
    }
}

impl ThemeError {
    pub const _25: &'static str = "#FFFBFA";
    pub const _50: &'static str = "#FEF3F2";
    pub const _100: &'static str = "#FEE4E2";
    pub const _200: &'static str = "#FECDCA";
    pub const _300: &'static str = "#FDA29B";
    pub const _400: &'static str = "#F97066";
    pub const _500: &'static str = "#F04438";
    pub const _600: &'static str = "#D92D2D";
    pub const _700: &'static str = "#B42318";
    pub const _800: &'static str = "#912018";
    pub const _900: &'static str = "#7A271A";
}

// -------- color-warning ------------------------------------
// COLOR_WARNING_25 = #FFFCF5;
// COLOR_WARNING_50 = #FFFAEB;
// COLOR_WARNING_100 = #FEF0C7;
// COLOR_WARNING_200 = #FEDF89;
// COLOR_WARNING_300 = #FEC84B;
// COLOR_WARNING_400 = #FDB022;
// COLOR_WARNING_500 = #F79009;
// COLOR_WARNING_600 = #DC6803;
// COLOR_WARNING_700 = #B54708;
// COLOR_WARNING_800 = #93370D;
// COLOR_WARNING_900 = #7A2E0E;
#[derive(Debug, Clone)]
pub struct ThemeWarning(Vec4);

impl Default for ThemeWarning {
    fn default() -> Self {
        Self(hex_to_vec4(Self::_500))
    }
}

impl ThemeWarning {
    pub const _25: &'static str = "#FFFCF5";
    pub const _50: &'static str = "#FFFAEB";
    pub const _100: &'static str = "#FEF0C7";
    pub const _200: &'static str = "#FEDF89";
    pub const _300: &'static str = "#FEC84B";
    pub const _400: &'static str = "#FDB022";
    pub const _500: &'static str = "#F79009";
    pub const _600: &'static str = "#DC6803";
    pub const _700: &'static str = "#B54708";
    pub const _800: &'static str = "#93370D";
    pub const _900: &'static str = "#7A2E0E";
}

// -------- color-success ------------------------------------
// COLOR_SUCCESS_25 = #F6FEF9;
// COLOR_SUCCESS_50 = #ECFDF3;
// COLOR_SUCCESS_100 = #D1FADF;
// COLOR_SUCCESS_200 = #A6F4C5;
// COLOR_SUCCESS_300 = #6CE9A6;
// COLOR_SUCCESS_400 = #32D583;
// COLOR_SUCCESS_500 = #12B76A;
// COLOR_SUCCESS_600 = #039855;
// COLOR_SUCCESS_700 = #027A48;
// COLOR_SUCCESS_800 = #05603A;
// COLOR_SUCCESS_900 = #054F31;
#[derive(Debug, Clone)]
pub struct ThemeSuccess(Vec4);

impl Default for ThemeSuccess {
    fn default() -> Self {
        Self(hex_to_vec4(Self::_500))
    }
}

impl ThemeSuccess {
    pub const _25: &'static str = "#F6FEF9";
    pub const _50: &'static str = "#ECFDF3";
    pub const _100: &'static str = "#D1FADF";
    pub const _200: &'static str = "#A6F4C5";
    pub const _300: &'static str = "#6CE9A6";
    pub const _400: &'static str = "#32D583";
    pub const _500: &'static str = "#12B76A";
    pub const _600: &'static str = "#039855";
    pub const _700: &'static str = "#027A48";
    pub const _800: &'static str = "#05603A";
    pub const _900: &'static str = "#054F31";
}

// COLOR_INFO_25 = #FCFCFD;
// COLOR_INFO_50 = #F9FAFB;
// COLOR_INFO_100 = #F2F4F7;
// COLOR_INFO_200 = #EAECF0;
// COLOR_INFO_300 = #D0D5DD;
// COLOR_INFO_400 = #95A2D3;
// COLOR_INFO_500 = #667085;
// COLOR_INFO_600 = #475467;
// COLOR_INFO_700 = #344054;
// COLOR_INFO_800 = #1D2939;
// COLOR_INFO_900 = #101828;
#[derive(Debug, Clone)]
pub struct ThemeInfo(Vec4);

impl Default for ThemeInfo {
    fn default() -> Self {
        Self(hex_to_vec4(Self::_500))
    }
}

impl ThemeInfo {
    pub const _25: &'static str = "#FCFCFD";
    pub const _50: &'static str = "#F9FAFB";
    pub const _100: &'static str = "#F2F4F7";
    pub const _200: &'static str = "#EAECF0";
    pub const _300: &'static str = "#D0D5DD";
    pub const _400: &'static str = "#95A2D3";
    pub const _500: &'static str = "#667085";
    pub const _600: &'static str = "#475467";
    pub const _700: &'static str = "#344054";
    pub const _800: &'static str = "#1D2939";
    pub const _900: &'static str = "#101828";
}

color_v_trait!(ThemePrimary);
color_v_trait!(ThemeError);
color_v_trait!(ThemeSuccess);
color_v_trait!(ThemeWarning);
color_v_trait!(ThemeInfo);
color_v_trait!(ThemeDark);

pub fn hex_to_vec4(hex: &str) -> Vec4 {
    // 去掉开头的 '#' 符号
    let hex = hex.trim_start_matches('#');

    // 解析 RGB 值
    let (r, g, b, a) = if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
        (r, g, b, 255)
    } else if hex.len() == 8 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
        let a = u8::from_str_radix(&hex[6..8], 16).unwrap();
        (r, g, b, a)
    } else {
        panic!("invalid hex color: {}", hex);
    };

    Vec4 {
        x: r as f32 / 255.0,
        y: g as f32 / 255.0,
        z: b as f32 / 255.0,
        w: a as f32 / 255.0,
    }
}

/// v: color value, range: `[25, 50, 100 ,200, 300, 400, 500, 600, 700, 800, 900]`
pub fn get_color(theme: Themes, color: Option<&Vec4>, v: u32) -> Vec4 {
    return if let Some(target) = color {
        target.clone()
    } else {
        match theme {
            Themes::Dark => ThemeDark::v(v),
            Themes::Primary => ThemePrimary::v(v),
            Themes::Error => ThemeError::v(v),
            Themes::Warning => ThemeWarning::v(v),
            Themes::Success => ThemeSuccess::v(v),
            Themes::Info => ThemeInfo::v(v),
        }
    };
}

#[cfg(test)]
mod test_themes {
    use crate::themes::hex_to_vec4;
    #[test]
    fn v4() {
        let v = hex_to_vec4("#F4F4F6");
        dbg!(v);
    }
}
