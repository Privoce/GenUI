#[cfg(test)]
mod template_parsers {

    use std::time::Instant;

    use gen_parser::target::template::html::{parse_tag_start, parse_template};

    #[test]
    fn test_tag_start() {
        let input = r#"<button value="Hello world" class="button1" @clicked="handle_actions"/>"#;
        let res = parse_tag_start(input).unwrap();
        dbg!(res);
    }

    #[test]
    fn test_script_tag() {
        let input = r#"<script lang="ets">"#;
        let res = parse_tag_start(input).unwrap();
        dbg!(res);
    }

    #[test]
    fn test_template_nested_same() {
        let template = r#"
        <view id="main_page">
            <view id="title_wrap">
                <view class="title_s_n">
                    <view id="nnn">
                        <label class="common_txt"></label>
                    </view>
                </view>
            </view>
            <view id="support_wrap">
            
            </view>
        </view>
        "#;

        let _res = parse_template(template);
        // dbg!(res);
    }

    #[test]
    fn bad_template3() {
        let template = r#"
        
        "#;
        // dbg!(parse_template(template));
        assert!(parse_template(template).is_err())
    }

    #[test]
    fn bad_template2() {
        let template = r#"
        <input>xxx</input>
        "#;
        // dbg!(parse_template(template));
        assert!(parse_template(template).is_err())
    }

    #[test]
    fn bad_template1() {
        let template = r#"
            </input>
        "#;
        assert!(parse_template(template).is_err());
    }

    #[test]
    fn test_template_all() {
        let template = r#"
        // this is a window
            <window class="ui">
                <view class="body">
                    /// button componet
                    <button value="Hello world" class="button1" @clicked="handle_actions">
                        <div></div>
                        <div />
                    </button>
                    <text_input value="Click to count" class="input1" />
                    <label :value="counter" class="label1" />
                </view>
            </window>
            <text_input value="Click to count" class="input1" />
        "#;
        let t = Instant::now();
        let _ = parse_template(template).unwrap();
        // about 470µs
        dbg!(t.elapsed());
        // let res = res
        //     .into_iter()
        //     .map(|x| x.to_string())
        //     .collect::<Vec<String>>()
        //     .join("\n");
        // //E:/Rust/try/makepad/gen/parser/t.gen
        // let mut f =
        //     File::create("/Users/user/Downloads/beyond-framework-main/gen/parser/t.html").unwrap();
        // let _ = f.write(res.as_bytes());
        // dbg!(res);
    }
    #[test]
    fn test_parse_template_multi() {
        let tag = r#" 
        //! file!
        <button value="Hello world" class="button1" @clicked="handle_actions"/>"#;

        let res = parse_template(tag).unwrap();
        dbg!(res);
    }

    #[test]
    fn test_parse_template() {
        let tag = r#" <button value="Hello world" class="button1" @clicked="handle_actions"/>"#;
        let comment = r#"//! file!"#;
        let res1 = parse_template(tag).unwrap();
        let res2 = parse_template(comment).unwrap();
        dbg!(res1);
        dbg!(res2);
    }

    #[test]
    fn test_parse_tag_nesting() {
        let tag1 = r#"
        <view class="body">
            <button value="Hello world" class="button1" @clicked="handle_actions"/>
            <text-input value="Click to count" class="input1"/>
        </view>
        "#;
        assert!(parse_template(tag1).is_ok())
    }

    #[test]
    fn test_parse_tag_normal_close() {
        let tag1 =
            r#"<button :value="hello_world" class="button1" @clicked="handle_actions"></button>"#;
        let res = parse_template(tag1).unwrap();
        dbg!(res);
    }

    #[test]
    fn test_parse_tag_close_self() {
        let tag1 = r#"<button value="Hello world" class="button1" @clicked="handle_actions"/>"#;
        let res = parse_template(tag1).unwrap();
        dbg!(res);
    }
}
