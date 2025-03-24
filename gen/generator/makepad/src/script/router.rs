use quote::ToTokens;

#[derive(Debug, Clone)]
pub struct RouterScript();

impl RouterScript{

}

impl ToTokens for RouterScript{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        todo!()
    }
}