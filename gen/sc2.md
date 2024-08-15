```
[E:\Rust\try\makepad\Gen-UI\gen\generator\makepad\src\widget\model\widget.rs:183:37] &sc = GenScriptModel {
    sub_prop_binds: None,
    sub_event_binds: None,
    current_instance: Some(
        CurrentInstance {
            is_mut: true,
            name: Some(
                Ident(
                    props,
                ),
            ),
            ptr: Ident(
                RootComponent,
            ),
        },
    ),
    instance_opt: Some(
        [
            Stmt::Expr(
                Expr::Assign {
                    attrs: [],
                    left: Expr::Field {
                        attrs: [],
                        base: Expr::Path {
                            attrs: [],
                            qself: None,
                            path: Path {
                                leading_colon: None,
                                segments: [
                                    PathSegment {
                                        ident: Ident(
                                            props,
                                        ),
                                        arguments: PathArguments::None,
                                    },
                                ],
                            },
                        },
                        dot_token: Dot,
                        member: Member::Named(
                            Ident(
                                flag1,
                            ),
                        ),
                    },
                    eq_token: Eq,
                    right: Expr::Lit {
                        attrs: [],
                        lit: Lit::Bool {
                            value: false,
                        },
                    },
                },
                Some(
                    Semi,
                ),
            ),
        ],
    ),
    other: Some(
        [
            Stmt::Item(
                Item::Impl {
                    attrs: [],
                    defaultness: None,
                    unsafety: None,
                    impl_token: Impl,
                    generics: Generics {
                        lt_token: None,
                        params: [],
                        gt_token: None,
                        where_clause: None,
                    },
                    trait_: Some(
                        (
                            None,
                            Path {
                                leading_colon: None,
                                segments: [
                                    PathSegment {
                                        ident: Ident(
                                            Default,
                                        ),
                                        arguments: PathArguments::None,
                                    },
                                ],
                            },
                            For,
                        ),
                    ),
                    self_ty: Type::Path {
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment {
                                    ident: Ident(
                                        RootComponent,
                                    ),
                                    arguments: PathArguments::None,
                                },
                            ],
                        },
                    },
                    brace_token: Brace,
                    items: [
                        ImplItem::Fn {
                            attrs: [],
                            vis: Visibility::Inherited,
                            defaultness: None,
                            sig: Signature {
                                constness: None,
                                asyncness: None,
                                unsafety: None,
                                abi: None,
                                fn_token: Fn,
                                ident: Ident(
                                    default,
                                ),
                                generics: Generics {
                                    lt_token: None,
                                    params: [],
                                    gt_token: None,
                                    where_clause: None,
                                },
                                paren_token: Paren,
                                inputs: [],
                                variadic: None,
                                output: ReturnType::Type(
                                    RArrow,
                                    Type::Path {
                                        qself: None,
                                        path: Path {
                                            leading_colon: None,
                                            segments: [
                                                PathSegment {
                                                    ident: Ident(
                                                        Self,
                                                    ),
                                                    arguments: PathArguments::None,
                                                },
                                            ],
                                        },
                                    },
                                ),
                            },
                            block: Block {
                                brace_token: Brace,
                                stmts: [
                                    Stmt::Expr(
                                        Expr::Struct {
                                            attrs: [],
                                            qself: None,
                                            path: Path {
                                                leading_colon: None,
                                                segments: [
                                                    PathSegment {
                                                        ident: Ident(
                                                            Self,
                                                        ),
                                                        arguments: PathArguments::None,
                                                    },
                                                ],
                                            },
                                            brace_token: Brace,
                                            fields: [
                                                FieldValue {
                                                    attrs: [],
                                                    member: Member::Named(
                                                        Ident(
                                                            flag1,
                                                        ),
                                                    ),
                                                    colon_token: Some(
                                                        Colon,
                                                    ),
                                                    expr: Expr::Lit {
                                                        attrs: [],
                                                        lit: Lit::Bool {
                                                            value: true,
                                                        },
                                                    },
                                                },
                                            ],
                                            dot2_token: None,
                                            rest: None,
                                        },
                                        None,
                                    ),
                                ],
                            },
                        },
                    ],
                },
            ),
        ],
    ),
}
```