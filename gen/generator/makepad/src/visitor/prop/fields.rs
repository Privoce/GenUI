use gen_utils::error::{CompilerError, Error};
use quote::ToTokens;
use syn::{parse_quote, Field};

/// 当前可用使用#[live]的类型, 其他类型使用#[rust]
/// 之后考虑扩展为只要是字段类型标注了#[live_prop]，就相当于该类型必须要实现Default trait并在组件中可用使用#[live]
/// 改目标可以通过构建一个gen_macro crate来进行实现
pub const LIVE_TYPES: [&str; 32] = [
    "i32",
    "i64",
    "isize",
    "u32",
    "u64",
    "usize",
    "f32",
    "f64",
    "bool",
    "String",
    "LiveDependency",
    "Vec4",
    "Vec3",
    "Vec2",
    "Vec",
    "Margin",
    "Padding",
    "GOsType",
    "Direction",
    "Align",
    "GChooseType",
    "MouseCursor",
    "Direction",
    "EventOrder",
    "Flow",
    "Size",
    "ImageFit",
    "LinkType",
    "ViewOptimize",
    "TextWrap",
    "Themes",
    "Option",
];

/// ## handle attrs from field
/// - check if #[live] or #[rust] is exist? return err if exist
/// - if type is in LIVE_TYPES, use #[live] else use #[rust]
pub fn handle_field_attrs(field: &mut Field) -> Result<(), Error> {
    // check if #[live] or #[rust] is exist? return err if exist
    for attr in field.attrs.iter_mut() {
        if attr.path().is_ident("live") || attr.path().is_ident("rust") {
            return Err(CompilerError::runtime(
                "Makepad Compiler",
                "#[live], #[rust] are keywords, please remove them from field attributes",
            )
            .into());
        }
    }

    // check is in LIVE_TYPES
    let ty_str = field.ty.to_token_stream().to_string();
    // do not use contains, because such as Vec can use #[live], and Vec<i32> can use #[live] too
    let is_live = LIVE_TYPES.iter().any(|&live| ty_str.contains(live));
    field.attrs.push(if is_live {
        parse_quote! {
            #[live]
        }
    } else {
        parse_quote! {
            #[rust]
        }
    });

    Ok(())
}
