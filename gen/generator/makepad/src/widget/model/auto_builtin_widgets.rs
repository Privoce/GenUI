use std::collections::HashMap;

use gen_parser::{For, PropsKey, Value};
use gen_utils::{
    common::{fs, snake_to_camel, Source, Ulid},
    error::{Errors, FsError},
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, parse_str, Ident};

use crate::ToToken;

use super::{live_design::LiveDesign, role::Role, safe_widget::SafeWidget};

pub trait AutoBuiltinCompile {
    fn before_compile<P>(&self, path: P) -> Result<(), Errors>
    where
        P: AsRef<std::path::Path>;
    /// widget -> safe_widget (if role is for or if_else) -> insert into AUTO_BUILTIN_WIDGETS -> AutoBuiltinWidgets -> compile
    /// this fn will return a vec of live_register!
    fn compile<P>(&self, path: P) -> Option<Vec<String>>
    where
        P: AsRef<std::path::Path>;
}

impl AutoBuiltinCompile for Vec<SafeWidget> {
    fn before_compile<P>(&self, path: P) -> Result<(), Errors>
    where
        P: AsRef<std::path::Path>,
    {
        // judget if src/auto dir exists, if exists, remove all files in it, if not exists, create it
        let auto_dir = path.as_ref().join("src").join("auto");
        if auto_dir.exists() {
            std::fs::remove_dir_all(auto_dir.as_path()).map_err(|e| {
                Errors::FsError(FsError::Delete {
                    path: path.as_ref().to_path_buf(),
                    reason: e.to_string(),
                })
            })?;
        }
        std::fs::create_dir(auto_dir.as_path()).map_err(|e| {
            Errors::FsError(FsError::Create {
                path: path.as_ref().to_path_buf(),
                reason: e.to_string(),
            })
        })
    }
    fn compile<P>(&self, path: P) -> Option<Vec<String>>
    where
        P: AsRef<std::path::Path>,
    {
        if self.is_empty() {
            return None;
        }
        let mut registers = vec![];
        for widget in self {
            match &widget.role {
                Role::For {
                    id,
                    credential,
                    loop_type,
                    props,
                } => {
                    let (source, live_design) =
                        for_widget_to_live_design(widget, id, credential, loop_type, props);

                    // insert target mod into auto/mod.rs
                    fs::append(
                        path.as_ref(),
                        &format!(
                            "#[allow(non_snake_case)] pub mod {}; ",
                            source.compiled_file.file_stem().unwrap().to_str().unwrap()
                        ),
                    )
                    .expect("insert auto builtin widget mod failed");
                    registers.push(format!(
                        "crate::{}::live_design(cx);",
                        source.to_live_register()
                    ));
                    // now should compile to source file
                    let _ = fs::write(
                        source.compiled_file.as_path(),
                        &live_design.to_token_stream().to_string(),
                    )
                    .expect("write auto builtin widget source file failed");
                }
                Role::If { .. } => {
                    todo!("if_else widget compile");
                }
                _ => {}
            }
        }
        Some(registers)
    }
}

fn for_widget_to_live_design(
    widget: &SafeWidget,
    ulid: &Ulid,
    credential: &For,
    loop_type: &str,
    props: &HashMap<PropsKey, Value>,
) -> (Source, LiveDesign) {
    let origin_widget_name = snake_to_camel(&widget.name).unwrap();
    let mut live_design = LiveDesign::default();
    // get widget source and change compiled_file to xxx/src_gen/src/auto/${source}.rs ---------------------------------------------------------------
    let mut source = widget.source.as_ref().unwrap().clone();
    source.compiled_file = source
        .compiled_dir
        .as_path()
        .join("src")
        .join("auto")
        .join(&format!("{}_{}.rs", &origin_widget_name, ulid));
    // check current widget is define or is static ---------------------------------------------------------------------------------------------------
    if widget.is_static {
        let widget_name =
            parse_str::<TokenStream>(&format!("{}{}", &origin_widget_name, ulid)).unwrap();
        let inner_tree = parse_str::<TokenStream>(widget.tree.as_ref().unwrap()).unwrap();
        // generate widget tree code -----------------------------------------------------------------------------------------------------------------
        let tree = quote! {
            #widget_name = {{#widget_name}}{
                item: #inner_tree
            }
        };
        live_design.tree = Some(tree);
        // generate widget logic ---------------------------------------------------------------------------------------------------------------------
        let loop_ident = parse_str::<TokenStream>(&credential.iter_ident).unwrap();
        let loop_type = parse_str::<TokenStream>(&loop_type).unwrap();
        let origin_ref = parse_str::<TokenStream>(&format!("{}Ref", &origin_widget_name)).unwrap();
        let widget_ref =
            parse_str::<TokenStream>(&format!("{}{}Ref", &origin_widget_name, ulid)).unwrap();
        let live_hook = widget
            .live_hook
            .as_ref()
            .map(|x| x.to_token_stream(parse2::<Ident>(widget_name.clone()).unwrap()));

        let set_loop =
            parse_str::<TokenStream>(&format!("set_{}", &loop_ident.to_string())).unwrap();
        let set_widget_props = if props.is_empty() {
            None
        } else {
            let mut set_props = TokenStream::new();

            for (key, value) in props.iter() {
                let set_key = parse_str::<TokenStream>(&format!(
                    "set_{}({})",
                    key.name(),
                    &value.to_string()
                ))
                .unwrap();

                set_props.extend(quote! {
                    target.#set_key;
                });
            }
            Some(set_props)
        };
        // 注意！这个方法需要处理
        let enumerate = parse_str::<TokenStream>(&credential.fmt_enumerate()).unwrap();

        let logic = quote! {
            #[derive(Live, Widget)]
            pub struct #widget_name {
                #[redraw] #[rust] area: Area,
                #[live] item: Option<LivePtr>,
                #[rust] children: ComponentMap<LiveId, #origin_ref>,
                #[layout] layout: Layout,
                #[walk] walk: Walk,
                #[rust] pub #loop_ident: #loop_type,
            }

            impl Widget for #widget_name {
                fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
                    cx.begin_turtle(walk, self.layout);
                    for #enumerate in self.#loop_ident.iter().enumerate() {
                        let target = self.children.get_or_insert(cx, LiveId(index as u64), |cx|{
                            WidgetRef::new_from_ptr(cx, self.item).as_button()
                        });

                        #set_widget_props
                        target.draw_all(cx, &mut Scope::empty());
                    }
                    cx.end_turtle();
                    self.children.retain_visible();
                    DrawStep::done()
                }
                fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
                    self.children.iter().enumerate().for_each(|(_index, (_id, widget_ref))|{
                        widget_ref.handle_event(cx, event, scope);
                    });
                }
            }

            #live_hook

            impl #widget_ref {
                pub fn #set_loop(&mut self, looper: #loop_type) {
                    if let Some(mut instance) = self.borrow_mut(){
                        instance.#loop_ident = looper;
                    }
                }
            }
        };
        live_design.logic = Some(logic);
    } else {
        todo!("do define widget, not support now");
    }

    (source, live_design)
}
