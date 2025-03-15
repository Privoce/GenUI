use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use gen_analyzer::{value::Bind, Polls, Script, Template};
use gen_utils::{common::Source, err_from_to, error::Error};

use crate::{
    compiler::{Context, WidgetPoll},
    model::{role::ForParent, widget::role::Role, AbsWidget, Widget, WidgetTemplate, WidgetType},
};

use super::{TemplatePtrs, TemplateResult};

pub fn template_script(
    context: &mut Context,
    source: Source,
    template: Option<Template>,
    script: Option<Script>,
    is_entry: bool,
    polls: Arc<RwLock<Polls>>,
) -> Result<Widget, Error> {
    // [初始化一些必要的池] ----------------------------------------------------------------------------------
    let mut widget_poll: WidgetPoll = HashMap::new();
    let mut template_ptrs: TemplatePtrs = vec![];
    // [处理template] --------------------------------------------------------------------------------------
    let template = if let Some(template) = template {
        if let TemplateResult::Widget(template) = handle(
            template,
            &mut template_ptrs,
            &mut widget_poll,
            0,
            Role::Normal,
        )? {
            Some(template)
        } else {
            None
        }
    } else {
        None
    };

    // [处理script] ----------------------------------------------------------------------------------------
    let script = if let Some(script) = script {
        Some(crate::script::Script::new(
            script,
            context,
            polls,
            &widget_poll,
            &template_ptrs,
            template.as_ref(),
        )?)
    } else {
        if let Some(ident) = template.as_ref().map(|t| t.root_name()) {
            Some(crate::script::Script::default(ident))
        } else {
            None
        }
    };
    // [处理动态生成语法糖需要的代码] ----------------------------------------------------------------------
    let template_ptrs = if template_ptrs.is_empty() {
        None
    } else {
        Some(template_ptrs)
    };

    let mut widget = Widget {
        source,
        template,
        script,
        is_entry,
        has_plugin: context.plugins.is_some(),
        template_ptrs,
    };

    // 执行前需要执行default_script
    let _ = widget.patch_or_default_script()?;
    Ok(widget)
}

fn handle(
    template: Template,
    template_ptrs: &mut TemplatePtrs,
    widget_poll: &mut WidgetPoll,
    index: usize,
    father_role: Role,
) -> Result<TemplateResult, Error> {
    let is_static = template.is_static();
    let is_define = template.is_component();
    let Template {
        id,
        as_prop,
        name,
        props,
        callbacks,
        inherits,
        root,
        children,
        sugar_props,
        parent,
        binds,
        ..
    } = template;
    // [绑定变量处理] ----------------------------------------------------------------------------------------
    let mut bind_props = HashMap::new();
    if let Some(binds) = binds.as_ref() {
        for (k, v) in binds {
            bind_props.insert(v.as_bind()?.ident(), k.name.to_string());
        }
    }
    // [处理语法糖] -----------------------------------------------------------------------------------------
    // - [for] --------------------------------------------------------------------------------------------
    let mut role = sugar_props.for_sign.map_or_else(
        || Ok(Role::default()),
        |v| {
            if let Ok(Bind::For(bind)) = v.as_bind() {
                let mut parent: ForParent = parent.as_ref().unwrap().into();
                parent.set_credential(father_role);
                if let Some(id) = id.as_ref() {
                    Ok(Role::For {
                        parent,
                        creditial: bind,
                        origin_pos: index,
                        props: bind_props.clone(),
                        children: vec![],
                        id: id.to_string(),
                        name: name.to_string(),
                    })
                } else {
                    Err(err_from_to!(
                        "GenUI Component" => "Makepad Widget, for widget need id!"
                    ))
                }
            } else {
                Ok(Role::default())
            }
        },
    )?;
    let is_role_virtual = role.is_virtual();
    // [处理inherits] --------------------------------------------------------------------------------------
    if inherits.is_some() {
        return Err(err_from_to!(
            "GenUI Component" => "Makepad Widget, Static Widget has no inherits"
        ));
    }
    // [当id存在时，说明有可能会进行脚本处理或有绑定变量] ----------------------------------------------------------
    if let Some(id) = id.as_ref() {
        let widget = AbsWidget::new(&name, props.clone());
        widget_poll.insert(id.to_string(), widget);
    }
    // [处理callbacks] --------------------------------------------------------------------------------------
    // 如果当前组件使用了as_prop，那么需要将绑定变量的值传递给父组件，并且当前组件不能调用自身的事件
    if callbacks.is_some() {
        if as_prop.is_some() {
            return Err(err_from_to!(
                "GenUI Component" => "Makepad Widget, as_prop widget can't have callback!"
            ));
        }
        // 当组件有callback时，组件必须要有id，否则抛出异常
        if id.is_none() {
            return Err(err_from_to!(
                "GenUI Component" => "Makepad Widget, callback widget need id!"
            ));
        }
    }
    // [处理节点, 属性, 子组件] ------------------------------------------------------------------------------
    let ty = if !is_define {
        WidgetType::try_from((name, props, root))?
    } else {
        WidgetType::Define((name, props, root).try_into()?)
    };

    let children = if let Some(children) = children {
        let mut w_children = vec![];
        for (index, child) in children.into_iter().enumerate() {
            let w = handle(child, template_ptrs, widget_poll, index, role.clone())?;
            match w {
                TemplateResult::Widget(widget_template) => {
                    w_children.push(widget_template);
                }
                TemplateResult::Role(child_role) => {
                    role.push_child(child_role);
                }
            }
        }
        if w_children.is_empty() {
            None
        } else {
            Some(w_children)
        }
    } else {
        None
    };

    let binds = if bind_props.is_empty() {
        None
    } else {
        Some(bind_props)
    };

    let widget = WidgetTemplate {
        id,
        is_root: root,
        as_prop,
        is_static,
        ty,
        children,
        role,
        binds,
    };
    if is_role_virtual {
        let role = widget.role.clone();
        template_ptrs.push(widget);
        Ok(TemplateResult::Role(role))
    } else {
        Ok(TemplateResult::Widget(widget))
    }
}
