use std::{fmt::Display, time::{SystemTime, UNIX_EPOCH}};

use rand::{rng, Rng};

/// # Ulid
#[derive(Clone, Debug, PartialEq)]
pub struct Ulid(pub String);

impl Ulid {
    pub fn new() -> Self {
        Ulid(ulid())
    }
    /// convert to snake case
    /// Ulid always is like: 01J55FKXJ1J2ETBD5WR3R4N8S7
    /// so we need to convert it to snake case
    /// 01J55FKXJ1J2ETBD5WR3R4N8S7 -> 01_j55_fkxj1_j2_etbd5_wr3_r4_n8_s7
    /// we can find the rule is:
    /// 1. no `_` before number
    /// 2. `_` before lower case
    /// 3. number together then `_` then lower case
    /// 4. lower case with number together then if next is alpha then add `_`
    pub fn to_snake(&self) -> String {
        // read byte to byte
        let mut result = String::new();
        let mut pre = 0;
        for (index, b) in self.0.to_ascii_lowercase().as_bytes().iter().enumerate() {
            // index == 0 is the first byte, we donot need to care
            if index == 0 {
                result.push(*b as char);
                pre = *b;
                continue;
            } else {
                // let's check the pre byte, if current byte is number, donot add `_`, if is alpha, add `_`
                if b.is_ascii_digit() {
                    pre = *b;
                    result.push(*b as char);
                    continue;
                }
                if b.is_ascii_lowercase() && pre.is_ascii_digit() {
                    result.push('_');
                    result.push(*b as char);
                    pre = *b;
                } else {
                    result.push(*b as char);
                    pre = *b;
                }
            }
        }
        result
    }
}

impl From<&str> for Ulid {
    fn from(value: &str) -> Self {
        Ulid(value.to_string())
    }
}

impl Display for Ulid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}


/// 生成组件的ulid, 用于生成组件的唯一标识, 经过测试无法使用ulid::Ulid直接生成
/// 限制: 
/// 1. 需要生成唯一标识必须以英文字母开头
/// 2. 标识长度必须大于等于6,小于等于12, 以防止生成的标识过短或过长
/// 3. 标识中只能包含数字和英文字母, 不能包含特殊字符
pub fn ulid() -> String {
    // 使用当前时间戳作为部分种子
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();
    
    // 创建一个随机数生成器
    let mut rng = rng();
    
    // 生成一个随机字母作为开头
    let first_char = (rng.random_range(0..12) + b'A') as char;
    
    // 转换时间戳为Base32编码（用0-9A-Z表示）
    let mut result = String::with_capacity(12);
    result.push(first_char);
    
    // 添加时间戳部分（确保至少有10个字符）
    let timestamp_part = encode_base32(timestamp);
    result.push_str(&timestamp_part);
    
    // 生成随机字符填充剩余部分，最长到12个字符
    let remaining = 12 - result.len();
    if remaining > 0 {
        for _ in 0..remaining {
            let random_char = if rng.random_bool(0.5) {
                // 生成随机数字
                (rng.random_range(0..10) + b'0') as char
            } else {
                // 生成随机大写字母
                (rng.random_range(0..12) + b'A') as char
            };
            result.push(random_char);
        }
    }
    
    // 确保长度满足要求
    if result.len() < 6 {
        // 如果长度不足6，添加随机字符到6位
        while result.len() < 6 {
            let random_char = if rng.random_bool(0.5) {
                (rng.random_range(0..10) + b'0') as char
            } else {
                (rng.random_range(0..12) + b'A') as char
            };
            result.push(random_char);
        }
    } else if result.len() > 12 {
        // 如果长度超过12，截断到12位
        result.truncate(12);
    }
    
    result
}

// 将数字编码为Base32字符串（只使用0-9和A-Z）
fn encode_base32(mut value: u128) -> String {
    const CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut result = String::new();
    
    while value > 0 {
        let remainder = (value % 36) as usize;
        result.insert(0, CHARS[remainder] as char);
        value /= 36;
    }
    
    result
}

#[cfg(test)]
mod test_ulid {
    use crate::common::string::FixedString;

    #[test]
    fn to_snake() {
        let ulid = super::Ulid(super::ulid());
        dbg!(ulid.to_snake());
    }
    #[test]
    fn snake_to_camel(){
        let a = "IfWidget01J55FKXJ1J2ETBD5WR3R4N8S7";
        
        dbg!(a.camel_to_snake_ulid(Some("IfWidget")));
    }
}
