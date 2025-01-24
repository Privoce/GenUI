use std::str::FromStr;

use nom::{bytes::complete::tag, sequence::preceded, IResult};

use gen_utils::{
    error::{ConvertError, Error},
    parser::parse_closure_body,
};

pub enum Special {
    MakepadShader,
}

impl Special {
    /// return `(remain, (sign, (name, params, is_style)))`
    pub fn makepad_shader_parser(input: &str) -> IResult<&str, (&str, (&str, &str, Option<bool>))> {
        let (remain, body) = preceded(tag("shader"), parse_closure_body)(input)?;

        Ok((remain, ("()", ("shader", body.trim(), Some(true)))))
    }
}

impl FromStr for Special {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "shader" => Ok(Special::MakepadShader),
            _ => Err(ConvertError::FromTo {
                from: s.to_string(),
                to: "Fn Special".to_string(),
            }
            .into()),
        }
    }
}
