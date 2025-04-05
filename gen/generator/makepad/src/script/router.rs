use gen_utils::error::{CompilerError, Error};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use rssyin::bridger::RouterTk;
use syn::{parse_quote, Fields};

use crate::{
    compiler::{Context, RouterBuilder, TabbarItem},
    script::RsScript,
    str_to_tk,
    token::{import_default, use_router},
};

#[derive(Debug, Clone)]
pub struct RouterScript(pub RouterBuilder);

impl RouterScript {}

impl TryFrom<(RouterTk, &mut Context)> for RouterScript {
    type Error = Error;

    fn try_from(value: (RouterTk, &mut Context)) -> Result<Self, Self::Error> {
        // 从context中获取对应的router
        if let Some(router) = value.1.router.as_ref() {
            if router.id == value.0 .0 {
                return Ok(Self(router.clone()));
            }
        }
        Err(CompilerError::Conf(format!(
            "{} router can not found in context, please check!",
            value.0 .0
        ))
        .into())
    }
}

impl ToTokens for RouterScript {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        fn tabbar_item_to_tokens(item: &TabbarItem) -> TokenStream {
            let icon = item.icon.as_ref().map(|icon| {
                quote! {
                    icon_slot: {src: #icon},
                }
            });

            let text = item.text.as_ref().map(|text| {
                let text = str_to_tk!(text).unwrap();
                quote! {
                    text_slot: {text: #text},
                }
            });
            quote! {
                <GTabbarItem>{
                    #icon
                    #text
                }
            }
        }

        let uses = use_router();
        let active = self.0.active.as_ref().map(|active| {
            let active = str_to_tk!(active).unwrap();
            quote! {
                active(id!(#active))
            }
        });
        let component = str_to_tk!(&self.0.name).unwrap();
        let mut imports = import_default();
        let router_id = str_to_tk!(&self.0.id).unwrap();
        let nav_mode = self.0.mode.to_token_stream();

        let (bar_pages, bar_pages_ids, tabbar_items, used_items) = self.0.bar_pages.iter().fold(
            (
                TokenStream::new(),
                Vec::new(),
                TokenStream::new(),
                Vec::new(),
            ),
            |(mut bars, mut ids, mut tabbar_items, mut used_items), (key, page)| {
                let page_id = str_to_tk!(key).unwrap();
                let page_name = match page {
                    crate::compiler::Page::Path(import) => {
                        imports.extend(import.to_token_stream());
                        import.component().unwrap()
                    }
                    crate::compiler::Page::Component { path, component } => {
                        imports.extend(path.to_token_stream());
                        str_to_tk!(component).unwrap()
                    }
                };
                ids.push(page_id.clone());
                bars.extend(quote! {
                    #page_id = <GBarPage>{
                        <#page_name>{}
                    }
                });

                self.0.tabbar.as_ref().map(|tabbar| {
                    if tabbar.active {
                        tabbar.bars.get(key).map(|value| {
                            used_items.push(key.to_string());
                            tabbar_items.extend(tabbar_item_to_tokens(value));
                        });
                    }
                });

                (bars, ids, tabbar_items, used_items)
            },
        );

        let tabbar = self.0.tabbar.as_ref().and_then(|tabbar| {
            if tabbar.active {
                let theme = tabbar.theme.map(|theme| {
                    quote! {
                        theme: #theme,
                    }
                });

                let bars =
                    tabbar
                        .bars
                        .iter()
                        .fold(TokenStream::new(), |mut acc, (bar_key, bar_val)| {
                            if !used_items.contains(bar_key) {
                                acc.extend(tabbar_item_to_tokens(bar_val));
                            }
                            acc
                        });

                Some(quote! {
                    tabbar = <GTabbar>{
                        #theme
                        #tabbar_items
                        #bars
                    }
                })
            } else {
                None
            }
        });

        let (nav_pages, nav_pages_ids) = self.0.nav_pages.iter().fold(
            (TokenStream::new(), Vec::new()),
            |(mut acc, mut ids), (key, page)| {
                let page_id = str_to_tk!(key).unwrap();
                let page_name = match page {
                    crate::compiler::Page::Path(import) => {
                        imports.extend(import.to_token_stream());
                        import.component().unwrap()
                    }
                    crate::compiler::Page::Component { path, component } => {
                        imports.extend(path.to_token_stream());
                        str_to_tk!(component).unwrap()
                    }
                };
                ids.push(page_id.clone());
                acc.extend(quote! {
                    #page_id = <GNavPage>{
                        header = {
                            visible: false,
                        }
                        body = <#page_name>{}
                    }
                });

                (acc, ids)
            },
        );

        let nav_pages_ids = if nav_pages_ids.is_empty() {
            quote! {None}
        } else {
            quote! {Some(ids!(#(#nav_pages_ids),*))}
        };

        let mut script = RsScript::default(component.clone());
        // add `#[rust]lifetime: Lifetime`, for router LiveComponent
        script.live_component.as_mut().map(|c| {
            if let Fields::Named(fields) = &mut c.0.fields {
                fields.named.push(parse_quote! {
                    #[rust] lifetime: Lifetime
                });
            }
        });
        script.impls.as_mut().map(|impls| {
            impls.traits().widget.draw_walk = quote! {
                let _ = self.deref_widget.draw_walk(cx, scope, walk);
                self.lifetime
                    .init()
                    .execute(|| {
                        let router = self.grouter(id!(app_router));
                        router.borrow_mut().map(|mut router| {
                            let _ = router
                                .init(
                                    ids!(#(#bar_pages_ids),*),
                                    #nav_pages_ids,
                                    Some(RouterIndicatorMode::Define),
                                )
                                .#active
                                .build(cx);
                        });
                    })
                    .map(|_| {
                        let router = self.grouter(id!(app_router));
                        router.borrow().map(|router| {
                            if router.scope_path.is_some() {
                                self.lifetime.next();
                            }
                        })
                    });
                DrawStep::done()
            };
            impls.traits().widget.handle_event.other = quote! {
                let router = self.grouter(id!(app_router));
                router.handle_nav_events(cx, &actions);
            };
            impls.self_ref_impl.extend(vec![
                parse_quote! {
                    pub fn nav_to(&self, path: &[LiveId], cx: &mut Cx) {
                        self.borrow_mut().map(|router| {
                            let router = router.grouter(id!(app_router));
                            router.nav_to(cx, path);
                        });
                    }

                },
                parse_quote! {
                    pub fn nav_back(&self, cx: &mut Cx) {
                        self.borrow_mut().map(|router| {
                            let router = router.grouter(id!(app_router));
                            router.nav_back(cx);
                        });
                    }
                },
            ]);
        });

        tokens.extend(quote! {
            #uses
            live_design!{
                #imports
                pub #component = {{#component}}{
                    #router_id = <GRouter>{
                        nav_mode: #nav_mode,
                        bar_pages = {
                            #bar_pages
                            #tabbar
                        },
                        nav_pages = {
                            #nav_pages
                        }
                    }
                }
            }
            #script
        });
    }
}
