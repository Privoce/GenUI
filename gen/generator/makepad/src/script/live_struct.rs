use gen_utils::error::{CompilerError, Error};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, Field, Fields, ItemStruct};

#[derive(Debug, Clone)]
pub struct LiveComponent(pub ItemStruct);

impl LiveComponent {
    pub fn ident(&self) -> TokenStream {
        self.0.ident.to_token_stream()
    }
    pub fn default(ident: &TokenStream) -> Self{
        Self(parse_quote!{
            #[derive(Live, Widget)]
            pub struct #ident {
                #[deref]
                pub deref_widget: GView,
            }
        })
    }
    pub fn push_field(&mut self, field: Field) -> Result<(), Error> {
        match &mut self.0.fields {
            Fields::Named(fields) => {
                fields.named.push(field);
                return Ok(());
            }
            _ => {
                return Err(CompilerError::runtime(
                    "Makepad Compiler - Script",
                    "prop should be a struct with named fields",
                )
                .into())
            }
        }
    }
}

impl ToTokens for LiveComponent {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl From<ItemStruct> for LiveComponent {
    fn from(item: ItemStruct) -> Self {
        LiveComponent(item)
    }
}
