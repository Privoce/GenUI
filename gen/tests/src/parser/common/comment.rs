#[cfg(test)]
mod comment_test {
    // use gen_parser::{comment::position::OfflinePosition, Targets};


    // #[test]
    // fn offline_comment_empty() {
    //     let input = r#"
    //         <template></template>
    //     "#;
    //     let success = match parse_offline_comment(input) {
    //         Ok(_) => false,
    //         Err(_) => true,
    //     };
    //     assert!(success);
    // }

    // #[test]
    // fn offline_comment_above_template() {
    //     let input = r#"
    //         //! This is File comment
    //         //! This is a comment for a file
    //         /// This is doc comment
    //         // normal comment
    //         <template></template>
    //     "#;

    //     // let (a,b) = many0(alt((parse_offline_comment,trim(tag("<template>")))))(input).unwrap();
    //     let (remain, comment) = parse_offline_comment(input).unwrap();
    //     assert_eq!(remain, "\n            <template></template>\n        ");

    //     match comment {
    //         Targets::Comment(c) => {
    //             assert_eq!(c.position(), OfflinePosition::AboveTemplate)
    //         }
    //         _ => panic!("Invalid"),
    //     }
    // }

    // #[test]
    // fn offline_comment_above_script() {
    //     let input = r#"
    //         //! This is File comment
    //         //! This is a comment for a file
    //         /// This is doc comment
    //         // normal comment
    //         <script></script>
    //     "#;

    //     let (_remain, comment) = parse_offline_comment(input).unwrap();
    //     match comment {
    //         Targets::Comment(c) => {
    //             assert_eq!(c.position(), OfflinePosition::AboveScript)
    //         }
    //         _ => panic!("Invalid"),
    //     }
    // }

    // #[test]
    // fn offline_comment_above_style() {
    //     let input = r#"
    //         //! This is File comment
    //         //! This is a comment for a file
    //         /// This is doc comment
    //         // normal comment
    //         <style></style>
    //     "#;

    //     let (_remain, comment) = parse_offline_comment(input).unwrap();
    //     match comment {
    //         Targets::Comment(c) => {
    //             assert_eq!(c.position(), OfflinePosition::AboveStyle)
    //         }
    //         _ => panic!("Invalid"),
    //     }
    // }

    // #[test]
    // fn offline_comment_end() {
    //     let input = r#"
    //         //! This is File comment
    //         //! This is a comment for a file
    //         /// This is doc comment
    //         // normal comment
    //     "#;
    //     let (_remain, comment) = parse_offline_comment(input).unwrap();
    //     match comment {
    //         Targets::Comment(c) => assert_eq!(c.position(), OfflinePosition::End),
    //         _ => panic!("Invalid"),
    //     }
    // }
}
