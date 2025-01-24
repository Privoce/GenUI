use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, parse_str, Block, Stmt};
use toml_edit::{Formatted, InlineTable, Value};

mod category;
use super::PluginError;
pub use category::*;

#[derive(Debug, Clone)]
pub struct Macro {
    pub name: String,
    pub category: Category,
    pub stmts: Vec<Stmt>,
}

impl From<Macro> for toml_edit::Item {
    fn from(value: Macro) -> Self {
        let mut table = InlineTable::new();
        table.insert("category", value.category.into());
        if !value.stmts.is_empty() {
            let stmts = value.stmts;
            table.insert(
                "stmts",
                Value::String(Formatted::new(
                    quote! {
                        #(#stmts)*
                    }
                    .to_string(),
                )),
            );
        }

        toml_edit::Item::Value(Value::InlineTable(table))
    }
}

impl TryFrom<&InlineTable> for Macro {
    type Error = PluginError;

    fn try_from(value: &InlineTable) -> Result<Self, Self::Error> {
        let category = value
            .get("category")
            .ok_or_else(|| PluginError::from("Invalid category [macros.category]"))?;
        let stmts = value.get("stmts").map_or_else(
            || Err(PluginError::from("Invalid stmt [macros.stmt]")),
            |stmt| {
                let stmt_str = stmt
                    .as_str()
                    .ok_or_else(|| PluginError::from("Invalid stmts [macros.stmt]"))?;

                let stmt: Block =
                    parse2(parse_str::<TokenStream>(&format!("{{{}}}", stmt_str)).unwrap())
                        .map_err(|e| {
                            PluginError::from(format!("parse block [macros.stmt]: {}", e))
                        })?;

                Ok(stmt)
            },
        )?;

        Ok(Macro {
            name: String::new(),
            category: Category::try_from(category)?,
            stmts: stmts.stmts,
        })
    }
}

impl TryFrom<(&str, &InlineTable)> for Macro {
    type Error = PluginError;

    fn try_from(value: (&str, &InlineTable)) -> Result<Self, Self::Error> {
        let (name, table) = value;
        let mut mac = Macro::try_from(table)?;
        mac.name = name.to_string();
        Ok(mac)
    }
}

impl Macro {
    pub fn to_table(self) -> (String, toml_edit::Item) {
        (self.name.clone(), self.into())
    }
    pub fn to_expr(&self) -> TokenStream {
        if let Category::PropMacro = self.category {
            let stmt = &self.stmts;
            quote! {
                #(#stmt)*
            }
        } else {
            unimplemented!("Macro category not supported now!")
        }
    }
}
