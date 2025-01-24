mod result;

#[cfg(test)]
mod ast_test {
    // use std::{fs::File, io::Write};

    use std::{fs::File, io::Write};

    use gen_parser::{comment::{offline::OfflineComment, position::OfflinePosition, Comments}, ParseTarget};


    #[test]
    fn parse_t_s() {
        let input = r#"
        <template>
            <window id="ui">
                <label text="Hello"></label>
            </window>
        </template>
        <style>
        #ui{
            width: Fill;
            height: Fill;
            show_bg: true;
            draw_bg: #1C2128
        }
        </style>
        "#;

        let target = ParseTarget::try_from(input).unwrap();
        dbg!(target);
    }

    #[test]
    fn parse_target() {
        let input = r#"
        //! This is a comment1
        //! This is a comment2
        //! This is a comment3
        <template>
            <window class="ui">
            </window>
        </template>

        // This is line comment
        /// This is a doc comment
        /// hello
        <script>
        let mut counter:usize = 0

        let handle_actions:FnOnce()->() = || {
            counter += 1;
        }
        </script>

        // This is line comment2
        <style>
        .ui{
            height : fill;
            width : fill;
            show_bg : true;
        }
        </style>
        // end of line comment
        "#;

        let target = ParseTarget::try_from(input).unwrap();
        let mut parse = ParseTarget::default();
        parse.set_template("<window class=\"ui\">\n            </window>\n        ");
        let _ = parse.set_script("let mut counter:usize = 0\n\n        let handle_actions:FnOnce()->() = || {\n            counter += 1;\n        }\n        ", Some("rust".to_string()));
        parse.set_style(".ui{\n            height : fill;\n            width : fill;\n            show_bg : true;\n        }\n        ");
        parse.set_comment(vec![OfflineComment::from((
            vec![Comments::File("This is a comment1".to_string())],
            OfflinePosition::AboveTemplate,
        ))]);
        assert_eq!(target, parse);
    }

    #[test]
    fn parse_empty() {
        let input = r#"
       
        "#;
        let target = ParseTarget::try_from(input);
        assert_eq!(target.unwrap().to_string().as_str(), "\n");
    }

    #[test]
    fn parse_only_code() {
        let input = r#"
        let a:&str = "trest";
        "#;
        let target = ParseTarget::try_from(input);
        assert!(target.is_err());
    }

    #[test]
    fn display() {
        let input = r#"
        //! This is a comment1
        //! This is a comment2
        //! This is a comment3
        <template>
            <window class="ui">
            </window>
        </template>

        // This is line comment
        /// This is a doc comment
        /// hello
        <script>
        let mut counter:usize = 0

        let handle_actions:FnOnce()->() = || {
            counter += 1;
        }
        </script>

        // This is line comment2
        // end of line comment
        "#;

        let target = ParseTarget::try_from(input).unwrap();
        let mut f =
            File::create("/Users/user/Downloads/beyond-framework-main/gen/parser/template.vue")
                .unwrap();
        let _ = f.write_all(target.to_string().as_bytes());
        dbg!(target.to_string());
    }

    #[test]
    fn display_no_template() {
        let input = r#"//! This is a comment1
        //! This is a comment2
        //! This is a comment3
        // This is line comment
        <template></template>
        /// This is a doc comment
        /// hello
        <script>
        let mut counter:usize = 0
        
        let handle_actions:FnOnce()->() = || {
            counter += 1;
        }
        </script>
        
        <style></style>
        // This is line comment2
        // end of line comment
        "#;

        let target = ParseTarget::try_from(input).unwrap();
        // let mut f =
        //     File::create("/Users/user/Downloads/beyond-framework-main/gen/parser/template.gen")
        //         .unwrap();
        // let _ = f.write_all(target.to_string().as_bytes());
        assert_eq!(target.to_string().as_str(),"//! This is a comment1\n//! This is a comment2\n//! This is a comment3\n// This is line comment\n\n/// This is a doc comment\n/// hello\n<script>\nlet mut counter:usize = 0\n        \n        let handle_actions:FnOnce()->() = || {\n            counter += 1;\n        }\n        </script>\n\n// This is line comment2\n// end of line comment\n");
    }

    #[test]
    fn display_only_comments() {
        let input = r#"//! This is a comment1
        //! This is a comment2
        //! This is a comment3
        // This is line comment
        
        /// This is a doc comment
        /// hello
        // This is line comment2
        // end of line comment
        "#;

        let target = ParseTarget::try_from(input).unwrap();
        dbg!(&target.to_string());
        // let mut f =
        //     File::create("/Users/user/Downloads/beyond-framework-main/gen/parser/template.gen")
        //         .unwrap();
        // let _ = f.write_all(target.to_string().as_bytes());
    }
}
