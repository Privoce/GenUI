use crate::{
    builtin::prop::err_from_to,
    model::{widget::role::Role,  Widget, WidgetTemplate, WidgetType},
    visitor::{IdClass, StyleVisitor},
};
use gen_converter::{ConvertStyle, TemplateModel};
use gen_utils::{common::Source, error::Error};
use std::collections::HashMap;

/// 处理template + style的情况
pub fn template_style(
    source: Source,
    template: Option<TemplateModel>,
    style: Option<ConvertStyle>,
    is_entry: bool,
) -> Result<Widget, Error> {
    let template = if let Some(template) = template {
        Some(handle(template, style.as_ref(), &mut vec![])?)
    } else {
        None
    };

    let mut widget = Widget {
        source,
        template,
        script: None,
        is_entry,
        has_plugin: false,
        template_ptrs: None,
    };
    // 执行前需要执行default_script
    let _ = widget.default_script();

    Ok(widget)
}

/// 处理模板， 和single_template的区别是，我们需要确定模版中是否有id或class
/// 如果含有id或class，我们需要从style中获取对应的样式合并到模板中
/// 我不想将这个方法和single_template合并，因为这个方法的逻辑比较复杂，而single_template的逻辑比较简单，合并后会显得很乱并且会导致额外性能开销
fn handle(
    template: TemplateModel,
    styles: Option<&ConvertStyle>,
    chain: &mut Vec<IdClass>,
) -> Result<WidgetTemplate, Error> {
    let is_static = template.is_static();
    let is_define = template.is_component();
    let TemplateModel {
        id,
        class,
        as_prop,
        name,
        mut props,
        callbacks,
        inherits,
        root,
        children,
        ..
    } = template;
    // [处理callbacks] ------------------------------------------------------------------------------------
    if callbacks.is_some() {
        return Err(err_from_to(
            "GenUI Component",
            "Makepad Widget, Static Widget has no callbacks",
        )
        .into());
    }
    // [处理inherits] --------------------------------------------------------------------------------------
    if inherits.is_some() {
        return Err(err_from_to(
            "GenUI Component",
            "Makepad Widget, Static Widget has no inherits",
        )
        .into());
    }
    // [处理节点, 属性, 子组件] ------------------------------------------------------------------------------
    if let Some(styles) = styles.as_ref() {
        let other_props = StyleVisitor::visit(styles, id.as_ref(), class.as_ref(), chain)?;
        // 合并props
        if !other_props.is_empty() {
            if props.is_none() {
                props = Some(HashMap::new());
            }
            if let Some(props) = props.as_mut() {
                for p in other_props {
                    props.extend(p);
                }
            }
        }
    }
 
    let ty = if !is_define {
        WidgetType::try_from((name, props, root))?
    } else {
        WidgetType::Define((name, props, root).try_into()?)
    };

    let children = if let Some(children) = children {
        let mut w_children = vec![];
        chain.push(IdClass {
            id: id.clone(),
            class: class.clone(),
        });
        for child in children {
            let w = handle(child, styles, chain)?;
            w_children.push(w);
        }
        Some(w_children)
    } else {
        None
    };
    Ok(WidgetTemplate {
        id,
        is_root: root,
        as_prop,
        is_static,
        ty,
        children,
        role: Role::default(),
        binds: None,
    })
}
