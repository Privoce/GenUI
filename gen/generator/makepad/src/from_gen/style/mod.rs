//! 用于将GenUI的单<style>标签的`.gen`文件转化为Makepad的`.rs`文件
//! 这里需要处理的是crate::src::builtin::prop::mod的[str_to_tk!_prop_value_static]中的[str_to_tk!_custom_prop_value_static]方法
//! 大部分可以识别prop的键的会在之前就处理掉，这里只处理非Builtin标准的prop的value
//! 对于这类无法识别prop的键，一般都出现在开发者自定义的组件上

use std::str::FromStr;

use super::MakepadColor;
use crate::{
    builtin::prop::{err_from_to, LiveDependency},
    str_to_tk,
    traits::ToTokensExt,
};
use gen_analyzer::value::{BuiltinColor, Enum, Function, Struct, Value};
use gen_utils::common::format_float;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_str;
impl ToTokensExt for usize {
    fn to_token_stream(&self) -> Result<proc_macro2::TokenStream, gen_utils::error::Error> {
        str_to_tk!(&self.to_string())
    }
}

impl ToTokensExt for isize {
    fn to_token_stream(&self) -> Result<proc_macro2::TokenStream, gen_utils::error::Error> {
        str_to_tk!(&self.to_string())
    }
}

impl ToTokensExt for f32 {
    fn to_token_stream(&self) -> Result<proc_macro2::TokenStream, gen_utils::error::Error> {
        str_to_tk!(format_float(*self as f64).as_str())
    }
}

impl ToTokensExt for f64 {
    fn to_token_stream(&self) -> Result<proc_macro2::TokenStream, gen_utils::error::Error> {
        str_to_tk!(format_float(*self).as_str())
    }
}

impl ToTokensExt for bool {
    fn to_token_stream(&self) -> Result<proc_macro2::TokenStream, gen_utils::error::Error> {
        str_to_tk!(self.to_string().as_str())
    }
}

impl ToTokensExt for String {
    fn to_token_stream(&self) -> Result<proc_macro2::TokenStream, gen_utils::error::Error> {
        if let Ok(hex) = BuiltinColor::from_str(self) {
            let color = MakepadColor {
                fn_name: None,
                color: hex,
            };

            return ToTokensExt::to_token_stream(&color);
        } else {
            return Ok(quote! {
                #self
            });
        }
    }
}

impl ToTokensExt for Enum {
    fn to_token_stream(&self) -> Result<proc_macro2::TokenStream, gen_utils::error::Error> {
        let Enum { field_chain } = self;
        let len = field_chain.len();
        if len == 0 {
            return Err(err_from_to("Enum", "TokenStream").into());
        } else if len == 1 {
            str_to_tk!(&field_chain.get(0).unwrap().to_string())
        } else {
            // get the last leaf
            let leaf = field_chain.last().unwrap();
            str_to_tk!(&leaf.to_string())
        }
    }
}

// 在GenUI中的方法其实都是GenUI需要进行转化封装的，现在基本有dep,linear,radial,rgb,rgba这些,dep属于makepad直接提供的，其他需要进行转化
// 对于makepad直接提供的，转化没什么问题，但是对于linear, radial, shader这几个函数，makepad无法接受，只有到绑定到某个组件上才能使用
// 目前的策略是生成一个伪组件, 这样就没有语法错误, 后续等支持了样式的import和$bind之后再进行处理
// ```
// // GenUI
// .draw{
//      bg: shader(|self|{
//          fn pixel(self) -> vec4{return #FFFF00}
//      });
// }
// // Makepad
// DRAW__BG = {
//     fn pixel(self) -> vec4{
//          return #FFFF00;
//     }
// }
// ```
impl ToTokensExt for Function {
    fn to_token_stream(&self) -> Result<proc_macro2::TokenStream, gen_utils::error::Error> {
        BuiltinColor::try_from(self).map_or_else(
            |_| {
                LiveDependency::try_from(self).map_or_else(
                    |_| Err(format!("not support this function: {}", &self.name).into()),
                    |dep| Ok(quote::ToTokens::to_token_stream(&dep)),
                )
            },
            |color| {
                ToTokensExt::to_token_stream(&MakepadColor {
                    fn_name: None,
                    color,
                })
            },
        )
    }
}

impl ToTokensExt for Vec<Value> {
    fn to_token_stream(&self) -> Result<proc_macro2::TokenStream, gen_utils::error::Error> {
        let mut tk = TokenStream::new();
        for item in self.iter() {
            tk.extend(ToTokensExt::to_token_stream(item)?);
        }
        Ok(quote! {
            [#tk]
        })
    }
}

impl ToTokensExt for Struct {
    fn to_token_stream(&self) -> Result<TokenStream, gen_utils::error::Error> {
        let Struct { fields, .. } = self;
        let mut tk = TokenStream::new();
        for (k, v) in fields {
            let k = parse_str::<TokenStream>(k)
                .map_err(|e| gen_utils::error::Error::FromDynError(e.to_string()))?;
            let v = ToTokensExt::to_token_stream(v)?;
            tk.extend(quote! {
                #k: #v
            });
        }
        Ok(quote! {
            {
                #tk
            }
        })
    }
}

impl ToTokensExt for Value {
    fn to_token_stream(&self) -> Result<proc_macro2::TokenStream, gen_utils::error::Error> {
        match self {
            Value::USize(num) => num.to_token_stream(),
            Value::ISize(num) => num.to_token_stream(),
            // Value::Float(num) => num.to_token_stream(),
            Value::Double(num) => num.to_token_stream(),
            Value::Bool(b) => b.to_token_stream(),
            Value::Vec(vec) => vec.to_token_stream(),
            Value::String(s) => s.to_token_stream(),
            Value::Bind(_bind) => unimplemented!("wait till GenUI > v0.1.1"),
            Value::Function(function) => ToTokensExt::to_token_stream(function),
            Value::Struct(s) => ToTokensExt::to_token_stream(s),
            Value::Enum(e) => ToTokensExt::to_token_stream(e),
            Value::Animation(_hash_map) => unimplemented!("wait till GenUI > v0.1.2"),
            Value::UnKnown(s) => parse_str::<TokenStream>(s).map_err(|e| e.to_string().into()),
        }
    }
}
