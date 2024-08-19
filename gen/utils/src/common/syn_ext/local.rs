use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse2, parse_str, Expr, ExprBlock, ExprClosure, Ident, Local, LocalInit, Pat, Stmt};

use crate::common::syn_ext::PatTypeGetter;

use super::{MacroConverter, SynEmpty};

#[allow(dead_code)]
pub trait VarGetter {
    fn get(&self, var: &str) -> Option<&Ident>;
    fn get_to_str(&self, var: &str) -> Option<String> {
        self.get(var).map(|x| x.to_string())
    }
    fn has(&self, var: &str) -> bool {
        self.get(var).is_some()
    }
    fn ty(&self, var: &str) -> Option<String>;
}
impl VarGetter for Stmt {
    fn get(&self, var: &str) -> Option<&Ident> {
        if let Stmt::Local(local) = self {
            if let Pat::Ident(ident) = &local.pat {
                if ident.ident.eq(var) {
                    return Some(&ident.ident);
                }
            }
        }
        None
    }

    fn ty(&self, var: &str) -> Option<String> {
        if let Stmt::Local(local) = self {
            let mut flag = false;
            if let Pat::Ident(ident) = &local.pat {
                if ident.ident.eq(var) {
                    // get type
                    flag = true;
                }
            }

            if flag {
                // get init, and get expr lit
                if let Some(init) = &local.init {
                    if let syn::Expr::Lit(lit) = &*init.expr {
                        // wait to make a rustc type analysis
                        return Some(lit.lit.to_token_stream().to_string());
                    }
                }
            }
        }
        None
    }
}

pub trait ClosureGetter {
    /// get closure without body
    /// here we only need to do is: clear init.expr.body
    fn get_without_body(&self) -> Option<TokenStream>;
}

pub trait ClosureSetter {
    fn set_body(&mut self, body: ExprBlock);
}

impl ClosureSetter for Local {
    fn set_body(&mut self, body: ExprBlock) {
        self.init.as_mut().map(|init| {
            if let Expr::Closure(closure) = &mut (*init.expr) {
                closure.body = Box::new(Expr::Block(body));
            }
        });
    }
}

impl ClosureGetter for Local {
    fn get_without_body(&self) -> Option<TokenStream> {
        if let Some(init) = self.init.as_ref() {
            if let Expr::Closure(closure) = &*init.expr {
                // here copy closure and convert to TokenStream without body
                let expr_closure = ExprClosure {
                    attrs: closure.attrs.clone(),
                    lifetimes: closure.lifetimes.clone(),
                    constness: closure.constness,
                    movability: closure.movability,
                    asyncness: closure.asyncness,
                    capture: closure.capture,
                    or1_token: closure.or1_token,
                    inputs: closure.inputs.clone(),
                    or2_token: closure.or2_token,
                    output: closure.output.clone(),
                    body: Box::new(Expr::Block(ExprBlock::empty())),
                };

                Some(
                    Local {
                        attrs: self.attrs.clone(),
                        let_token: self.let_token,
                        pat: self.pat.clone(),
                        init: Some(LocalInit {
                            eq_token: init.eq_token,
                            expr: Box::new(Expr::Closure(expr_closure)),
                            diverge: init.diverge.clone(),
                        }),
                        semi_token: self.semi_token,
                    }
                    .to_token_stream(),
                )
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// # Closure Converter for Local
/// only use in GenUI
///
pub trait ClosureConverter {
    /// get and convert target ident to `self.ident_name` and add two-way binding
    /// `F: Fn(Vec<(origin_prop, after_prop)>) -> Option<TokenStream>`
    /// - origin_prop: origin prop name (instance_name.xxx)
    /// - after_prop: after replaced prop name (self.xxx)
    fn prop_to_self_binding<F>(
        &self,
        instance: Option<&Ident>,
        fields: Option<&Vec<Ident>>,
        prop_binds: F,
    ) -> Option<TokenStream>
    where
        F: Fn(Vec<(String, String)>) -> Option<TokenStream>;
}

impl ClosureConverter for Stmt {
    fn prop_to_self_binding<F>(
        &self,
        instance: Option<&Ident>,
        fields: Option<&Vec<Ident>>,
        prop_binds: F,
    ) -> Option<TokenStream>
    where
        F: Fn(Vec<(String, String)>) -> Option<TokenStream>,
    {
        if instance.is_none() || fields.is_none() {
            return None;
        }
        // replacer is : `instance.fields` and instance itSelf
        let instance = instance.expect("instance is neeed").to_string();
        let replacer = if fields.is_some() {
            let mut res = fields.unwrap().iter().fold(Vec::new(), |mut acc, field| {
                acc.push((&instance, field.to_string()));
                acc
            });
            res.push((&instance, "".to_string()));
            res
        } else {
            vec![(&instance, "".to_string())]
        };

        return if let Stmt::Local(local) = self {
            let mut tk = TokenStream::new();
            // props need to draw
            let mut draw_props = vec![];
            // get closure name --------------------------------------------------------------------------------------
            let closure_name = if let Pat::Ident(ident) = &local.pat {
                Some(&ident.ident)
            } else {
                None
            }
            .unwrap();
            // get closure args and loop closure body------------------------------------------------------------------
            let (closure_args, closure_body) = if let Some(init) = &local.init {
                if let Expr::Closure(closure) = init.expr.as_ref() {
                    // args -------------------------------------------------------------------------------------------
                    let args = parse_str::<TokenStream>(
                        &closure
                            .inputs
                            .iter()
                            .fold(Vec::new(), |mut acc, item| {
                                let _ = item.get().map(|(k, _)| {
                                    acc.push(k.to_string());
                                });
                                acc
                            })
                            .join(", "),
                    )
                    .unwrap();
                    // body -------------------------------------------------------------------------------------------
                    let body =
                        if let Expr::Block(block) = &*closure.body {
                            Some(block.block.stmts.iter().fold(
                                TokenStream::new(),
                                |mut acc, stmt| {
                                    // check stmt, if is macro, judge is active! macro, if is, convert to cx.widget_action
                                    match stmt {
                                        Stmt::Macro(mac) => {
                                            let _ = mac.active_macro_to_action().map(|x| {
                                                acc.extend(x);
                                            });
                                        }
                                        other => {
                                            // convert each stmt to string and check if has target replacer, if exist, replace it
                                            for (k, v) in &replacer {
                                                draw_props.push((
                                                    format!("{}.{}", k, v),
                                                    format!("self . {}", v),
                                                ));
                                                let (old, new) = if v.is_empty() {
                                                    let old = other.to_token_stream().to_string();
                                                    let new =
                                                        old.replace(k.as_str(), &format!("self"));
                                                    (old, new)
                                                } else {
                                                    let old = other.to_token_stream().to_string();
                                                    let new = old.replace(
                                                        &format!("{} . {}", k, v),
                                                        &format!("self . {}", v),
                                                    );
                                                    (old, new)
                                                };

                                                if old != new {
                                                    acc.extend(
                                                        parse_str::<TokenStream>(&new).unwrap(),
                                                    );
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                    acc
                                },
                            ))
                        } else {
                            None
                        };

                    (Some(args), body)
                } else {
                    (None, None)
                }
            } else {
                (None, None)
            };

            let mut new_local = local.clone();
            new_local.set_body({
                let body = closure_body.expect("closure body is needed");
                parse2::<ExprBlock>(quote! {
                    {#body}
                })
                .unwrap()
            });
            // add closure call ---------------------------------------------------------------------------------------
            let draw_prop = prop_binds(draw_props);
            tk.extend(quote! {
                #new_local
                let _ = #closure_name(#closure_args);
                #draw_prop
            });
            Some(tk)
        } else {
            None
        };
    }
}
