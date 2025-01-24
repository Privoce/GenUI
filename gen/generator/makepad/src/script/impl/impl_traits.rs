use proc_macro2::TokenStream;

use crate::{model::Traits, two_way_binding::TWBPollBuilder};
#[derive(Default, Debug, Clone)]
pub struct ImplTraits(pub Traits);

impl ImplTraits {
    pub fn to_token_stream(
        &self,
        ident: &TokenStream,
        twb_poll: Option<&TWBPollBuilder>,
    ) -> TokenStream {
        self.0.to_token_stream(ident, twb_poll)
    }
}
