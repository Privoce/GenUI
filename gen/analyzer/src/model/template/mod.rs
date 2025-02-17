mod prop;


use std::{collections::HashMap, str::FromStr};

// use gen_parser::{ASTNodes, BuiltinProps, PropertyKeyType, Props, PropsKey, Tag, Value};

use gen_utils::{
    common::ulid,
    error::{Error, ParseError},
};

/// ## 事件回调集合
/// 用于标识外部传入组件的事件的集合
/// 它由以下部分组成
/// - 事件名称
/// - 事件指针（这个指针只是代表这个事件在代码中赋值的变量名，例如let `btn_click` = || {}， btn_click就是这个指针）
/// - 事件
pub type Callbacks = HashMap<PropsKey, Value>;
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
    pub special: Option<String>,
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
    /// 组件的属性(由外部设置的属性)
    /// 无论是自定义组件还是内置组件，都有属性，只是有些被显示的书写在.gen文件中，有些被隐藏在组件内部
    /// 对GenUI来说，不需要关心这些属性的默认值是什么，这些都由插入的转化框架来决定
    /// 但是，GenUI需要关心这些属性是否是绑定的还是静态的
    /// 对于自定义组件来说，这些属性却是一个重要的部分，因为这些属性需要被外部传入
    pub props: Props,
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
    /// 当前组件是否为根组件
    /// 根组件指的是当前组件是整个.gen文件的组件树的根
    /// 在GenUI中，每个.gen文件都有一个根组件
    pub root: bool,
    /// 组件的子组件
    pub children: Option<Vec<Template>>,
    /// 记录父组件的标识
    pub parent: Option<Parent>,
    // /// 组件的插槽(暂不开启)
    // /// 插槽的作用在于将子组件插入到指定的位置
    // /// 在GenUI中插槽使用<slot>标签进行指定
    // /// ```gen
    // /// // parent
    // /// <my-widget>
    // ///     <slot ptr="footer">
    // ///         <input></input>
    // ///     </slot>
    // /// </my-widget>
    // ///
    // /// // child
    // /// <component name="my-widget">
    // ///     <view></view>
    // ///     <view>
    // ///         <slot name="footer"></slot>
    // ///     </view>
    // /// </component>
    // /// ```
    // slots:
}

impl Template {
    pub fn set_special(&mut self, special: &str) -> () {
        let _ = self.special.replace(special.to_string());
    }
    pub fn set_class(&mut self, class: Value) -> () {
        let _ = self.class.replace(class);
    }
    /// judge the root template tag is `<component>` or not
    pub fn is_static(&self) -> bool {
        // self.get_name().ne("component")
        let is_dyn = if let Some(props) = self.props.as_ref() {
            // check PropsKey is not normal
            props.iter().any(|(k, _v)| !k.is_normal())
        } else {
            false
        };

        self.callbacks.is_none() && !is_dyn
    }
    pub fn set_id(&mut self, id: String) -> () {
        let _ = self.id.replace(id);
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn set_name(&mut self, name: &str) -> () {
        self.name = name.to_string();
    }
    pub fn get_props(&self) -> &Props {
        &self.props
    }
    pub fn set_props(&mut self, props: Props) -> () {
        self.props = props;
    }
    pub fn has_props(&self) -> bool {
        self.props.is_some()
    }
    pub fn push_prop(&mut self, key: PropsKey, value: Value) -> () {
        match &mut self.props {
            Some(props) => {
                let _ = props.insert(key, value);
            }
            None => {
                let mut item = HashMap::new();
                item.insert(key, value);
                self.set_props(Some(item));
            }
        }
    }

    pub fn get_unbind_props(&self) -> Option<HashMap<&PropsKey, &Value>> {
        match self.props.as_ref() {
            Some(props) => Some(props.iter().filter(|(k, _)| k.is_normal()).collect()),
            None => None,
        }
    }
    pub fn get_bind_props(&self) -> Option<HashMap<&PropsKey, &Value>> {
        match self.props.as_ref() {
            Some(props) => Some(props.iter().filter(|(k, _)| k.is_bind()).collect()),
            None => None,
        }
    }
    /// get all bind props from the template model and children
    pub fn get_all_bind_props(&self) -> Option<HashMap<&PropsKey, &Value>> {
        let mut bind_props = HashMap::new();
        if let Some(items) = self.get_bind_props() {
            bind_props.extend(items);
        }

        // get all bind props from children
        if let Some(children) = self.get_children() {
            for child in children {
                if let Some(items) = child.get_all_bind_props() {
                    bind_props.extend(items);
                }
            }
        }

        if bind_props.is_empty() {
            None
        } else {
            Some(bind_props)
        }
    }
    pub fn get_callbacks(&self) -> Option<&Callbacks> {
        self.callbacks.as_ref()
    }
    pub fn set_callbacks(&mut self, callbacks: Callbacks) -> () {
        let _ = self.callbacks.replace(callbacks);
    }
    pub fn push_callbacks(&mut self, k: PropsKey, v: Value) -> () {
        match self.callbacks.as_mut() {
            Some(callbacks) => {
                let _ = callbacks.insert(k, v);
            }

            None => {
                self.callbacks = Some(
                    vec![(k, v)]
                        .into_iter()
                        .collect::<HashMap<PropsKey, Value>>(),
                )
            }
        }
    }
    pub fn has_callbacks(&self) -> bool {
        self.callbacks.is_some()
    }
    pub fn set_callbacks_from_props(&mut self) -> bool {
        let tmp_props = self.props.clone();
        match self.props.as_mut() {
            Some(props) => {
                // 所有callbacks都是Value::Function的并且也直接在PropKey上的ty是Function
                tmp_props.unwrap().iter().for_each(|(k, _)| {
                    if PropertyKeyType::Function.eq(k.ty()) {
                        match props.remove_entry(k) {
                            Some((k, v)) => match self.callbacks.as_mut() {
                                Some(callbacks) => {
                                    let _ = callbacks.insert(k, v);
                                }

                                None => {
                                    self.callbacks = Some(
                                        vec![(k, v)]
                                            .into_iter()
                                            .collect::<HashMap<PropsKey, Value>>(),
                                    )
                                }
                            },
                            None => (),
                        }
                    }
                });

                self.has_callbacks()
            }
            None => false,
        }
    }

    pub fn is_component(&self) -> bool {
        self.get_name().eq("component")
    }
    pub fn set_inherits(&mut self, inherits: &str) -> () {
        let _ = self.inherits.replace(inherits.to_string());
    }
    pub fn is_root(&self) -> bool {
        self.root
    }
    pub fn set_root(&mut self, root: bool) -> () {
        self.root = root;
    }
    pub fn get_children(&self) -> Option<&Vec<Template>> {
        self.children.as_ref()
    }
    pub fn set_children(&mut self, children: Vec<Template>) -> () {
        let _ = self.children.replace(children);
    }
    pub fn has_children(&self) -> bool {
        self.children.is_some()
    }
    pub fn push_child(&mut self, child: Template) -> () {
        match &mut self.children {
            Some(children) => children.push(child),
            None => {
                let _ = self.children.replace(vec![child]);
            }
        }
    }
    pub fn set_parent(&mut self, special: String, name: String, is_root: bool) -> () {
        let _ = self.parent.replace((special, name, is_root).into());
    }
    pub fn convert(ast: &ASTNodes, is_root: bool) -> Result<Self, Error> {
        match ast {
            ASTNodes::Tag(tag) => convert_template(&*tag, is_root),
            _ => Err(ParseError::template("template model must be a tag").into()),
        }
    }

    /// this function is used to get all props from the template model
    /// and return a tuple of two PropTree
    /// (bind_tree, fn_tree)
    pub fn get_props_tree(&self) -> (PropTree, PropTree) {
        fn append(node: &Template) -> (PropTree, PropTree) {
            let mut bind_tree = Vec::new();
            let mut fn_tree = Vec::new();
            if node.get_name().ne("component") {
                // let id = node.get_id().expect(format!("bind prop need id: {}", node.get_name()).as_str()).to_string();
                if let Some(id) = node.id.as_ref() {
                    let name = node.get_name().to_string();

                    let _ = node.get_bind_props().map(|props| {
                        if !props.is_empty() {
                            bind_tree.push((
                                (name.clone(), id.to_string()),
                                Some(
                                    props
                                        .into_iter()
                                        .map(|(k, v)| (k.clone(), v.clone()))
                                        .collect(),
                                ),
                            ));
                        }
                    });
                    match node.get_callbacks().clone() {
                        Some(callbacks) => {
                            fn_tree.push((
                                (name, id.to_string()),
                                Some(callbacks.clone().into_iter().collect()),
                            ));
                        }
                        None => (),
                    }
                }
            }

            match node.get_children() {
                Some(children) => {
                    for child in children {
                        let (binds, fns) = append(child);
                        bind_tree.extend(binds);
                        fn_tree.extend(fns);
                    }
                }
                None => (),
            }
            (bind_tree, fn_tree)
        }

        // 从根节点开始遍历
        // 获取每个节点的props以及采集节点名称
        append(self)
    }
}

/// ## 转换模板
/// 将ASTNodes::Tag转换为Template
/// - 为模型生成一个唯一标识符
/// - 获取Tag的名称作为Template的名称
/// - 获取Tag被设置的属性作为Template传入的属性
/// - 提取id, class, inherits
/// - 无需设置prop_ptr和event_ptr（这两个需要解析script才能决定）
/// - 设置root
/// - 获取所有外部传入的事件设置到callbacks上
/// - 设置children
fn convert_template(tag: &Tag, is_root: bool) -> Result<Template, Error> {
    let mut model = Template::default();
    // [生成ulid作为模型的唯一标识符]------------------------------------------------------
    let special = ulid();
    model.set_special(&special);
    model.set_root(is_root);
    // [获取Tag的名称作为Template的名称]------------------------------------------------
    model.set_name(tag.get_name());
    // [获取Tag被设置的属性作为Template传入的属性]--------------------------------------
    // 其中id、class会被单独提出来，其他的属性会被放入props中（for,if,inherits等也一样）
    if let Some(props) = tag.get_props() {
        for (k, v) in props {
            let is_normal = k.is_normal();
            if let Ok(prop) = BuiltinProps::from_str(k.name()) {
                match prop {
                    BuiltinProps::AsProp => {
                        if is_normal {
                            model.as_prop = Some(v.to_string());
                        } else {
                            return Err(
                                ParseError::template("as_prop must be a normal property").into()
                            );
                        }
                    }
                    BuiltinProps::Id => {
                        if is_normal {
                            model.set_id(v.to_string());
                        } else {
                            return Err(ParseError::template("id must be a normal property").into());
                        }
                    }
                    BuiltinProps::Class => {
                        if is_normal {
                            model.set_class(v.clone());
                        } else {
                            return Err(
                                ParseError::template("class must be a normal property").into()
                            );
                        }
                    }
                    BuiltinProps::Inherits => {
                        if is_normal {
                            model.set_inherits(v.to_string().as_str());
                        } else {
                            return Err(
                                ParseError::template("inherits must be a normal property").into()
                            );
                        }
                    }
                    BuiltinProps::For => {
                        if is_normal {
                            return Err(ParseError::template(
                                "for sugar sync must be a bind property",
                            )
                            .into());
                        } else {
                            model.sugar_props.set_for(v.clone());
                        }
                    }
                    BuiltinProps::If => {
                        if is_normal {
                            return Err(ParseError::template(
                                "if sugar sync must be a bind property",
                            )
                            .into());
                        } else {
                            model.sugar_props.set_if(IfSign::If(v.clone()));
                        }
                    }

                    BuiltinProps::ElseIf => {
                        if is_normal {
                            return Err(ParseError::template(
                                "else_if sugar sync must be a bind property",
                            )
                            .into());
                        } else {
                            model.sugar_props.set_if(IfSign::ElseIf(v.clone()));
                        }
                    }
                    BuiltinProps::Else => {
                        if is_normal {
                            return Err(ParseError::template(
                                "else sugar sync must be a bind property",
                            )
                            .into());
                        } else {
                            model.sugar_props.set_if(IfSign::Else);
                        }
                    }
                }
            } else {
                model.push_prop(k.clone(), v.clone());
            }
        }
    }
    // [设置callbacks]------------------------------------------------------------------
    model.set_callbacks_from_props();
    // [设置children]-------------------------------------------------------------------
    if tag.has_children() {
        let children = tag
            .get_children()
            .unwrap()
            .iter()
            .map(|child| {
                let mut c_model = Template::convert(child, false).unwrap();
                let id = if let Some(id) = model.id.as_ref() {
                    id.to_string()
                } else {
                    special.to_string()
                };
                c_model.set_parent(id, model.name.to_string(), is_root);
                c_model
            })
            .collect();

        model.set_children(children);
    }
    Ok(model)
}

impl Default for Template {
    fn default() -> Self {
        Self {
            special: Default::default(),
            class: Default::default(),
            id: Default::default(),
            name: Default::default(),
            props: Default::default(),
            callbacks: Default::default(),
            inherits: Default::default(),
            root: Default::default(),
            children: Default::default(),
            parent: Default::default(),
            as_prop: None,
            sugar_props: SugarProps::default(),
        }
    }
}

/// ## GenUI组件属性的语法糖
#[derive(Debug, Clone, Default)]
pub struct SugarProps {
    /// for语法糖
    pub for_sign: Option<Value>,
    /// if_else_if_else语法糖
    pub if_sign: Option<IfSign>,
}

impl SugarProps {
    pub fn set_for(&mut self, for_sign: Value) -> () {
        let _ = self.for_sign.replace(for_sign);
    }
    pub fn set_if(&mut self, if_sign: IfSign) -> () {
        let _ = self.if_sign.replace(if_sign);
    }
}

#[derive(Debug, Clone)]
pub enum IfSign {
    If(Value),
    ElseIf(Value),
    Else,
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
            root: value.2
        }
    }
}
