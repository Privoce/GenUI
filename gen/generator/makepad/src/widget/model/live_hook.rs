//! pub trait LiveHook {
//!     //fn before_live_design(_cx:&mut Cx){}
//!
//!     fn apply_value_unknown(&mut self, cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) -> usize {
//!         if !nodes[index].origin.node_has_prefix() {
//!             cx.apply_error_no_matching_field(live_error_origin!(), index, nodes);
//!         }
//!         nodes.skip_node(index)
//!     }
//!
//!     fn apply_value_instance(&mut self, _cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) -> usize {
//!         nodes.skip_node(index)
//!     }
//!
//!     fn skip_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode])->Option<usize>{None}
//!     fn before_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]){}
//!     fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {}
//!     fn after_apply_from(&mut self, cx: &mut Cx, apply: &mut Apply) {
//!         match &apply.from{
//!             ApplyFrom::NewFromDoc{..}=>{self.after_new_from_doc(cx);self.after_apply_from_doc(cx);}
//!             ApplyFrom::UpdateFromDoc{..}=>{self.after_update_from_doc(cx);self.after_apply_from_doc(cx);}
//!             _=>()
//!         }
//!     }
//!     fn after_new_from_doc(&mut self, _cx:&mut Cx){}
//!     fn after_update_from_doc(&mut self, _cx:&mut Cx){}
//!     fn after_apply_from_doc(&mut self, _cx:&mut Cx){}
//!     fn after_new_before_apply(&mut self, _cx: &mut Cx) {}
//! }

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

/// LiveHookTrait是一个trait，它包含了LiveHook的所有方法
/// 它被使用在Widget的实现中
#[derive(Debug, Default, Clone)]
pub struct LiveHookTrait {
    pub before_live_design: Option<TokenStream>,
    pub apply_value_unknown: Option<TokenStream>,
    pub apply_value_instance: Option<TokenStream>,
    pub skip_apply: Option<TokenStream>,
    pub before_apply: Option<TokenStream>,
    pub after_apply: Option<TokenStream>,
    pub after_apply_from: Option<TokenStream>,
    pub after_new_from_doc: Option<TokenStream>,
    pub after_update_from_doc: Option<TokenStream>,
    pub after_apply_from_doc: Option<TokenStream>,
    pub after_new_before_apply: Option<TokenStream>,
}

impl LiveHookTrait {
    pub fn to_token_stream(&self, target: Ident) -> TokenStream {
        let before_live_design = self.before_live_design();
        let apply_value_unknown = self.apply_value_unknown.as_ref();
        let apply_value_instance = self.apply_value_instance.as_ref();
        let skip_apply = self.skip_apply.as_ref();
        let before_apply = self.before_apply.as_ref();
        let after_apply = self.after_apply.as_ref();
        let after_apply_from = self.after_apply_from.as_ref();
        let after_new_from_doc = self.after_new_from_doc.as_ref();
        let after_update_from_doc = self.after_update_from_doc.as_ref();
        let after_apply_from_doc = self.after_apply_from_doc.as_ref();
        let after_new_before_apply = self.after_new_before_apply.as_ref();

        quote! {
            impl LiveHook for #target{
                #before_live_design
                #apply_value_unknown
                #apply_value_instance
                #skip_apply
                #before_apply
                #after_apply
                #after_apply_from
                #after_new_from_doc
                #after_update_from_doc
                #after_apply_from_doc
                #after_new_before_apply
            }
        }
    }
}

pub trait ImplLiveHook {
    fn before_live_design(&self) -> Option<TokenStream>;
    fn apply_value_unknown(&self) -> Option<TokenStream>;
    fn apply_value_instance(&self) -> Option<TokenStream>;
    fn skip_apply(&self) -> Option<TokenStream>;
    fn before_apply(&self) -> Option<TokenStream>;
    fn after_apply(&self) -> Option<TokenStream>;
    fn after_apply_from(&self) -> Option<TokenStream>;
    fn after_new_from_doc(&self) -> Option<TokenStream>;
    fn after_update_from_doc(&self) -> Option<TokenStream>;
    fn after_apply_from_doc(&self) -> Option<TokenStream>;
    fn after_new_before_apply(&self) -> Option<TokenStream>;
}

impl ImplLiveHook for LiveHookTrait {
    fn before_live_design(&self) -> Option<TokenStream> {
        self.before_live_design.as_ref().map(|tk| {
            quote! {
                fn before_live_design(_cx:&mut Cx){
                    #tk
                }
            }
        })
    }
    fn apply_value_unknown(&self) -> Option<TokenStream> {
        self.apply_value_unknown.as_ref().map(|tk|quote! {
            fn apply_value_unknown(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) -> usize {
                #tk
            }
        })
    }
    fn apply_value_instance(&self) -> Option<TokenStream> {
        self.apply_value_instance.as_ref().map(|tk|quote! {
            fn apply_value_instance(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) -> usize {
                #tk
            }
        })
    }
    fn skip_apply(&self) -> Option<TokenStream> {
        self.skip_apply.as_ref().map(|tk|quote! {
            fn skip_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) -> Option<usize> {
                #tk
            }
        })
    }
    fn before_apply(&self) -> Option<TokenStream> {
        self.before_apply.as_ref().map(|tk|quote! {
            fn before_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
                #tk
            }
        })
    }
    fn after_apply(&self) -> Option<TokenStream> {
        self.after_apply.as_ref().map(|tk|quote! {
            fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
                #tk
            }
        })
    }
    fn after_apply_from(&self) -> Option<TokenStream> {
        self.after_apply_from.as_ref().map(|tk| {
            quote! {
                fn after_apply_from(&mut self, cx: &mut Cx, apply: &mut Apply) {
                    #tk
                }
            }
        })
    }
    fn after_new_from_doc(&self) -> Option<TokenStream> {
        self.after_new_from_doc.as_ref().map(|tk| {
            quote! {
                fn after_new_from_doc(&mut self, cx: &mut Cx) {
                    #tk
                }
            }
        })
    }
    fn after_update_from_doc(&self) -> Option<TokenStream> {
        self.after_update_from_doc.as_ref().map(|tk| {
            quote! {
                fn after_update_from_doc(&mut self, cx: &mut Cx) {
                    #tk
                }
            }
        })
    }
    fn after_apply_from_doc(&self) -> Option<TokenStream> {
        self.after_apply_from_doc.as_ref().map(|tk| {
            quote! {
                fn after_apply_from_doc(&mut self, cx: &mut Cx) {
                    #tk
                }
            }
        })
    }
    fn after_new_before_apply(&self) -> Option<TokenStream> {
        self.after_new_before_apply.as_ref().map(|tk| {
            quote! {
                fn after_new_before_apply(&mut self, cx: &mut Cx) {
                    #tk
                }
            }
        })
    }
}
