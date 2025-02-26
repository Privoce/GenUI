use std::sync::{Arc, RwLock};

use gen_analyzer::Polls;
use gen_utils::error::Error;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{ItemEnum, Stmt};

use crate::{compiler::Context, model::TemplatePtrs, two_way_binding::TWBPollBuilder};

use super::{Impls, LiveStruct};

#[derive(Debug, Clone, Default)]
pub struct ScRs {
    /// ident of the struct
    pub ident: Option<TokenStream>,
    /// rust uses
    pub uses: Option<TokenStream>,
    /// live struct
    pub live_struct: Option<LiveStruct>,
    /// events
    pub events: Option<Vec<ItemEnum>>,
    /// impls
    pub impls: Option<Impls>,
    pub twb_poll: Option<TWBPollBuilder>,
    pub others: Option<Vec<Stmt>>,
}

impl ScRs {
    pub fn handle(
        bridger: gen_analyzer::Script,
        ctx: &mut Context,
        polls: Arc<RwLock<Polls>>,
        template_ptrs: &TemplatePtrs,
        ident: TokenStream,
    ) -> Result<Self, Error> {
        // [ident] -------------------------------------------------------------------------------------------
        let sc_rs = ScRs::default();

    }
    pub fn default_sc(ident: TokenStream)-> Self{

    }
}

impl Default for ScRs {
    fn default() -> Self {
        Self {
            ident: Default::default(),
            uses: Default::default(),
            live_struct: Default::default(),
            events: Default::default(),
            impls: Default::default(),
            twb_poll: Default::default(),
            others: Default::default(),
        }
    }
}

impl ToTokens for ScRs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let uses = self.uses.as_ref();
        let struct_ident = self.ident.as_ref().unwrap();
        let live_struct = self.live_struct.to_token_stream();
        let impls = self
            .impls
            .as_ref()
            .map(|impls| impls.to_token_stream(struct_ident, self.twb_poll.as_ref()));
        let events = self.events.as_ref().map(|events| {
            quote! {
                #( #events )*
            }
        });
        tokens.extend(quote! {
            #uses
            #live_struct
            #events
            #impls
        });
        self.others.as_ref().map(|others| {
            tokens.extend(quote! {
                #(#others)*
            });
        });
    }
}
