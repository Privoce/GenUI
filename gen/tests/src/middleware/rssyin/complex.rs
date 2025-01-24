#[cfg(test)]
mod test_complex {
    use rssyin::{
        makepad::MakepadChainExpand,
        visitor::chain::{traits::ChainVisitor, VisitorChain},
    };
    use syn::{parse_str, Block};

    #[test]
    fn complex_fn() {
        let input = r#"
        {
            fn btn_click<T>(a: i32, b: &MyBtnProp, c: T) -> ()
            where T: Debug
            {
                fn handle_a(a: i32){
                    println!("a: {:?}", a);
                }

                let handle_c = |c: T|{
                    println!("c: {:?}", c);
                };

                dbg!(a, c);
                println!("btn clicked: {:?}", b);
                prop.name = "Hello".to_string();
                if a > 10 {
                    prop.a = 20;
                }
                let _ = handle_a(a);
                handle_c(c);
            }
        }
        "#;
        handle(input);
        // let block = parse_str::<Block>(&input).unwrap();
        // dbg!(block);
    }

    #[test]
    fn complex_colsure() {
        let input = r#"
        {
            let mut btn_click = |a: i32, b: &MyBtnProp|{
                dbg!(a);
                println!("btn clicked: {:?}", b);
                prop.name = "Hello".to_string();
                if a > 10 {
                    prop.a = 20;
                }
            };
        }
        "#;
        handle(input);
    }

    fn handle(input: &str) {
        let block = parse_str::<Block>(&input).unwrap();
        let mut chain = VisitorChain::build();
        let _ = chain.visit_block_with(&block);
        let content = chain.bridge.to_string();
        std::fs::write(
            "/Users/shengyifei/projects/gen_ui/GenUI/gen/res2.md",
            content,
        )
        .unwrap();
    }
}
