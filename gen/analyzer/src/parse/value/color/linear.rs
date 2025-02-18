use crate::value::Function;
use gen_utils::error::{Error, ParseError, ParseType};
use std::{fmt::Display, str::FromStr};

use super::{Hex, Percentage};
/// 语法: `linear_gradient(angle, color percentage, color percentage, ...)`
#[derive(Debug, Clone, PartialEq)]
pub struct LinearGradient {
    pub angle: f32,
    pub colors: Vec<(Hex, Percentage)>,
}

impl TryFrom<&Function> for LinearGradient {
    type Error = Error;

    fn try_from(value: &Function) -> Result<Self, Self::Error> {
        fn trans_angle(v: &str) -> Result<f32, Error> {
            v.trim_end_matches("deg").parse::<f32>().map_err(|_| {
                let mut err = ParseError::new(v, ParseType::Color("linear_gradient".to_string()));
                let _ = err.set_other("angle must be number, format as: `180.5deg`");
                Error::Parse(err)
            })
        }

        // 检查是否fn的名称叫linear_gradient
        if value.name.eq("linear_gradient") {
            // linear的参数至少有3个
            if let Some(params) = value.params.as_ref() {
                let len = params.len();
                if len >= 3 {
                    // 第一个参数是角度
                    let angle = trans_angle(&params[0].to_string())?;
                    // 其他参数是颜色
                    let mut colors: Vec<(Hex, Percentage, bool)> = vec![];
                    for i in 1..len {
                        colors.push(trans_hex_percentage(&params[i].to_string(), i - 1, len - 1)?);
                    }
                    let colors = check_and_fix(&mut colors);
                    return Ok(LinearGradient { angle, colors });
                }
            }
            let mut err = ParseError::new(
                &value.name,
                ParseType::Color("linear_gradient".to_string()),
            );
            let _ = err.set_other("linear_gradient fn need at least three params `(angle, color percentage, color percentage, ...)`");
            return Err(err.into());
        }
        Err(ParseError::new(
            &value.name,
            ParseType::Color("linear_gradient".to_string()),
        )
        .into())
    }
}

/// 将颜色转换为Hex和Percentage
/// 返回的最后一个参数表示是否是通过offset和len计算出来的百分比
/// 当为false时，需要将当前的索引和占比放入更正器中
/// 更正器根据索引和占比向前找出所有的颜色，然后重新计算占比
pub fn trans_hex_percentage(
    v: &str,
    offset: usize,
    len: usize,
) -> Result<(Hex, Percentage, bool), Error> {
    // 通过空格分割，无论第二个占比的百分比参数是否存在，第一个参数都是颜色，直接使用Hex::from_str
    let mut hex_percentage = v.trim().split_whitespace();
    let hex = Hex::from_str(hex_percentage.next().unwrap())?;
    // 第二个百分比可能没有
    match hex_percentage.next() {
        Some(percentage) => {
            let percentage = Percentage::from_str(percentage)?;
            Ok((hex, percentage, false))
        }
        None => {
            // 通过offset和len确定是第几个颜色参数, offset是从0开始的索引
            // 例如offset = 2， len = 4 表示第3个颜色参数，但总共有4个颜色参数，那么占比就是100 / (len - 1) * offset
            let len = (len - 1) as f32;
            let offset = offset as f32;
            let percentage = Percentage::from_str(&format!("{}%", 100.0 / len * offset))?;
            Ok((hex, percentage, true))
        }
    }
}

/// ## 检查并修复百分比(用于linear_gradient和radial_gradient)
/// 检查当前数组中是否有false，如果有，那么需要更正false之前的颜色占比, 需要忽略第一个和最后一个
pub fn check_and_fix(colors: &mut Vec<(Hex, Percentage, bool)>) -> Vec<(Hex, Percentage)> {
    // 修复默认计算出来的百分比
    if colors.len() > 2 {
        for i in 1..(colors.len() - 1) {
            if !colors[i].2 {
                continue;
            }

            let mut start = 0;
            let mut end = colors.len() - 1;

            // 找到最近的前一个非默认百分比
            for j in (0..i).rev() {
                if !colors[j].2 {
                    start = j;
                    break;
                }
            }

            // 找到最近的后一个非默认百分比
            for j in (i + 1)..colors.len() {
                if !colors[j].2 {
                    end = j;
                    break;
                }
            }
            let start_percentage = colors[start].1 .0;
            let end_percentage = colors[end].1 .0;

            for j in (start + 1)..end {
                if colors[j].2 {
                    colors[j]
                        .1
                        .fix(start_percentage, end_percentage, j, end - start);
                }
            }
        }
    }

    colors
        .iter()
        .map(|(hex, percentage, _)| (hex.clone(), *percentage))
        .collect()
}

impl Display for LinearGradient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "linear_gradient({}, {})",
            self.angle,
            self.colors
                .iter()
                .map(|(hex, percentage)| format!("{}, {}", hex, percentage))
                .collect::<Vec<String>>()
                .join(", ")
        ))
    }
}
