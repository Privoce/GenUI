use std::fmt::Display;

/// # Ulid
/// use ulid crate to generate ulid
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

pub fn ulid() -> String {
    ulid::Ulid::new().to_string()
}

#[cfg(test)]
mod test_ulid {
    use crate::common::string::FixedString;

    #[test]
    fn to_snake() {
        let ulid = super::Ulid("01J55FKXJ1J2ETBD5WR3R4N8S7".to_string());
        assert_eq!(ulid.to_snake(), "01_j55_fkxj1_j2_etbd5_wr3_r4_n8_s7");
    }
    #[test]
    fn snake_to_camel(){
        let a = "IfWidget01J55FKXJ1J2ETBD5WR3R4N8S7";
        
        dbg!(a.camel_to_snake_ulid(Some("IfWidget")));
    }
}
