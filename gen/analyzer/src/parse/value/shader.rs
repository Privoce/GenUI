use std::fmt::Display;

use gen_utils::error::{ConvertError, Error};

use crate::value::Function;

/// MakepadShader
/// 用于提供 Makepad 的着色器, 从BuiltinColor中获取并转化为着色器代码
#[derive(Debug, Clone)]
pub struct MakepadShader(pub String);

impl TryFrom<&Function> for MakepadShader {
    type Error = Error;

    fn try_from(value: &Function) -> Result<Self, Self::Error> {
        if value.name == "shader" {
            let tk = value
                .params
                .as_ref()
                .ok_or_else(|| Error::from("shader function must have params"))?
                .get(0)
                .ok_or_else(|| Error::from("shader function must have one param"))?
                .to_string();

            return Ok(Self(tk));
        }

        return Err(ConvertError::FromTo {
            from: value.name.to_string(),
            to: "Makepad Shader".to_string(),
        }
        .into());
    }
}

impl Display for MakepadShader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}