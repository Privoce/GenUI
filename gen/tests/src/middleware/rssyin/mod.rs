mod complex;

#[cfg(test)]
mod test_makepad {

    use quote::quote;
    use rssyin::{
        makepad::MakepadChainExpand,
        visitor::chain::{traits::ChainVisitor, VisitorChain},
    };
    use syn::{parse_str, Block, Stmt};

    #[test]
    fn if_block() {
        let input = r#"
        {
            let mut a = self.a();
            self.b = 10;
        }
        "#;

        let block = parse_str::<Stmt>(&input).unwrap();
        dbg!(block);
    }

    #[test]
    fn real_test1() {
        // my_label.text = "Clicked!".to_string();
        // let input = r#"
        // {
        //     let mut lb_hover_in = |param: impl EventParam|{
        //         let mut my_label = c_ref!(my_label);
        //         my_label.set_text("Clicked!");
        //     };
        // }
        // "#;

        let input = r#"
        {
            #[component]
            pub struct Hello3{
                pub lb: String
            }

            let mut prop = default_prop!{
                Hello3{
                    lb: "Hello".to_string()
                }
            };

            let mut btn_clicked = ||{
                prop.lb = "Clicked".to_string();
            };
        }"#;

        handle(input);
    }

    #[test]
    fn unified_test2() {
        let input = r#"
        {
            mod my_module;

            const MY_CONST: i32 = 10;

            use std::io::Write;
            use self::my_module::{Model1, Model2};
            pub use self::my_module2::Model3;

            import! {
                crate::views::my_button::*;
            }
            #[component]
            struct MyStruct {
                a: i32,
                b: f32,
            }

            #[event]
            enum MyEnum {
                A,
                B,
                C,
            }

            let mut prop = default_prop!{
                MyStruct{
                    a: 10,
                    b: 12.0,
                }
            };

            #[derive(Debug)]
            pub struct OtherStruct {
                pub name: String
            }

            fn btn_clicked(a:  i32){
                prop.b = 20.5;
                println!("btn clicked");
            }

            #[derive(Debug, PartialEq, Eq, Default)]
            pub enum OtherEnum {
                #[default]
                A2,
                B2,
            }

            let mut btn_clicked2 = ||{
                prop.a = 20;
                println!("btn clicked");
            };

            #[startup]
            fn app_start(){
                println!("app startup");
            }

            #[shutdown]
            fn app_shutdown(){
                println!("app shutdown");
            }

            #[net_error]
            fn net_work_error(){
                println!("network error");
            }
        }
        "#;

        handle(input);
    }

    #[test]
    fn unified_test() {
        let input = r#"
        {
            mod my_module;

            const MY_CONST: i32 = 10;

            use std::io::Write;
            use self::my_module::{Model1, Model2};
            pub use self::my_module2::Model3;

            import! {
                crate::views::my_button::*;
            }
            #[component]
            struct MyStruct {
                a: i32,
                b: f32,
            }

            #[event]
            enum MyEnum {
                A,
                B,
                C,
            }
        }
        "#;

        handle(input);
    }
    // lifetime -------------------------------------------------------------------------
    #[test]
    pub fn app_main_lifetime() {
        let input = r#"
        {
            #[startup]
            fn app_start(){
                println!("app startup");
            }

            #[shutdown]
            fn app_shutdown(){
                println!("app shutdown");
            }
            
            #[foreground]
            fn app_foreground(){
                println!("app foreground");
            }

            #[background]
            fn app_background(){
                println!("app background");
            }

            #[focus]
            fn app_focus(){
                println!("app focus");
            }
        }
        "#;
        // let block = parse_str::<Block>(&input).unwrap();
        // dbg!(block);
        handle(input);
    }

    // fn | closure ---------------------------------------------------------------------
    #[test]
    fn closure_with_prop() {
        let input = r#"
        {
            #[component]
            struct MyStruct {
                a: i32,
                b: f32,
            }

            let mut prop = default_prop!{
                MyStruct{
                    a: 10,
                    b: 12.0,
                }
            };

            let mut btn_clicked = ||{
                prop.a = 20;
                println!("btn clicked");
            }; 

            fn btn_clicked2(a:  i32){
                prop.name = "Hello".to_string();
                println!("btn clicked");
            }
            
        }"#;

        // let block = parse_str::<Block>(&input).unwrap();
        // dbg!(block);
        handle(input);
    }

    #[test]
    fn only_colsure() {
        let input = r#"
        {
            let mut btn_clicked2 = ||{
                prop.a = 20;
                println!("btn clicked");
            }; 

            let mut btn_clicked = |b: &MyB|{
                println!("btn clicked");
            }; 
        }
        "#;
        // let block = parse_str::<Block>(&input).unwrap();
        // dbg!(block);
        handle(input);
    }

    #[test]
    fn only_fn() {
        let input = r#"
        {
            fn btn_clicked(a:  i32){
                prop.a = 20;
                println!("btn clicked");
            }
        }
        "#;
        // let block = parse_str::<Block>(&input).unwrap();
        // dbg!(block);
        handle(input);
    }

    // default_prop! ---------------------------------------------------------------------
    #[test]
    fn local_default_prop_tk() {
        let input = r#"
        {
            let mut prop = default_prop!{
                MyStruct{
                    a: 10,
                    b: "Hello".to_string(),
                }
            };
        }"#;

        let block = parse_str::<Block>(&input).unwrap();
        let mut chain = VisitorChain::build();
        let _ = chain.visit_block_with(&block);

        dbg!(
            chain.bridge.gen_instance_tk().to_string(),
            chain.bridge.instance_ident()
        );
    }

    #[test]
    fn local_default_prop() {
        let input = r#"
        {
            let item_num = 10;
            let item_name = "item".to_string();
            let item_price: u32 = 10.0;
            let mut prop = default_prop!{
                MyStruct{
                    a: 10,
                    b: 10.0,
                }
            };
            let mut btn_clicked = ||{
                println!("btn clicked");
            }; 
        }
        "#;

        handle(input);
    }

    #[test]
    fn impl_default() {
        let input = r#"
        {
            let mut prop = default_prop!{
                MyStruct{
                    a: 10,
                    b: 10.0,
                }
            };
        }
        "#;

        // let block = parse_str::<Block>(&input).unwrap();
        // dbg!(block);
        handle(input);
    }
    // enum with #[event] ---------------------------------------------------------------
    #[test]
    fn event_macro_enum() {
        let input = r#"
        {
            use std::io::Write;
            #[event]
            enum MyEnum {
                A,
                B,
                C,
            }
        }"#;

        handle(input);
    }
    // struct with #[component] --------------------------------------------------------------
    #[test]
    fn prop_macro_struct() {
        let input = r#"
        {
            use std::io::Write;
           
            import!{
                crate::views::my_button::*;
            }

            #[component]
            struct MyStruct {
                a: i32,
                b: f32,
            }
        }"#;
        // let block = parse_str::<Block>(&input).unwrap();
        // dbg!(block);
        handle(input);
    }

    // const type use mod static union verbatim -----------------------------------------
    #[test]
    fn other_easy() {
        let input = r#"
        {
            const MY_CONST: i32 = 10;
            type MyType = i32;
            mod my_module;
            static MY_STATIC: i32 = 10;
            use std::io::Write;
            union MyUnion {
                a: i32,
                b: f32,
            }
        }
        "#;

        let block = parse_str::<Block>(&input).unwrap();
        let mut chain = VisitorChain::build();
        let res = chain.visit_block_with(&block);
        assert!(res.is_ok());
        let code = chain.bridge.others.unwrap();
        let res = quote! {
            #(#code)*
        };
        dbg!(res.to_string());
    }

    // use ------------------------------------------------------------------------------
    #[test]
    fn use_easy() {
        let input = r#"
        {
            mod my_module;
            pub mod my_module2;
            use std::io::Write;
            use crate::utils::my_utils::MyUtils;
            use self::my_module::{Model1, Model2};
            pub use self::my_module2::Model3;
        }"#;

        let block = parse_str::<Block>(&input).unwrap();

        let mut chain = VisitorChain::build();
        let res = chain.visit_block_with(&block);
        assert!(res.is_ok());
        let code = chain.bridge.others.unwrap();
        let res = quote! {
            #(#code)*
        };
        assert_eq!(&res.to_string(), "mod my_module ; pub mod my_module2 ; use std :: io :: Write ; use crate :: utils :: my_utils :: MyUtils ; use self :: my_module :: { Model1 , Model2 } ; pub use self :: my_module2 :: Model3 ;");
    }
    // import ---------------------------------------------------------------------------
    #[test]
    fn import_easy() {
        let input = r#"
        {
            import! {
                crate::views::my_button::*;
            }
        }
        "#;

        let block = parse_str::<Block>(&input).unwrap();
        let mut chain = VisitorChain::build();
        let res = chain.visit_block_with(&block);
        assert!(res.is_ok());
        assert_eq!(
            &chain.bridge.imports.take().map(|s| s.to_string()).unwrap(),
            "crate :: views :: my_button ::*;"
        );
    }

    #[test]
    fn import_err_multi() {
        let input = r#"
        {
            import! {
                crate::views::my_button::*;
                crate::views::my_button2::Button2;
            }
            import! {
                crate::views::my_button3::*;
            }
        }
        "#;

        let block = parse_str::<Block>(&input).unwrap();
        let mut chain = VisitorChain::build();
        let res = chain.visit_block_with(&block);
        if let Err(e) = res {
            dbg!(e.to_string());
        }
        assert_eq!(
            &chain.bridge.imports.take().map(|s| s.to_string()).unwrap(),
            "crate :: views :: my_button ::*; crate :: views :: my_button2 :: Button2 ;"
        );
    }

    #[test]
    fn back_to_rust(){
        let input = r#"
        {
            #[component]
            pub struct AProp{
                pub a: i32,
                pub b: f32,
            }

            impl Default for AProp {
                fn default() -> Self {
                    Self {
                        a: 10,
                        b: 10.0,
                    }
                }
            }

            impl AProp {
                fn btn_clicked(){
                    println!("btn clicked");
                }
            }
        }
        "#;

        let block = parse_str::<Block>(&input).unwrap();
        dbg!(block);


    }



    fn handle(input: &str) {
        let block = parse_str::<Block>(&input).unwrap();
        let mut chain = VisitorChain::build();
        let _ = chain.visit_block_with(&block);

        dbg!(&chain.bridge);

        let content = chain.bridge.to_string();

        println!("{}", content);
        // std::fs::write(
        //     "/Users/shengyifei/projects/gen_ui/GenUI/gen/res.md",
        //     content,
        // )
        // .unwrap();
    }
}
