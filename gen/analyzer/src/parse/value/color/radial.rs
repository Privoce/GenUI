use std::fmt::Display;

use super::{check_and_fix, trans_hex_percentage, Hex, Percentage};
use crate::Function;
use gen_utils::error::{Error, ParseError, ParseType};

/// 语法: `radial_gradient(color percentage, color percentage, ...)`
#[derive(Debug, Clone, PartialEq)]
pub struct RadialGradient {
    pub colors: Vec<(Hex, Percentage)>,
}

impl TryFrom<&Function> for RadialGradient {
    type Error = Error;

    fn try_from(value: &Function) -> Result<Self, Self::Error> {
        // 检查是否fn的名称叫radial_gradient
        if value.name.eq("radial_gradient") {
            // radial的参数至少有2个
            if let Some(params) = value.params.as_ref() {
                let len = params.len();
                if len >= 2 {
                    let mut colors: Vec<(Hex, Percentage, bool)> = vec![];
                    for i in 0..len {
                        colors.push(trans_hex_percentage(&params[i].to_string(), i, len)?);
                    }
                    let colors = check_and_fix(&mut colors);
                    return Ok(RadialGradient { colors });
                }
            }
            let mut err = ParseError::new(&value.name, ParseType::Color("radial_gradient".to_string()));
            let _ = err.set_other("radial_gradient fn need at least two params `(color percentage, color percentage, ...)`");
            return Err(err.into());
        }
        Err(ParseError::new(&value.name, ParseType::Color("radial_gradient".to_string())).into())
    }
}

impl Display for RadialGradient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for (hex, percentage) in &self.colors {
            s.push_str(&format!("{}, {} ", hex, percentage));
        }
        write!(f, "radial_gradient({})", s)
    }
}
