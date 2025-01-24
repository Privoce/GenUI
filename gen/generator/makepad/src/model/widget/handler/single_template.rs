use gen_converter::TemplateModel;
use gen_utils::{common::Source, error::Error};

use crate::{
    builtin::prop::err_from_to,
    model::{widget::role::Role, Widget, WidgetTemplate, WidgetType},
};

/// 处理单个模板<template>节点
pub fn single_template(
    source: Source,
    template: Option<TemplateModel>,
    is_entry: bool,
) -> Result<Widget, Error> {
    let template = if let Some(template) = template {
        Some(handle(template)?)
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

fn handle(template: TemplateModel) -> Result<WidgetTemplate, Error> {
    // [检查并解析template] ---------------------------------------------------------------------------------
    // - 对于只有<template>节点的.gen文件, 不能带有动态脚本, 不能带有callbacks, 只能是静态组件
    // - 不能含有inherit属性首个标签不能是<component>
    let is_static = template.is_static();
    let is_define = template.is_component();
    let TemplateModel {
        id,
        as_prop,
        name,
        props,
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
    let ty = if !is_define {
        WidgetType::try_from((name, props, root))?
    } else {
        WidgetType::Define((name, props, root).try_into()?)
    };

    let children = if let Some(children) = children {
        let mut w_children = vec![];
        for child in children {
            let w = handle(child)?;
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
