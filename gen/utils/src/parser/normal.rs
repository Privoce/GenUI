use nom::{
    bytes::complete::take_while_m_n,
    character::complete::{alpha1, multispace0},
    combinator::recognize,
    sequence::{delimited, pair},
    IResult,
};

/// ## normal parser for easy string and split string
/// depend on what split sign
pub fn parse_normal(input: &str, sign: char) -> IResult<&str, &str> {
    recognize(pair(
        alpha1,
        take_while_m_n(0, usize::MAX, |c: char| c == sign || c.is_alphanumeric()),
    ))(input)
}

/// ## ⚡️ parse normal value 🆗
/// use in property | value | script variable name
/// - parse xxx
/// - parse xxx_zzz
pub fn parse_value(input: &str) -> IResult<&str, &str> {
    parse_normal(input, '_')
}

/// ## trim any parser left and right multispace(if exist)
#[allow(unused_mut)]
pub fn trim<'a, P, O>(mut parser: P) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    P: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(multispace0, parser, multispace0)
}


#[cfg(test)]
mod normal {
    use super::*;
    #[test]
    fn test_parse_value() {
        let simple = "test";
        let complex = "test_input";
        let more = "test_input_value";
        let num = "123";
        let num_with = "1asd_sd";
        let with_num = "asd_123";
        let res1 = parse_value(simple).unwrap();
        let res2 = parse_value(complex).unwrap();
        let res3 = parse_value(more).unwrap();
        let res4 = parse_value(num);
        let res5 = parse_value(num_with);
        let res6 = parse_value(with_num).unwrap();
        assert_eq!(res1, ("", "test"));
        assert_eq!(res2, ("", "test_input"));
        assert_eq!(res3, ("", "test_input_value"));
        assert!(res4.is_err());
        assert!(res5.is_err());
        assert_eq!(res6, ("", "asd_123"));
    }

    // #[test]
    // fn path() {
    //     let path = "test/test1/test2";
    //     let res = parse_path(path).unwrap();
    //     assert_eq!(res, ("", "test/test1/test2"));
    // }
}
