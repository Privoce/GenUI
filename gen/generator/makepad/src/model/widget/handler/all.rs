use crate::{
    builtin::prop::err_from_to,
    compiler::{Context, WidgetPoll as ScriptPoll},
    model::{
        role::ForParent, widget::role::Role, AbsWidget, CallbackFn, CallbackWidget, PropWidget,
        Widget, WidgetTemplate, WidgetType,
    },
    script::handle_script,
    visitor::{IdClass, StyleVisitor},
};

use gen_analyzer::{value::Bind, Script, Style, Template};
use gen_utils::{common::Source, error::Error};
use std::collections::HashMap;

pub type PropBinds = HashMap<String, Vec<PropWidget>>;
/// 模版指针存储池
pub type TemplatePtrs = Vec<WidgetTemplate>;

pub fn all(
    context: &mut Context,
    source: Source,
    template: Option<Template>,
    script: Option<Script>,
    style: Option<Style>,
    is_entry: bool,
) -> Result<Widget, Error> {
    // [初始化一些必要的池] ----------------------------------------------------------------------------------
    // 用于存储脚本中可能会进行调用的Widget
    let mut sc_poll: ScriptPoll = HashMap::new();
    // 用于存储需要双向绑定的prop
    let mut prop_poll: PropBinds = HashMap::new();
    let mut callback_poll: Vec<CallbackWidget> = vec![];
    let mut template_ptrs: TemplatePtrs = vec![];
    // [处理template] --------------------------------------------------------------------------------------
    let template = if let Some(template) = template {
        if let TemplateResult::Widget(template) = handle_template(
            template,
            style.as_ref(),
            &mut template_ptrs,
            &mut sc_poll,
            &mut prop_poll,
            &mut callback_poll,
            &mut vec![],
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
    let script = handle_script(
        script,
        context,
        template.as_ref(),
        prop_poll,
        callback_poll,
        &template_ptrs,
        sc_poll,
    )?;
    // [处理动态生成语法糖需要的代码] ----------------------------------------------------------------------
    let template_ptrs = if template_ptrs.is_empty() {
        None
    } else {
        Some(template_ptrs)
    };

    let mut widget = Widget {
        source,
        template,
        template_ptrs,
        script,
        is_entry,
        has_plugin: context.plugins.is_some(),
    };

    // 执行前需要执行default_script
    let _ = widget.default_script();
    Ok(widget)
}

fn handle_template(
    template: Template,
    styles: Option<&Style>,
    template_ptrs: &mut TemplatePtrs,
    sc_poll: &mut ScriptPoll,
    prop_poll: &mut PropBinds,
    callback_poll: &mut Vec<CallbackWidget>,
    chain: &mut Vec<IdClass>,
    index: usize,
    father_role: Role,
) -> Result<TemplateResult, Error> {
    let is_static = template.is_static();
    let is_define = template.is_component();
    let Template {
        id,
        class,
        as_prop,
        name,
        mut props,
        callbacks,
        inherits,
        children,
        sugar_props,
        parent,
        ..
    } = template;
    // 是否是根节点，只有根节点没有父节点
    let root = parent.is_none();
    // [绑定变量处理] ----------------------------------------------------------------------------------------
    let mut binds = HashMap::new();
    if let Some(bind_props) = props.as_ref() {
        for (k, v) in bind_props {
            if k.is_bind() {
                let v = v.as_bind()?;
                match &v {
                    Bind::Normal(_normal) => {
                        binds.insert( v.to_string(), k.name.to_string());
                    },
                    Bind::For(_) => panic!("for has been remove from bind props, if you see this error, please connect the author"),
                }
            }
        }
    }

    // [处理语法糖] -----------------------------------------------------------------------------------------
    // [for] ------------------------------------------------------------------------------------------
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
                        props: binds.clone(),
                        children: vec![],
                        id: id.to_string(),
                        name: name.to_string(),
                    })
                } else {
                    Err(Error::from(err_from_to(
                        "GenUI Component",
                        "Makepad Widget, for widget need id!",
                    )))
                }
            } else {
                Ok(Role::default())
            }
        },
    )?;
    let is_role_virtual = role.is_virtual();
    // [处理inherits] --------------------------------------------------------------------------------------
    if inherits.is_some() {
        return Err(err_from_to(
            "GenUI Component",
            "Makepad Widget, Static Widget has no inherits",
        )
        .into());
    }
    // [当id存在时，说明有可能会进行脚本处理或有绑定变量] ----------------------------------------------------------
    if let Some(id) = id.as_ref() {
        let widget = AbsWidget::new(&name, props.clone());
        // sc_poll 进行insert
        sc_poll.insert(id.to_string(), widget);
    }
    // [当使用了as_prop时，说明需要将当前组件作为属性传递给父组件] --------------------------------------------------
    // 如果当前组件使用了绑定变量，那么需要将绑定变量的值传递给父组件，并且当前组件不能调用自身的事件
    binds.iter().for_each(|(bind, prop)| {
        let prop = as_prop.as_ref().map_or_else(
            || {
                id.as_ref()
                    .map(|id| PropWidget::new(id.to_string(), name.to_string(), prop.to_string()))
            },
            |as_prop| {
                let parent = parent.as_ref().unwrap();
                let mut prop_widget = PropWidget::new(
                    parent.id.to_string(),
                    parent.name.to_string(),
                    prop.to_string(),
                );
                prop_widget.as_prop = Some((name.to_string(), as_prop.to_string()));
                Some(prop_widget)
            },
        );

        if let Some(prop) = prop {
            prop_poll
                .entry(bind.clone())
                .or_insert_with(Vec::new)
                .push(prop);
        }
    });

    // [处理callbacks] --------------------------------------------------------------------------------------
    // 当需要处理callbacks时，我们需要使用LazyVisitor来进行处理, 这里只需要通过相关信息生成Visitor即可
    if let Some(callbacks) = callbacks {
        if as_prop.is_some() {
            return Err(err_from_to(
                "GenUI Component",
                "Makepad Widget, as_prop widget can't have callback!",
            )
            .into());
        }
        // 当组件有callback时，组件必须要有id，否则抛出异常
        let id = id.as_ref().map_or_else(
            || {
                Err(Error::from(err_from_to(
                    "GenUI Component",
                    "Makepad Widget, callback widget need id!",
                )))
            },
            |id| Ok(id.to_string()),
        )?;
        let mut fn_callbacks: HashMap<String, CallbackFn> = HashMap::new();
        for (key, call_fn) in callbacks {
            let callback = key.name.to_string();
            let func = call_fn.as_fn()?;
            fn_callbacks.insert(func.name.to_string(), CallbackFn::new(func, callback));
        }
        callback_poll.push(CallbackWidget {
            id,
            name: name.to_string(),
            callbacks: fn_callbacks,
        });
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
        for (index, child) in children.into_iter().enumerate() {
            let w = handle_template(
                child,
                styles,
                template_ptrs,
                sc_poll,
                prop_poll,
                callback_poll,
                chain,
                index,
                role.clone(),
            )?;
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

    let binds = if binds.is_empty() { None } else { Some(binds) };

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

pub enum TemplateResult {
    Widget(WidgetTemplate),
    Role(Role),
}
