use gen_mk_script_objs::makepad::ScriptBridger;
use proc_macro2::TokenStream;
use quote::quote;

use crate::model::Traits;

pub fn script_to_token_stream(
    sc: Option<&ScriptBridger>,
    traits: Option<&Traits>,
    widget_root: TokenStream,
) -> Option<TokenStream> {
    if let Some(sc) = sc {
        let mut tokens = TokenStream::new();
        // [imports] -----------------------------------------------------------------------------------------------
        // let imports = &sc.imports;
        // [props] -------------------------------------------------------------------------------------------------
        let prop = &sc.prop;
        // [events] ------------------------------------------------------------------------------------------------
        let events = sc.events.as_ref().map(|events| {
            quote! {
                #(#events)*
            }
        });
        // [instance 不需要输出] -------------------------------------------------------------------------------------

        // [fn-callbacks] ------------------------------------------------------------------------------------------
        // [others] ------------------------------------------------------------------------------------------------
        let others = if let Some(other) = sc.others.as_ref() {
            let tk = other.iter().fold(TokenStream::new(), |mut tk, stmt| {
                tk.extend(quote! {#stmt});
                tk
            });
            Some(tk)
        } else {
            None
        };
        // [traits] ------------------------------------------------------------------------------------------------
        let traits = traits
            .as_ref()
            .map(|t_sc| t_sc.to_token_stream(&widget_root, None));

        tokens.extend(quote! {
            #prop
            #events
            #traits
            #others
        });

        return Some(tokens);
    }

    None
}
