use proc_macro2::TokenStream;
use quote::quote;
use syn::StmtMacro;

pub trait MacroConverter {
    /// convert active! macro to makepad cx.widget_action
    fn active_macro_to_action(&self) -> Option<TokenStream>;
}

impl MacroConverter for StmtMacro {
    fn active_macro_to_action(&self) -> Option<TokenStream> {
        if self.mac.path.is_ident("active") {
            let action = &self.mac.tokens;
            Some(quote! {
                cx.widget_action(uid, &scope.path, #action);
            })
        } else {
            None
        }
    }
}
