mod comment;
mod prop;
pub use comment::*;
pub use prop::*;
use std::{
    borrow::Cow,
    collections::HashMap,
    str::FromStr,
    sync::{Arc, RwLock},
};

use gen_utils::{
    common::Ulid,
    err_from, err_from_to,
    error::{Error, ParseError},
};

use crate::{template, value::Value, PropComponent};

use super::{EventComponent, Polls};

/// ## 事件回调集合
/// 用于标识外部传入组件的事件的集合
/// 它由以下部分组成
/// - 事件名称
/// - 事件指针（这个指针只是代表这个事件在代码中赋值的变量名，例如let `btn_click` = || {}， btn_click就是这个指针）
/// - 事件
pub type Callbacks = HashMap<PropKey, Value>;
/// 记录组件中是否有绑定的属性和事件
/// 这个类型会记录下组件树中所有的绑定属性和事件, 使用`Template.get_props_tree()`获取
/// 返回结果为双元素元组，第一个元素是绑定属性，第二个元素是绑定事件
/// `Vec<(($widget_name, $widget_id), Some({key:PropsKey, value: Value}))>`
pub type PropTree = Vec<((String, String), Props)>;

/// # GenUI组件模型
/// 它用于完整的表示一个.gen文件，因为.gen文件就是一个完整的组件，所以这个模型也是一个完整的组件
/// 组件严格意义上并没有区分
/// 在GenUI中甚至没有内置组件的概念（因为GenUI是可插拔的，如果你想要转化为Makepad，那么内置组件就是Makepad的内置组件）
#[derive(Debug, Clone)]
pub struct Template {
    /// 组件的唯一标识符
    /// 它可以与文件模型的唯一标识符组合
    /// 以此来在不同的文件中区分相同的组件
    pub special: Ulid,
    /// class是一个数组，一个组件模型可以有多个class
    /// 这些class指向style中的样式
    /// 这些class可以是动态绑定的
    pub class: Option<Value>,
    /// id是一个字符串，一个组件模型只能有一个id
    /// 这个id不能是动态绑定的，只能是静态的
    pub id: Option<String>,
    /// 将组件作为一个普通组件还是属性插槽
    /// 只要使用了as_prop，那么这个组件就会被当作属性插槽, 当前写法如下
    /// 例如：`<view id="hello" as_prop="slot" />`
    /// 表明这个组件是一个属性插槽，插槽的名字是slot
    pub as_prop: Option<String>,
    /// 组件的名字，这个名字标识了组件应该如何在.gen文件中书写
    /// 例如，如果组件名字是`button`，那么在.gen文件中书写`<button></button>`就是正确的
    pub name: String,
    /// 组件的静态属性(可由外部设置的属性)
    /// 无论是自定义组件还是内置组件，都有属性，只是有些被显示的书写在.gen文件中，有些被隐藏在组件内部
    /// 对GenUI来说，不需要关心这些属性的默认值是什么，这些都由插入的转化框架来决定
    /// 但是，GenUI需要关心这些属性是否是绑定的还是静态的
    /// 对于自定义组件来说，这些属性却是一个重要的部分，因为这些属性需要被外部传入
    pub props: Option<Props>,
    /// 组件的动态属性通过绑定的方式传入
    pub binds: Option<Props>,
    /// 由GenUI提供的组件的属性的语法糖
    /// 例如: `[for, if, else_if, else]`
    /// 同样也会从props中提取这些属性
    pub sugar_props: SugarProps,
    /// 组件的事件的回调(是指组件内部允许暴露到外部的事件)
    /// 指的是外部组件当组件内部的事件被触发后，进行处理
    /// 回调的参数依赖于组件的事件提供给外部参数
    /// 回调表现为一个闭包或一个函数
    /// 语法：`<define_component @click="do_click" />`
    pub callbacks: Option<Callbacks>,
    /// 组件是否继承另一个组件
    /// 若继承另一个组件，当前组件就会自动继承另一个组件的所有属性和事件
    /// 注意这个属性只能是normal的不能是动态绑定的
    pub inherits: Option<String>,
    // 当前组件是否为根组件 (#[deprecated]) 可以直接确认无需传入
    // 根组件指的是当前组件是整个.gen文件的组件树的根
    // 在GenUI中，每个.gen文件都有一个根组件
    // pub root: bool,
    /// 组件的子组件
    pub children: Option<Vec<Template>>,
    /// 记录父组件的标识
    pub parent: Option<Parent>,
    /// 是否为根组件
    pub root: bool,
    /// 注释
    pub comments: Option<Vec<Comment>>,
}

impl Template {
    pub fn new(name: &str) -> Self {
        let mut template = Self::default();
        template.name = name.to_string();
        template
    }

    /// 解析模版部分并返回模版后续进行静态分析的池
    pub fn parse(input: &str, poll: Arc<RwLock<Polls>>) -> Result<Self, Error> {
        template::parse(input, poll, true)
    }

    /// ## after parse
    /// 在 template::parse(input) 内部调用在，在所有属性被分析完成后调用这个方法
    /// See: [`self.push_prop()`]
    pub fn after_prop_parse(
        &mut self,
        poll: Arc<RwLock<Polls>>,
        iter: Option<&SugarIter>,
    ) -> Result<Option<SugarIter>, Error> {
        let mut back_iter = None;
        // [获取Tag被设置的属性作为Template传入的属性]------------------------------------------
        // 其中id、class会被单独提出来，其他的属性会被放入props中（for,if,inherits等也一样）
        // 在进行属性处理的时候同时获取出池化属性
        if let Some(props) = self.props.take() {
            for (k, v) in props {
                let sugar_iter = self.push_prop(k, v, iter)?;
                if sugar_iter.is_some() {
                    back_iter = sugar_iter;
                }
            }
            // [set events] ----------------------------------------------------------------
            // 由于事件的存储是直接存储所有事件在EvenComponent中所以在外面一次性处理
            if let Some(callbacks) = self.callbacks.as_ref() {
                // get write lock
                let mut poll = poll.write().map_err(|e| err_from!(e.to_string()))?;
                poll.insert_event(self.as_event_component(callbacks)?);
            }
        }

        // 检查组件是否有id, 如果没有id则将special作为id
        if self.id.is_none() {
            self.id.replace(self.special.to_snake());
        }

        Ok(back_iter)
    }
    pub fn after_all(&mut self, poll: Arc<RwLock<Polls>>) -> Result<(), Error> {
        let mut poll = poll.write().map_err(|e| err_from!(e.to_string()))?;
        if let Some(binds) = self.binds.as_ref() {
            let (name, id) = self.get_name_and_id()?;
            // 延迟处理binds
            for (key, value) in binds {
                poll.insert_prop(
                    &value.as_bind()?.ident(),
                    PropComponent {
                        id: id.clone().into_owned(),
                        name: name.clone().into_owned(),
                        prop: key.name.to_string(),
                        as_prop: self.as_prop.clone(),
                        father_ref: self.parent.clone(),
                    },
                );
            }
        }

        if let Some(props) = self.sugar_props.as_props()? {
            let (name, id) = self.get_name_and_id()?;
            for (ident, prop) in props {
                poll.insert_prop(
                    &ident,
                    PropComponent {
                        id: id.clone().into_owned(),
                        name: name.clone().into_owned(),
                        prop,
                        as_prop: self.as_prop.clone(),
                        father_ref: self.parent.clone(),
                    },
                );
            }
        }

        Ok(())
    }

    /// judge the root template tag is `<component>` or not
    pub fn is_static(&self) -> bool {
        self.callbacks.is_none() && self.binds.is_none()
    }
    fn insert_prop(props: &mut Option<Props>, key: PropKey, value: Value) -> () {
        match props {
            Some(props) => {
                let _ = props.insert(key, value);
            }
            None => {
                let mut item = HashMap::new();
                item.insert(key, value);
                props.replace(item);
            }
        }
    }
    fn insert_event(callbacks: &mut Option<Callbacks>, key: PropKey, value: Value) -> () {
        match callbacks {
            Some(callbacks) => {
                let _ = callbacks.insert(key, value);
            }
            None => {
                let mut item = HashMap::new();
                item.insert(key, value);
                callbacks.replace(item);
            }
        }
    }
    pub fn push_prop(
        &mut self,
        key: PropKey,
        value: Value,
        iter: Option<&SugarIter>,
    ) -> Result<Option<SugarIter>, Error> {
        // [if is special props]---------------------------------------------------------------------------
        let mut back_iter = None;
        if let Ok(prop) = BuiltinProps::from_str(&key.name) {
            match prop {
                BuiltinProps::AsProp => {
                    if key.is_normal() {
                        self.as_prop = Some(value.to_string());
                    } else {
                        return Err(
                            ParseError::template("as_prop must be a normal property").into()
                        );
                    }
                }
                BuiltinProps::Id => {
                    if key.is_normal() {
                        self.id.replace(value.to_string());
                    } else {
                        return Err(ParseError::template("id must be a normal property").into());
                    }
                }
                BuiltinProps::Class => {
                    if key.is_normal() {
                        self.class.replace(value.clone());
                    } else {
                        return Err(ParseError::template("class must be a normal property").into());
                    }
                }
                BuiltinProps::Inherits => {
                    if key.is_normal() {
                        self.inherits.replace(value.to_string());
                    } else {
                        return Err(
                            ParseError::template("inherits must be a normal property").into()
                        );
                    }
                }
                BuiltinProps::For => {
                    if key.is_bind() {
                        self.sugar_props = SugarProps::For(value.clone());
                    } else {
                        return Err(
                            ParseError::template("for sugar sync must be a bind property").into(),
                        );
                    }
                }
                BuiltinProps::If => {
                    if key.is_bind() {
                        self.sugar_props = SugarProps::If(SugarIf::If(If {
                            expr: value.clone(),
                        }));
                        back_iter.replace(SugarIter::If { expr: value });
                    } else {
                        return Err(
                            ParseError::template("if sugar sync must be a bind property").into(),
                        );
                    }
                }

                BuiltinProps::ElseIf => {
                    if key.is_bind() {
                        if let Some(iter) = iter {
                            let else_if = match iter {
                                SugarIter::If { expr } => SugarIf::ElseIf(ElseIf {
                                    expr: value.clone(),
                                    if_expr: If { expr: expr.clone() },
                                    else_if_exprs: vec![],
                                }),
                                SugarIter::ElseIf {
                                    expr,
                                    else_if_exprs,
                                    if_expr,
                                } => {
                                    let mut else_if_exprs = else_if_exprs.clone();
                                    else_if_exprs.push(expr.clone());
                                    SugarIf::ElseIf(ElseIf {
                                        expr: value.clone(),
                                        if_expr: If {
                                            expr: if_expr.clone(),
                                        },
                                        else_if_exprs,
                                    })
                                }
                            };
                            self.sugar_props = SugarProps::If(else_if.clone());
                            back_iter.replace(self.sugar_props.clone().try_into()?);
                        } else {
                            return Err(ParseError::template(
                                "else_if sugar sync must be after `if` or `else_if`",
                            )
                            .into());
                        }
                    } else {
                        return Err(ParseError::template(
                            "else_if sugar sync must be a bind property",
                        )
                        .into());
                    }
                }
                BuiltinProps::Else => {
                    if key.is_bind() {
                        // dbg!(iter);
                        if let Some(iter) = iter {
                            match iter {
                                SugarIter::If { expr } => {
                                    self.sugar_props = SugarProps::If(SugarIf::Else(Else {
                                        if_expr: If { expr: expr.clone() },
                                        else_if_exprs: vec![],
                                    }));
                                }
                                SugarIter::ElseIf {
                                    expr,
                                    else_if_exprs,
                                    if_expr,
                                } => {
                                    let mut else_if_exprs = else_if_exprs.clone();
                                    else_if_exprs.push(expr.clone());
                                    self.sugar_props = SugarProps::If(SugarIf::Else(Else {
                                        if_expr: If {
                                            expr: if_expr.clone(),
                                        },
                                        else_if_exprs,
                                    }));
                                }
                            }
                        } else {
                            return Err(ParseError::template(
                                "else sugar sync must be after `if` or `else_if`",
                            )
                            .into());
                        }
                    } else {
                        return Err(ParseError::template(
                            "else sugar sync must be a bind property",
                        )
                        .into());
                    }
                }
            }
        } else {
            // [other props]---------------------------------------------------------------------------
            match key.ty {
                PropKeyType::Normal => {
                    Self::insert_prop(&mut self.props, key, value);
                }
                PropKeyType::Bind => {
                    Self::insert_prop(&mut self.binds, key, value);
                }
                PropKeyType::Function => {
                    Self::insert_event(&mut self.callbacks, key, value);
                }
            }
        }

        Ok(back_iter)
    }

    pub fn is_component(&self) -> bool {
        self.name.eq("component")
    }

    pub fn set_parent(&mut self, id: String, name: String, root: bool) -> () {
        let _ = self.parent.replace((id, name, root).into());
    }
    pub fn as_parent(&self) -> (String, String) {
        let id = self
            .id
            .as_ref()
            .unwrap_or(&self.special.to_snake())
            .to_string();

        (id, self.name.to_string())
    }

    fn get_name_and_id(&self) -> Result<(Cow<'_, String>, Cow<'_, String>), Error> {
        // [name] -------------------------------------------------------------------------------------------------------
        let name = Cow::Borrowed(&self.name);
        // [id (needed or err)] -----------------------------------------------------------------------------------------
        let id = if let Some(as_prop) = self.as_prop.as_ref() {
            Cow::Borrowed(as_prop)
        } else {
            self.id
            .as_ref()
            .map_or_else(
            || Err(err_from_to!("Template" => format!("PropComponent, can not find id in template please check: {}", &name))),
            |id| Ok(Cow::Borrowed(id))
            )?
        };
        Ok((name, id))
    }

    /// prop: bind prop name (:color="label_color" => color)
    pub fn as_prop_component(&self, prop: &str) -> Result<PropComponent, Error> {
        let (name, id) = self.get_name_and_id()?;

        Ok(PropComponent {
            id: id.into_owned(),
            name: name.into_owned(),
            prop: prop.to_string(),
            as_prop: self.as_prop.clone(),
            father_ref: self.parent.clone(),
        })
    }

    pub fn as_event_component(
        &self,
        callbacks: &HashMap<PropKey, Value>,
    ) -> Result<EventComponent, Error> {
        let (name, id) = self.get_name_and_id()?;

        Ok(EventComponent {
            id: id.into_owned(),
            name: name.into_owned(),
            callbacks: EventComponent::convert_callbacks(callbacks)?,
        })
    }
}

impl Default for Template {
    fn default() -> Self {
        Self {
            special: Ulid::new(),
            class: Default::default(),
            id: Default::default(),
            name: Default::default(),
            props: Default::default(),
            binds: Default::default(),
            callbacks: Default::default(),
            inherits: Default::default(),
            children: Default::default(),
            parent: Default::default(),
            as_prop: None,
            sugar_props: SugarProps::default(),
            comments: None,
            root: false,
        }
    }
}

/// ## GenUI组件属性的语法糖
#[derive(Debug, Clone, Default)]
pub enum SugarProps {
    /// for语法糖
    For(Value),
    /// if_else_if_else语法糖
    If(SugarIf),
    /// 没有语法糖
    #[default]
    None,
}

impl SugarProps {
    pub fn as_props(&self) -> Result<Option<Vec<(String, String)>>, Error> {
        match self {
            SugarProps::For(for_sign) => Ok(Some(vec![(
                for_sign.as_bind()?.ident(),
                crate::value::For::SUGAR_SIGN.to_string(),
            )])),
            SugarProps::If(sugar_if) => match sugar_if {
                SugarIf::If(sugar_if) => Ok(Some(vec![(
                    sugar_if.expr.as_bind()?.ident(),
                    If::SUGAR_SIGN.to_string(),
                )])),
                SugarIf::ElseIf(sugar_else_if) => {
                    let mut res = vec![
                        (
                            sugar_else_if.if_expr.expr.as_bind()?.ident(),
                            ElseIf::SUGAR_SIGN.to_string(),
                        ),
                        (
                            sugar_else_if.expr.as_bind()?.ident(),
                            ElseIf::SUGAR_SIGN.to_string(),
                        ),
                    ];

                    for expr in sugar_else_if.else_if_exprs.iter() {
                        res.push((expr.as_bind()?.ident(), ElseIf::SUGAR_SIGN.to_string()));
                    }

                    Ok(Some(res))
                }
                SugarIf::Else(sugar_else) => {
                    let mut res = vec![(
                        sugar_else.if_expr.expr.as_bind()?.ident(),
                        Else::SUGAR_SIGN.to_string(),
                    )];

                    for expr in sugar_else.else_if_exprs.iter() {
                        res.push((expr.as_bind()?.ident(), Else::SUGAR_SIGN.to_string()));
                    }

                    Ok(Some(res))
                }
            },
            SugarProps::None => Ok(None),
        }
    }
}

impl TryFrom<SugarProps> for SugarIter {
    type Error = Error;

    fn try_from(value: SugarProps) -> Result<Self, Self::Error> {
        if let SugarProps::If(if_sugar) = value {
            match if_sugar {
                SugarIf::If(if_sugar) => {
                    return Ok(SugarIter::If {
                        expr: if_sugar.expr,
                    });
                }
                SugarIf::ElseIf(else_if) => {
                    return Ok(SugarIter::ElseIf {
                        expr: else_if.expr,
                        else_if_exprs: else_if.else_if_exprs,
                        if_expr: else_if.if_expr.expr,
                    });
                }
                SugarIf::Else(_) => {
                    return Err(
                        err_from_to!("SugarProps" => "SugarIter, only `If`, `ElseIf` can be convert to SugarIter"),
                    );
                }
            }
        } else {
            return Err(
                err_from_to!("SugarProps" => "SugarIter, only `If`, `ElseIf` can be convert to SugarIter"),
            );
        }
    }
}

impl From<SugarIf> for SugarIter {
    fn from(value: SugarIf) -> Self {
        match value {
            SugarIf::If(if_sugar) => SugarIter::If {
                expr: if_sugar.expr,
            },
            SugarIf::ElseIf(else_if) => SugarIter::ElseIf {
                expr: else_if.expr,
                else_if_exprs: else_if.else_if_exprs,
                if_expr: else_if.if_expr.expr,
            },
            SugarIf::Else(_) => unreachable!("SugarIf::Else can not convert to SugarIter"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SugarIter {
    If {
        expr: Value,
        // component: Template,
    },
    ElseIf {
        expr: Value,
        else_if_exprs: Vec<Value>,
        if_expr: Value,
    },
}

impl SugarIter {
    pub fn is_if(&self) -> bool {
        matches!(self, SugarIter::If { .. })
    }
    pub fn is_else_if(&self) -> bool {
        matches!(self, SugarIter::ElseIf { .. })
    }
}

#[derive(Debug, Clone)]
pub enum SugarIf {
    If(If),
    ElseIf(ElseIf),
    Else(Else),
}

impl SugarIf {
    pub const SUGAR_SIGNS: [&'static str; 3] = [If::SUGAR_SIGN, ElseIf::SUGAR_SIGN, Else::SUGAR_SIGN];
}

#[derive(Debug, Clone)]
pub struct If {
    /// if语句的条件
    pub expr: Value,
}

impl If {
    pub const SUGAR_SIGN: &'static str = "if_sugar_sign";
}

#[derive(Debug, Clone)]
pub struct ElseIf {
    /// else if语句的条件
    pub expr: Value,
    /// 其他的else if语句，因为else if语句可以有多个
    pub else_if_exprs: Vec<Value>,
    /// if语句的条件
    pub if_expr: If,
    // /// if 语句组件
    // pub if_component: Template
}

impl ElseIf {
    pub const SUGAR_SIGN: &'static str = "else_if_sugar_sign";
}

#[derive(Debug, Clone)]
pub struct Else {
    pub if_expr: If,
    pub else_if_exprs: Vec<Value>,
    // pub if_component: Template,
    // pub else_components: Vec<Template>
}

impl Else {
    pub const SUGAR_SIGN: &'static str = "else_sugar_sign";
}
#[derive(Debug, Clone)]
pub struct Parent {
    pub id: String,
    pub name: String,
    pub root: bool,
}

impl From<(String, String, bool)> for Parent {
    fn from(value: (String, String, bool)) -> Self {
        Self {
            id: value.0,
            name: value.1,
            root: value.2,
        }
    }
}
