use crate::{script::Impls, str_to_tk, traits::MakepadExtComponent};
use gen_analyzer::{Binds, SugarIf};
use gen_utils::error::{CompilerError, Error};
use quote::{quote, ToTokens};
use syn::{parse_quote, ExprArray, ImplItem, ImplItemFn, Stmt};

pub struct ComputedVisitor;

/// ## 计算属性 (e.g.: #[computed(a, b, c)] )
/// 计算属性是GenUI中的一个重要概念，它是一个特殊的方法，使用`#[computed(${link_arg}...)]`进行方法标记，表示这个方法是一个计算属性
/// 计算属性需要关联到组件属性上，这样在组件属性进行变化时，计算属性会自动更新。对计算属性我们需要生成：
/// 1. 去除方法上的`#[computed(${link_arg}...)]`标记
/// 2. 生成`update_${computed_fn_name}`方法, 该方法用于更新计算属性和对应组件
/// 3. 找到宏中与计算属性关联的组件属性的setter方法，将更新方法添加到setter方法中
impl ComputedVisitor {
    pub fn visit(
        item_fn: &mut ImplItemFn,
        args: ExprArray,
        impls: &mut Impls,
        binds: Option<&Binds>,
    ) -> Result<(), Error> {
        // [去除方法上的属性宏] ------------------------------------------------------------------------------------
        item_fn
            .attrs
            .retain(|attr| !attr.path().is_ident("computed"));

        // [为方法添加cx参数] ------------------------------------------------------------------------------------
        item_fn.sig.inputs.push(parse_quote! {cx: &mut Cx});
        // [生成更新方法] ------------------------------------------------------------------------------------------
        let fn_name = item_fn.sig.ident.to_token_stream();
        let update_fn_name = str_to_tk!(&format!("update_{}", fn_name.to_string()))?;

        let bind_components = binds
            .map(|binds| binds.get(&fn_name.to_string()).map(|bind| bind.clone()))
            .flatten()
            .ok_or(CompilerError::runtime(
                "Makepad Plugin - Script",
                "can not find bind for computed method",
            ))?;

        for bind_component in bind_components {
            let widget_id = str_to_tk!(&bind_component.id)?;
            let widget = str_to_tk!(&bind_component.name())?;
            let set_fn = str_to_tk!(&format!(
                "set_{}",
                if SugarIf::SUGAR_SIGNS.contains(&bind_component.prop.as_str()) {
                    "visible"
                } else {
                    bind_component.prop.as_str()
                }
            ))?;
            // 如果是Else的话，需要对value进行累加
            let new_value_fn = match bind_component.prop {
                gen_analyzer::Prop::Value(_) => {
                    quote! {self.#fn_name(cx)}
                }
                gen_analyzer::Prop::Else(items) => str_to_tk!(&items
                    .iter()
                    .map(|item| format!("!self.{}(cx)", item))
                    .collect::<Vec<_>>()
                    .join(" && "))?,
            };

            let fn_block = quote! {
                let new_value = #new_value_fn;
                let widget = self.#widget(id!(#widget_id));
                widget.#set_fn(cx, new_value)?;
            };

            // 如果之前已经存在更新方法，则需要将代码附加到更新方法中，否则创建新的更新方法
            if let Some(update_fn) = impls.self_impl.get_mut_fn(&update_fn_name.to_string()) {
                let mut index = update_fn.block.stmts.len() - 1;
                let stmts: Vec<Stmt> = parse_quote!(#fn_block);
                for stmt in stmts {
                    update_fn.block.stmts.insert(index, stmt);
                    index += 1;
                }
            } else {
                let update_fn: ImplItem = parse_quote! {
                    fn #update_fn_name(&mut self, cx: &mut Cx) -> Result<(), Box<dyn std::error::Error>> {
                        #fn_block
                        Ok(())
                    }
                };

                impls.self_impl.push(update_fn);
            }
        }

        // [从args中获取绑定的组件属性并将更新方法添加到对应的setter中] -------------------------------------------------
        for arg in args.elems.iter() {
            let set_fn = format!("set_{}", arg.to_token_stream().to_string());

            if let Some(item_fn) = impls.self_impl.get_mut_fn(&set_fn) {
                let index = item_fn.block.stmts.len() - 1;
                item_fn.block.stmts.insert(
                    index,
                    parse_quote! {
                        self.#update_fn_name(cx)?;
                    },
                );
            }
        }

        Ok(())
    }
}
