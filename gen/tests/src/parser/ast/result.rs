#[cfg(test)]
mod test_result {
    use std::time::Instant;

    use gen_parser::{ParseResult, ParseTarget};


    #[test]
    fn test_result_ark() {
        let input = r#"
        <template>
            Row(){
                Text("Hello world")
                Column() {
                    Text("Hello world1")
                    Text("Hello world2")
                }.width("80%").height(50)
                Column() {
                    Text("Hello world3")
                }.width("80%").height(50)
            }
        </template>
        "#;
        let t = Instant::now();
        let res = ParseResult::try_from(ParseTarget::try_from(input).unwrap()).unwrap();
        dbg!(t.elapsed());
        dbg!(res.script());
    }

    #[test]
    fn test_result() {
        let input = r#"
        <template>
            <view id="my_view">
                <button id="my_btn">
                    <label as_prop="slot" text="'Hello World'"></label>
                </button>
            </view>
        </template>

        <style>
        #my_view {
            theme: Dark;
            background_color: #FFF;
            height: 200.0;
            width: 300.0;
            #my_btn {
                theme: Error;
                margin: 16.0;
            }
        }
        </style>
        "#;
        let t = Instant::now();
        let _ = ParseResult::try_from(ParseTarget::try_from(input).unwrap()).unwrap();
        dbg!(t.elapsed());
    }
}
