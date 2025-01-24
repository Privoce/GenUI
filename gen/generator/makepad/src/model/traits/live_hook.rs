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

use super::push_handle;

/// LiveHookTrait是一个trait，它包含了LiveHook的所有方法
/// 它被使用在Widget的实现中
#[derive(Debug, Clone)]
pub struct LiveHookTrait {
    pub apply_value_unknown: Option<TokenStream>,
    pub apply_value_instance: Option<TokenStream>,
    pub skip_apply_animator: Option<TokenStream>,
    pub skip_apply: Option<TokenStream>,
    pub before_apply: Option<TokenStream>,
    pub after_apply: Option<TokenStream>,
    pub after_apply_from: Option<TokenStream>,
    pub after_new_from_doc: Option<TokenStream>,
    pub after_update_from_doc: Option<TokenStream>,
    pub after_apply_from_doc: Option<TokenStream>,
    pub after_new_before_apply: Option<TokenStream>,
}

impl Default for LiveHookTrait {
    fn default() -> Self {
        Self {
            apply_value_unknown: Default::default(),
            apply_value_instance: Default::default(),
            skip_apply: Default::default(),
            before_apply: Default::default(),
            after_apply: Some(default_after_apply()),
            after_apply_from: Default::default(),
            after_new_from_doc: Default::default(),
            after_update_from_doc: Default::default(),
            after_apply_from_doc: Default::default(),
            after_new_before_apply: Default::default(),
            skip_apply_animator: Default::default(),
        }
    }
}

impl LiveHookTrait {
    pub fn to_token_stream<TK>(&self, target: TK) -> TokenStream
    where
        TK: Into<TokenStream>,
    {
        let target = target.into();

        let apply_value_unknown = self.apply_value_unknown_tk();
        let apply_value_instance = self.apply_value_instance_tk();
        let skip_apply_animator = self.skip_apply_animator_tk();
        let skip_apply = self.skip_apply_tk();
        let before_apply = self.before_apply_tk();
        let after_apply = self.after_apply_tk();
        let after_apply_from = self.after_apply_from_tk();
        let after_new_from_doc = self.after_new_from_doc_tk();
        let after_update_from_doc = self.after_update_from_doc_tk();
        let after_apply_from_doc = self.after_apply_from_doc_tk();
        let after_new_before_apply = self.after_new_before_apply_tk();

        quote! {
            impl LiveHook for #target{
                #apply_value_unknown
                #apply_value_instance
                #skip_apply
                #skip_apply_animator
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

fn default_after_apply() -> TokenStream {
    quote! {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

pub enum LiveHookType {
    ApplyValueUnknown,
    ApplyValueInstance,
    SkipApplyAnimator,
    SkipApply,
    BeforeApply,
    AfterApply,
    AfterApplyFrom,
    AfterNewFromDoc,
    AfterUpdateFromDoc,
    AfterApplyFromDoc,
    AfterNewBeforeApply,
}

pub trait ImplLiveHook {
    fn apply_value_unknown_tk(&self) -> Option<TokenStream>;
    fn apply_value_instance_tk(&self) -> Option<TokenStream>;
    fn skip_apply_tk(&self) -> Option<TokenStream>;
    fn skip_apply_animator_tk(&self) -> Option<TokenStream>;
    fn before_apply_tk(&self) -> Option<TokenStream>;
    fn after_apply_tk(&self) -> Option<TokenStream>;
    fn after_apply_from_tk(&self) -> Option<TokenStream>;
    fn after_new_from_doc_tk(&self) -> Option<TokenStream>;
    fn after_update_from_doc_tk(&self) -> Option<TokenStream>;
    fn after_apply_from_doc_tk(&self) -> Option<TokenStream>;
    fn after_new_before_apply_tk(&self) -> Option<TokenStream>;
    fn push(&mut self, tk: TokenStream, ty: LiveHookType) -> ();
}

impl ImplLiveHook for LiveHookTrait {
    fn apply_value_unknown_tk(&self) -> Option<TokenStream> {
        self.apply_value_unknown.as_ref().map(|tk|quote! {
            #[allow(unused_variables)]
            fn apply_value_unknown(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) -> usize {
                #tk
            }
        })
    }
    fn apply_value_instance_tk(&self) -> Option<TokenStream> {
        self.apply_value_instance.as_ref().map(|tk|quote! {
            #[allow(unused_variables)]
            fn apply_value_instance(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) -> usize {
                #tk
            }
        })
    }
    fn skip_apply_tk(&self) -> Option<TokenStream> {
        self.skip_apply.as_ref().map(|tk|quote! {
            #[allow(unused_variables)]
            fn skip_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) -> Option<usize> {
                #tk
            }
        })
    }
    fn before_apply_tk(&self) -> Option<TokenStream> {
        self.before_apply.as_ref().map(|tk|quote! {
            #[allow(unused_variables)]
            fn before_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
                #tk
            }
        })
    }
    fn after_apply_tk(&self) -> Option<TokenStream> {
        self.after_apply.as_ref().map(|tk|quote! {
            #[allow(unused_variables)]
            fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
                #tk
            }
        })
    }
    fn after_apply_from_tk(&self) -> Option<TokenStream> {
        self.after_apply_from.as_ref().map(|tk| {
            quote! {
                #[allow(unused_variables)]
                fn after_apply_from(&mut self, cx: &mut Cx, apply: &mut Apply) {
                    #tk
                }
            }
        })
    }
    fn after_new_from_doc_tk(&self) -> Option<TokenStream> {
        self.after_new_from_doc.as_ref().map(|tk| {
            quote! {
                fn after_new_from_doc(&mut self, cx: &mut Cx) {
                    #tk
                }
            }
        })
    }
    fn after_update_from_doc_tk(&self) -> Option<TokenStream> {
        self.after_update_from_doc.as_ref().map(|tk| {
            quote! {
                #[allow(unused_variables)]
                fn after_update_from_doc(&mut self, cx: &mut Cx) {
                    #tk
                }
            }
        })
    }
    fn after_apply_from_doc_tk(&self) -> Option<TokenStream> {
        self.after_apply_from_doc.as_ref().map(|tk| {
            quote! {
                #[allow(unused_variables)]
                fn after_apply_from_doc(&mut self, cx: &mut Cx) {
                    if !self.visible {return;}
                    #tk
                }
            }
        })
    }
    fn after_new_before_apply_tk(&self) -> Option<TokenStream> {
        self.after_new_before_apply.as_ref().map(|tk| {
            quote! {
                #[allow(unused_variables)]
                fn after_new_before_apply(&mut self, cx: &mut Cx) {
                    #tk
                }
            }
        })
    }

    fn skip_apply_animator_tk(&self) -> Option<TokenStream> {
        self.skip_apply_animator.as_ref().map(|tk| {
            quote! {
                #[allow(unused_variables)]
                fn skip_apply_animator(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode])->bool{
                    #tk
                }
            }
        })
    }

    fn push(&mut self, tk: TokenStream, ty: LiveHookType) -> () {
        match ty {
            LiveHookType::ApplyValueUnknown => push_handle(&mut self.apply_value_unknown, tk),
            LiveHookType::ApplyValueInstance => push_handle(&mut self.apply_value_instance, tk),
            LiveHookType::SkipApplyAnimator => push_handle(&mut self.skip_apply_animator, tk),
            LiveHookType::SkipApply => push_handle(&mut self.skip_apply, tk),
            LiveHookType::BeforeApply => push_handle(&mut self.before_apply, tk),
            LiveHookType::AfterApply => push_handle(&mut self.after_apply, tk),
            LiveHookType::AfterApplyFrom => push_handle(&mut self.after_apply_from, tk),
            LiveHookType::AfterNewFromDoc => push_handle(&mut self.after_new_from_doc, tk),
            LiveHookType::AfterUpdateFromDoc => push_handle(&mut self.after_update_from_doc, tk),
            LiveHookType::AfterApplyFromDoc => push_handle(&mut self.after_apply_from_doc, tk),
            LiveHookType::AfterNewBeforeApply => push_handle(&mut self.after_new_before_apply, tk),
        }
    }
}
