//! mode:
//! - variable: let | const variable_name: variable_type = variable_value;
//! - funcation: let function_name: function_type = ||{ function_handle };
use gen_utils::error::Error;
use proc_macro2::TokenStream;
use syn::{parse2, Block};
#[allow(dead_code)]
pub fn parse_script(input: &str) -> Result<Block, Error> {
    let input = format!("{{ {} }}", input);
    // make input to TokenStream
    let token = match input.parse::<TokenStream>() {
        Ok(t) => t,
        Err(_) => {
            return Err(Error::parse_error(
                "cannot parse gen-ui script to rust TokenStream!",
            ));
        }
    };
    // token to ast
    match parse2::<Block>(token) {
        Ok(ast) => Ok(ast),
        Err(_) => Err(Error::parse_error(
            "cannot convert TokenStream to rust Block!",
        )),
    }
}

#[cfg(test)]
mod test_script_parse {
    use proc_macro2::TokenStream;
    use syn::{parse2, parse_str, Block, Expr, Stmt};

    use super::parse_script;

    #[test]
    fn test_ets_sc(){
        let ets = r#"
        import { hilog } from '@kit.PerformanceAnalysisKit';
        import { BackupExtensionAbility, BundleVersion } from '@kit.CoreFileKit';

        export default class EntryBackupAbility extends BackupExtensionAbility {
            async onBackup() {
                hilog.info(0x0000, 'testTag', 'onBackup ok');
            }

            async onRestore(bundleVersion: BundleVersion) {
                hilog.info(0x0000, 'testTag', 'onRestore ok %{public}s', JSON.stringify(bundleVersion));
            }
        }
        "#;

        let res = parse_script(ets);
        assert!(res.is_err());
    }

    #[test]
    fn test_syn_parse_var() {
        let gen_code_var = r#"let counter: usize = 0_usize;"#;

        let ast_var = parse_str::<Stmt>(gen_code_var).unwrap();
        dbg!(ast_var);
    }

    #[test]
    fn test_syn_parse_fn() {
        let gen_code_fn = r#"
        let mut btn_click = ||{
            log!("BUTTON CLICKED {}", counter);
            counter += 1;
          }
        "#;

        let ast_fn = parse_str::<Expr>(gen_code_fn).unwrap();
        dbg!(ast_fn);
    }

    #[test]
    fn test_parse_mixin() {
        let code = r#"
        let mut counter:usize = 0_usize;

        let mut click = ||{
            counter += 1;
        };
        "#;
        // to tokenStream
        let token: TokenStream = format!("{{ {} }}", code).parse().expect("error token");

        // to ast -> Block
        let ast = parse2::<Block>(token).expect("ast  error");
        dbg!(ast);
    }
}
