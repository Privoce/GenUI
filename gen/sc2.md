```
code: Stmt::Local {
                attrs: [],
                let_token: Let,
                pat: Pat::Ident {
                    attrs: [],
                    by_ref: None,
                    mutability: Some(
                        Mut,
                    ),
                    ident: Ident(
                        toggle,
                    ),
                    subpat: None,
                },
                init: Some(
                    LocalInit {
                        eq_token: Eq,
                        expr: Expr::Closure {
                            attrs: [],
                            lifetimes: None,
                            constness: None,
                            movability: None,
                            asyncness: None,
                            capture: None,
                            or1_token: Or,
                            inputs: [],
                            or2_token: Or,
                            output: ReturnType::Default,
                            body: Expr::Block {
                                attrs: [],
                                label: None,
                                block: Block {
                                    brace_token: Brace,
                                    stmts: [
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
                                },
                            },
                        },
                        diverge: None,
                    },
                ),
                semi_token: Semi,
            },
            is_prop: false,
        },
    ],
)
```