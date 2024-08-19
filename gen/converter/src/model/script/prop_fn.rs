use gen_parser::{PropsKey, Value};
use gen_utils::common::{string::FixedString, syn_ext::ImplGetter};
use syn::{ItemImpl, Stmt};

use crate::model::PropTree;

/// 组件属性绑定
#[derive(Debug, Clone)]
pub struct PropFn {
    /// 组件名
    pub widget: String,
    /// 组件id
    pub id: String,
    /// 组件属性
    pub key: PropsKey,
    /// 绑定的属性值（它会索引到script中设置的变量或方法）
    /// 例如：`<div :text="div_text" @click="on_click"></div>`
    /// 这里的`div_text`和`on_click`就是绑定的属性值
    /// 但也可能出现`<div :text="props.div_text" @click="on_click"></div>`,这属于从上层传入的属性
    pub ident: Value,
    /// 绑定的属性值对应的代码
    pub code: Stmt,
    /// 是否是由上一层传入的属性
    pub is_prop: bool,
}

/// # PropFnOnly
/// remove the `code` field from `PropFn`
/// now it only use in model.rs: `GenScriptModel.instance_default_impl: Option<(Vec<PropFnOnly>, ItemImpl)>,`
#[derive(Debug, Clone)]
pub struct PropFnOnly {
    /// 组件名
    pub widget: String,
    /// 组件id
    pub id: String,
    /// 组件属性
    pub key: PropsKey,
    /// 绑定的属性值（它会索引到script中设置的变量或方法）
    pub ident: Value,
}

impl From<&PropFn> for PropFnOnly {
    fn from(value: &PropFn) -> Self {
        PropFnOnly {
            widget: value.widget.clone(),
            id: value.id.clone(),
            key: value.key.clone(),
            ident: value.ident.clone(),
        }
    }
}

impl PropFnOnly {
    /// ## get bind value from default impl fields
    /// > Note!: this fn will ignore the bind prop which do not have split sign `.`
    /// if you have following code(item_impl):
    /// ```rust
    /// <label :text="prop.name" id="name_label"></label> // prop.name is bind value
    /// 
    /// // default impl
    /// // ...ignore Default trait impl
    /// Self{
    ///    name: "div_text".to_string(), // field1
    ///    age: 18, // field2
    /// }
    /// ```
    /// then the `prop.name` will insert into `Vec<PropFnOnly>`
    /// it looks like:
    /// ```rust
    /// [
    ///     PropFnOnly {
    ///         widget: "label",
    ///         id: "name_label",
    ///         key: PropsKey {
    ///             name: "text",
    ///             is_style: false,
    ///             ty: Bind,
    ///         },
    ///         ident: Bind(
    ///             Normal(
    ///                 "prop.name",
    ///             ),
    ///         ),
    ///     },
    /// ],
    /// ```
    pub fn filter_default_impl(
        item_impl: &ItemImpl,
        binds: &PropTree,
    ) -> (Vec<PropFnOnly>, ItemImpl) {
        // quick return
        if binds.is_empty() {
            return (Vec::new(), item_impl.clone());
        }

        // filter the default impl
        (
            item_impl.fields().iter().fold(Vec::new(), |mut acc, item| {
                // find from binds depend on item
                let _ = binds
                    .iter()
                    .find_map(|((widget, id), prop_fn_key)| {
                        // check prop_fn_key is some? and then use get fn to get the value, key is the item
                        if let Some(props) = prop_fn_key {
                            props.iter().find_map(|(k, v)| {
                                // such as `props.div_text`, split it by `.`, and judget the second part is equal to item
                                let v_call = v
                                    .is_bind_and_get()
                                    .unwrap()
                                    .get_normal()
                                    .unwrap()
                                    .split_fixed(".");
                                if v_call.len() < 2 {
                                    None
                                } else {
                                    if v_call[1].eq(item) {
                                        Some(PropFnOnly {
                                            widget: widget.clone(),
                                            id: id.clone(),
                                            key: k.clone(),
                                            ident: v.clone(),
                                        })
                                    } else {
                                        None
                                    }
                                }
                            })
                        } else {
                            None
                        }
                    })
                    .map(|p| {
                        acc.push(p);
                    });
                acc
            }),
            item_impl.clone(),
        )
    }
}
