//! mode:
//! - variable: let | const variable_name: variable_type = variable_value;
//! - funcation: let function_name: function_type = ||{ function_handle };
use gen_utils::error::{ConvertError, Error};
use proc_macro2::TokenStream;
use syn::{parse2, Block};

pub fn parse_script(input: &str) -> Result<Block, Error> {
    let input = format!("{{ {} }}", input);
    // make input to TokenStream
    let token = match input.parse::<TokenStream>() {
        Ok(t) => t,
        Err(_) => {
            return Err(ConvertError::FromTo {
                from: "GenUI Script".to_string(),
                to: "Rust TokenStream".to_string(),
            }
            .into());
        }
    };
    // token to ast
    match parse2::<Block>(token) {
        Ok(ast) => Ok(ast),
        Err(_) => Err(ConvertError::FromTo {
            from: "Rust TokenStream".to_string(),
            to: "Rust Block".to_string(),
        }
        .into()),
    }
}
