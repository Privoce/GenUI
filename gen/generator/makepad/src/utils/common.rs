use gen_utils::common::*;
use proc_macro2::{Span, TokenStream, TokenTree};
use string::FixedString;
use syn::{
    punctuated::Punctuated,
    token::{Bracket, Colon, Pound, Pub},
    Field, Ident, Path,
};

use super::{derive_macros, id_macro};

/// generate `live!{ //.. }`
pub fn live_macro(code: Vec<TokenTree>) -> Vec<TokenTree> {
    vec![
        token_tree_ident("live"),
        token_tree_punct_alone('!'),
        token_tree_group(code),
    ]
}

/// generate `self.[ui_name].[tag_name].apply_over_and_redraw(cx, live!{...});`
pub fn apply_over_and_redraw(
    ui: Option<String>,
    tag: &str,
    id: &str,
    pv: Vec<TokenTree>,
) -> Vec<TokenTree> {
    let mut f = vec![token_tree_ident("self"), token_tree_punct_joint('.')];
    if ui.is_some() {
        f.push(token_tree_ident(ui.unwrap().as_str()));
        f.push(token_tree_punct_alone('.'));
    }

    f.extend(vec![
        token_tree_ident(tag),
        token_tree_group_paren(vec![
            token_tree_ident("id"),
            token_tree_punct_alone('!'),
            token_tree_group_paren(vec![token_tree_ident(&id)]),
        ]),
        token_tree_punct_alone('.'),
        token_tree_ident("apply_over_and_redraw"),
        token_tree_group_paren(vec![
            token_tree_ident("cx"),
            token_tree_punct_alone(','),
            token_tree_ident("live"),
            token_tree_punct_alone('!'),
            token_tree_group(pv),
        ]),
        token_tree_punct_alone(';'),
    ]);
    f
}

/// ```
/// #[derive(Debug, Clone, Default)]
/// struct Instance {
///   pub field: Type,
/// }
/// ```
pub fn instance(kvs: Vec<TokenTree>) -> Vec<TokenTree> {
    let mut tk = derive_macros(vec!["Debug", "Clone", "Default"]);
    tk.extend(vec![
        token_tree_ident("struct"),
        token_tree_ident("Instance"),
        token_tree_group(kvs),
    ]);
    tk
}

/// generate `impl xxx{...}`
pub fn impl_target(target: &str, code: Vec<TokenTree>) -> Vec<TokenTree> {
    vec![
        token_tree_ident("impl"),
        token_tree_ident(target),
        token_tree_group(code),
    ]
}

/// generate `fn new()->Self{}`
pub fn instance_new_fn(code: Vec<TokenTree>) -> Vec<TokenTree> {
    vec![
        token_tree_ident("fn"),
        token_tree_ident("new"),
        token_tree_group_paren(vec![]),
        token_tree_punct_joint('-'),
        token_tree_punct_joint('>'),
        token_tree_ident("Self"),
        token_tree_group(code),
    ]
}

/// generate `...init code...; Self{...}`
pub fn instance_return_self(mut init: Vec<TokenTree>, code: Vec<TokenTree>) -> Vec<TokenTree> {
    init.extend(vec![token_tree_ident("Self"), token_tree_group(code)]);
    init
}
/// generate `self.instance = Instance::new();`
pub fn instance_new() -> Vec<TokenTree> {
    vec![
        token_tree_ident("self"),
        token_tree_punct_alone('.'),
        token_tree_ident("instance"),
        token_tree_punct_alone('='),
        token_tree_ident("Instance"),
        token_tree_punct_joint(':'),
        token_tree_punct_joint(':'),
        token_tree_ident("new"),
        token_tree_group_paren(vec![]),
        token_tree_punct_alone(';'),
    ]
}

/// generate `pub field: Type,`
pub fn struct_field_type(field: &str, ty: TokenTree) -> Vec<TokenTree> {
    vec![
        token_tree_ident("pub"),
        token_tree_ident(field),
        token_tree_punct_alone(':'),
        ty,
        token_tree_punct_alone(','),
    ]
}

pub fn if_group(condition: Vec<TokenTree>, code: Vec<TokenTree>) -> Vec<TokenTree> {
    let mut if_tk = vec![token_tree_ident("if")];
    if_tk.extend(condition);
    if_tk.push(token_tree_group(code));
    if_tk
}

/// generate `self.[ui_name].tag_name(id_macro!(id)).event_name(&actions){...}`
pub fn self_event_react(
    ui: Option<String>,
    tag: &str,
    id: &str,
    event: &str,
    code: Vec<TokenTree>,
) -> Vec<TokenTree> {
    let mut tk = vec![token_tree_ident("self")];

    if ui.is_some() {
        tk.push(token_tree_punct_alone('.'));
        tk.push(token_tree_ident(ui.unwrap().as_str()));
    }

    tk.extend(vec![
        token_tree_punct_alone('.'),
        token_tree_ident(tag),
        token_tree_group_paren(id_macro(id)),
        token_tree_punct_alone('.'),
        token_tree_ident(event),
        token_tree_group_paren(vec![
            token_tree_punct_alone('&'),
            token_tree_ident("actions"),
        ]),
        token_tree_group(code),
    ]);

    tk
}

/// generate widget first node which may define special struct in following script
/// - is_static = true: `special = <BuiltIn>{...}`
/// - is_static = flase: `special = {{special}}{...}`
pub fn special_struct(
    ui: &str,
    ptr: &str,
    code: Option<TokenStream>,
    is_static: bool,
) -> Vec<TokenTree> {
    let mut tk = if is_static {
        
        vec![
            token_tree_ident(ui),
            token_tree_punct_alone('='),
            token_tree_punct_joint('<'),
            token_tree_ident(&ptr.snake_to_camel()),
            token_tree_punct_joint('>'),
        ]
    } else {
        vec![
            token_tree_ident(ui),
            token_tree_punct_alone('='),
            token_tree_punct_joint('{'),
            token_tree_punct_alone('{'),
            token_tree_ident(ptr),
            token_tree_punct_joint('}'),
            token_tree_punct_joint('}'),
        ]
    };

    if let Some(code) = code {
        tk.push(token_tree_group(token_stream_to_tree(code)));
    }
    tk
}

/// generate `[id] :|= <tag_name>{...prop...}`
/// - `:`: is app main [0]
/// - `=`: common
pub fn component_render(
    id: Option<&String>,
    is_root: bool,
    is_component: bool,
    as_prop: bool,
    tag: &str,
    props: Option<TokenStream>,
    children: Option<TokenStream>,
) -> Vec<TokenTree> {
    let mut tk = Vec::new();
    if id.is_some() {
        tk.push(token_tree_ident(id.unwrap()));

        if as_prop {
            tk.push(token_tree_punct_alone(':'));
        } else {
            match (is_component, is_root) {
                (false, true) => tk.push(token_tree_punct_alone(':')),
                _ => tk.push(token_tree_punct_alone('=')),
            };
        }
    }

    tk.extend(vec![
        token_tree_punct_alone('<'),
        token_tree_ident(tag),
        token_tree_punct_joint('>'),
    ]);

    //先加props再加chidren
    let mut props_children = vec![];

    if props.is_some() {
        props_children.extend(props.unwrap());
    }
    if children.is_some() {
        props_children.extend(children.unwrap());
    }
    tk.push(token_tree_group(props_children));

    tk
}

/// generate `self.[ui].handle_event(cx, event, &mut Scope::empty());`
pub fn self_handle_event(target: Option<String>) -> Vec<TokenTree> {
    let mut tk = vec![token_tree_ident("self")];
    if target.is_some() {
        tk.push(token_tree_punct_alone('.'));
        tk.push(token_tree_ident(target.unwrap().as_str()));
    }
    tk.extend(vec![
        token_tree_punct_alone('.'),
        token_tree_ident("handle_event"),
        token_tree_group_paren(vec![
            token_tree_ident("cx"),
            token_tree_punct_alone(','),
            token_tree_ident("event"),
            token_tree_punct_alone(','),
            token_tree_punct_alone('&'),
            token_tree_ident("mut"),
            token_tree_ident("Scope"),
            token_tree_punct_joint(':'),
            token_tree_punct_alone(':'),
            token_tree_ident("empty"),
            token_tree_group_paren(vec![]),
        ]),
        token_tree_punct_alone(';'),
    ]);
    tk
}

/// generate `match event{...}`
pub fn match_event(code: Vec<TokenTree>) -> Vec<TokenTree> {
    vec![
        token_tree_ident("match"),
        token_tree_ident("event"),
        token_tree_group(code),
    ]
}
/// generate `enum => {...},`
pub fn match_item(enum_item: Vec<TokenTree>, code: Vec<TokenTree>) -> Vec<TokenTree> {
    let mut tk = enum_item;

    tk.extend(vec![
        token_tree_punct_joint('='),
        token_tree_punct_alone('>'),
        token_tree_group(code),
        token_tree_punct_alone(','),
    ]);

    tk
}

/// generate `_ => (),`
pub fn match_other() -> Vec<TokenTree> {
    vec![
        token_tree_ident("_"),
        token_tree_punct_joint('='),
        token_tree_punct_alone('>'),
        token_tree_group_paren(vec![]),
        token_tree_punct_alone(','),
    ]
}

pub fn event_enum_item(item: &str) -> Vec<TokenTree> {
    vec![
        token_tree_ident("Event"),
        token_tree_punct_joint(':'),
        token_tree_punct_joint(':'),
        token_tree_ident(item),
    ]
}

/// generate `Event::Startup`
pub fn event_start_up() -> Vec<TokenTree> {
    event_enum_item("Startup")
}
pub fn event_shutdown() -> Vec<TokenTree> {
    event_enum_item("Shutdown")
}
/// `self.handle_startup(cx),`
pub fn self_handle_startup() -> Vec<TokenTree> {
    vec![
        token_tree_ident("self"),
        token_tree_punct_alone('.'),
        token_tree_ident("handle_startup"),
        token_tree_group_paren(vec![token_tree_ident("cx")]),
    ]
}

pub fn struct_field(attrs: Vec<&str>, ident: &str, seg: &str) -> Field {
    let attrs = attrs
        .iter()
        .map(|attr| {
            let attr = syn::Ident::new(attr, proc_macro2::Span::call_site());
            let mut segments = Punctuated::new();
            let path = syn::PathSegment {
                ident: attr,
                arguments: syn::PathArguments::None,
            };
            segments.push(path);
            syn::Attribute {
                pound_token: Pound::default(),
                style: syn::AttrStyle::Outer,
                bracket_token: Bracket::default(),
                meta: syn::Meta::Path(Path {
                    leading_colon: None,
                    segments,
                }),
            }
        })
        .collect();

    let mut segments = Punctuated::new();
    let path = syn::PathSegment {
        ident: Ident::new(seg, Span::call_site()),
        arguments: syn::PathArguments::None,
    };
    segments.push(path);

    Field {
        attrs,
        vis: syn::Visibility::Public(Pub::default()),
        mutability: syn::FieldMutability::None,
        ident: Some(Ident::new(ident, Span::call_site())),
        colon_token: Some(Colon::default()),
        ty: syn::Type::Path(syn::TypePath {
            qself: None,
            path: Path {
                leading_colon: None,
                segments,
            },
        }),
    }
}

pub fn float_to_str(num: f32) -> String {
    float_to_str_f64(num as f64)
}

pub fn float_to_str_f64(num: f64) -> String {
    if num.fract() == 0.0 {
        format!("{}.0", num)
    } else {
        format!("{}", num)
    }
}
