use crate::{model::widget::role::Role, script::Script, token::ToLiveDesign};
use gen_parser::Props;
use gen_utils::{
    common::{camel_to_snake, snake_to_camel},
    error::{CompilerError, Error},
};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{parse_quote, parse_str, ItemStruct};

/// DefineWidget
/// 通用Widget模型
/// 包括了Makepad Widget的DSL结构和声明，事件，需要实现的Trait等
#[derive(Debug, Clone)]
pub struct DefineWidget {
    pub name: String,
    pub prop: Option<HashMap<String, String>>,
    // pub traits: Option<WidgetTraits>,
    pub role: Role,
}

impl DefineWidget {
    pub fn root_name(&self) -> TokenStream {
        self.name()
    }
    pub fn default_prop(&self) -> ItemStruct {
        let name = parse_str::<TokenStream>(&self.name).unwrap();
        parse_quote! {
             #[prop] pub struct #name{}
        }
    }
    pub fn default_script(&self) -> Script {
        let name = parse_str::<TokenStream>(&self.name).unwrap();
        Script::default(name)
    }
    pub fn snake_name(&self) -> String {
        camel_to_snake(&self.name)
    }
    // pub fn is_static(&self) -> bool {
    //     self.role.is_normal()
    // }
}

impl ToLiveDesign for DefineWidget {
    fn name(&self) -> proc_macro2::TokenStream {
        parse_str::<TokenStream>(&snake_to_camel(&self.name)).unwrap()
    }

    fn props(&self) -> Option<proc_macro2::TokenStream> {
        if let Some(props) = &self.prop {
            let props = props.iter().fold(TokenStream::new(), |mut tk, (k, v)| {
                let k = parse_str::<TokenStream>(&k).unwrap();
                let v = parse_str::<TokenStream>(&v).unwrap();

                tk.extend(quote! {#k: #v,});
                tk
            });

            Some(props)
        } else {
            None
        }
    }
}

pub fn to_prop_map(prop: Props) -> Option<HashMap<String, String>> {
    prop.map(|prop| {
        prop.iter()
            .filter_map(|(k, v)| {
                if k.is_normal() {
                    Some((k.name().to_string(), v.to_string()))
                } else {
                    None
                }
            })
            .collect()
    })
}

impl TryFrom<(String, Props, bool)> for DefineWidget {
    type Error = Error;

    fn try_from(value: (String, Props, bool)) -> Result<Self, Self::Error> {
        let (name, prop, root) = value;

        let mut prop = to_prop_map(prop);
        // [handle name] -----------------------------------------------------------------------------------------
        // 如果是根节点，那么需要从prop中获取name
        let name = if root && name == "component" {
            let name = if prop.is_some() {
                prop.as_ref().unwrap().get("name").map(|v| v.to_string())
            } else {
                None
            };

            if name.is_some() {
                let _ = prop.as_mut().unwrap().remove("name");
                name.unwrap()
            } else {
                return Err(CompilerError::runtime(
                    "Makepad Compiler - Define Widget",
                    "root component must has `name` prop",
                )
                .into());
            }
        } else {
            name
        };

        // [keep prop exist if not empty] ---------------------------------------------------------------------
        let prop = if let Some(prop) = prop {
            if prop.is_empty() {
                None
            } else {
                Some(prop)
            }
        } else {
            None
        };

        Ok(DefineWidget {
            name,
            prop,
            // traits: todo!(),
            role: Role::Normal,
        })
    }
}
