use gen_utils::error::Error;
use syn::{parse_quote, ImplItemFn};

/// # 表示生命周期的访问者
/// 声明周期需要处理的的代码类似于fn-callback中的代码
/// 目前提供的生命周期钩子有：
/// 1. `#[before_mount]` -> `fn after_new_from_doc(&mut self, cx: &mut Cx)` 表示组件结构已经创建，但还未应用到文档中 （makepad提供）
/// 2. `#[mounted]` -> `fn after_apply_from_doc(&mut self, cx: &mut Cx)` 表示组件已经应用到文档中并且已经渲染 （makepad提供）
/// 3. `#[before_update]` -> `fn ${do_before_each_upadte}(&mut self, cx: &mut Cx)` 标识组件任意属性变化前触发 （由genui构建，makepad不提供）
/// 4. `#[updated]` -> `fn ${do_after_each_update}(&mut self, cx: &mut Cx)` 表示组件中任意属性发生变化后触发 （由genui构建，makepad不提供）
pub struct LifeCycleLzVisitor;

impl LifeCycleLzVisitor {
    pub fn visit<L>(item_fn: &mut ImplItemFn, life_cycle: L) -> Result<LifeCycle, Error>
    where
        L: Into<LifeCycle>,
    {
        let life_cycle: LifeCycle = life_cycle.into();
        match life_cycle {
            LifeCycle::BeforeMount => Self::before_mount(item_fn),
            LifeCycle::Mounted => Self::mounted(item_fn),
            LifeCycle::BeforeUpdate => Self::before_update(item_fn),
            LifeCycle::Updated => Self::updated(item_fn),
        }?;

        Ok(life_cycle)
    }
    /// # 处理before_mount生命周期
    fn before_mount(item_fn: &mut ImplItemFn) -> Result<(), Error> {
        // [去除#[before_mount]宏] --------------------------------------------------
        item_fn
            .attrs
            .retain(|attr| !attr.path().is_ident("before_mount"));
        Ok(())
    }
    /// # 处理mounted生命周期
    fn mounted(item_fn: &mut ImplItemFn) -> Result<(), Error> {
        // [去除#[mounted]宏] --------------------------------------------------
        item_fn
            .attrs
            .retain(|attr| !attr.path().is_ident("mounted"));

        Ok(())
    }

    /// # 处理before_update生命周期
    fn before_update(item_fn: &mut ImplItemFn) -> Result<(), Error> {
        // [去除#[before_update]宏] --------------------------------------------------
        item_fn
            .attrs
            .retain(|attr| !attr.path().is_ident("before_update"));

        // [添加cx: &mut Cx作为入参] --------------------------------------------------
        item_fn.sig.inputs.push(parse_quote! {
            cx: &mut Cx
        });

        Ok(())
    }

    fn updated(item_fn: &mut ImplItemFn) -> Result<(), Error> {
        // [去除#[updated]宏] --------------------------------------------------
        item_fn
            .attrs
            .retain(|attr| !attr.path().is_ident("updated"));

        // [添加cx: &mut Cx作为入参] --------------------------------------------------
        item_fn.sig.inputs.push(parse_quote! {
            cx: &mut Cx
        });
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LifeCycle {
    BeforeMount,
    Mounted,
    BeforeUpdate,
    Updated,
}

impl From<String> for LifeCycle {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<&str> for LifeCycle {
    fn from(value: &str) -> Self {
        match value {
            "before_mount" => LifeCycle::BeforeMount,
            "mounted" => LifeCycle::Mounted,
            "before_update" => LifeCycle::BeforeUpdate,
            "updated" => LifeCycle::Updated,
            _ => unreachable!(),
        }
    }
}
