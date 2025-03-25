use gen_utils::error::{CompilerError, Error};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use rssyin::bridger::RouterTk;

use crate::{
    compiler::{Context, RouterBuilder, TabbarItem},
    str_to_tk,
    token::{import_default, import_default_all, use_default_all},
};

#[derive(Debug, Clone)]
pub struct RouterScript(pub RouterBuilder);

impl RouterScript {}

impl TryFrom<(RouterTk, &mut Context)> for RouterScript {
    type Error = Error;

    fn try_from(value: (RouterTk, &mut Context)) -> Result<Self, Self::Error> {
        // 从context中获取对应的router
        if let Some(routers) = value.1.routers.as_ref() {
            if let Some(builder) = routers.get(&value.0 .0) {
                return Ok(Self(builder.clone()));
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

        let uses = use_default_all();
        let component = str_to_tk!(&self.0.name).unwrap();
        let mut imports = import_default();
        let router_id = str_to_tk!(&self.0.id).unwrap();
        let nav_mode = self.0.mode.to_token_stream();

        let (bar_pages, tabbar_items, used_items) = self.0.bar_pages.iter().fold(
            (TokenStream::new(), TokenStream::new(), Vec::new()),
            |(mut bars, mut tabbar_items, mut used_items), (key, page)| {
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

                (bars, tabbar_items, used_items)
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

        let nav_pages = self.0.nav_pages.iter().fold(TokenStream::new(), |mut acc, (key, page)| {
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

            acc.extend(quote! {
                #page_id = <GNavPage>{
                    header = {
                        visible: false,
                    }
                    body = <#page_name>{}
                }
            });

            acc
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
        });
    }
}
