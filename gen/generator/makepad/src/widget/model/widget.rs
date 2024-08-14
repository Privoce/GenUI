use std::{collections::HashMap, hash::Hash, iter::once};

use gen_converter::model::{
    prop::ConvertStyle,
    script::{CurrentInstance, GenScriptModel, PropFn, ScriptModel, UseMod},
    TemplateModel,
};
use gen_parser::{Bind, For, PropsKey, Value};

use gen_utils::common::{
    ident, snake_to_camel,
    syn_ext::{let_to_self, TypeGetter},
    IFSignal, Source, Ulid,
};

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_str, Ident, ItemEnum, ItemStruct, Stmt, StmtMacro};

use crate::{
    compiler::AUTO_BUILTIN_WIDGETS,
    utils::{component_render, special_struct},
    widget::{
        utils::{combine_option, quote_draw_widget},
        BuiltIn,
    },
};

use super::{
    handler::WidgetHandler,
    live_hook::LiveHookTrait,
    role::{Role, RoleType},
    safe_widget::SafeWidget,
    traits::WidgetTrait,
    ToLiveDesign,
};

/// ## 当生成 live_design! 中的节点时
/// `[id] [:|=] <name>{ [...props|widget...] }`
/// ## 当生成一个完整的组件时
#[derive(Debug, Default, Clone)]
pub struct Widget {
    /// Makepad live_design! macro
    // pub live_design: Option<LiveDesign>,
    pub is_root: bool,
    /// is a built-in widget
    pub is_built_in: bool,
    /// is a define widget
    pub is_static: bool,
    /// widget id, if widget is prop, id is prop
    pub id: Option<String>,
    /// is widget as a prop? if prop is true , widget need id
    /// `<view id="a" as_prop></view>` => as_prop = true
    pub as_prop: bool,
    pub name: String,
    pub source: Option<Source>,
    pub imports: Option<TokenStream>,
    pub uses: Option<TokenStream>,
    // pub compiled_source: Option<PathBuf>,
    /// props in live_design
    pub props: Option<TokenStream>,
    /// events called in makepad
    pub events: Option<HashMap<String, TokenStream>>,
    pub prop_ptr: Option<TokenStream>,
    pub event_ptr: Option<TokenStream>,
    pub event_ref: Option<TokenStream>,
    pub event_set: Option<TokenStream>,
    pub children: Option<Vec<Widget>>,
    pub inherits: Option<BuiltIn>,
    pub traits: Option<WidgetTrait>,
    pub live_hook: Option<LiveHookTrait>,
    pub role: Role,
}

impl PartialEq for Widget {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
    }
}

impl Eq for Widget {}

impl Hash for Widget {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.source.hash(state);
    }
}

impl Widget {
    pub fn default_ui_root() -> Self {
        let mut widget = Widget::default();
        widget.is_root = true;
        widget.id.replace("ui".to_string());
        widget.name = "Root".to_string();
        widget.is_static = true;

        widget
    }
    pub fn new(
        special: Option<&Source>,
        name: &str,
        inherits: Option<&String>,
        id: Option<&String>,
        is_root: bool,
        script: Option<&ScriptModel>,
    ) -> Self {
        let mut widget = Widget::default();
        // current widget source must be existed
        let source = special.expect("widget source must be existed");
        let is_component = name == "component";
        let (name, inherits, is_builtin, is_static) = WidgetHandler::build_widget_struct_name(
            &source.source_name(),
            name,
            id,
            is_component,
            inherits,
            is_root,
            script,
        );
        // handle widget ------------------------------------------------------------------------------------------
        widget.name = name;
        let _ = inherits.map(|x| widget.inherits.replace(x));
        let _ = id.map(|x| widget.id.replace(x.to_string()));
        widget.is_built_in = is_builtin;
        widget.is_static = is_static; // is static may be change after
        widget.is_root = is_root;
        widget.source.replace(source.clone());

        // widget.set_traits(WidgetTrait::default());
        // widget.live_hook.replace(LiveHookTrait::default());
        widget
    }
    pub fn new_builtin(name: &str) -> Self {
        let mut widget = Widget::default();
        widget.name = name.to_string();
        widget.is_built_in = BuiltIn::try_from(name).is_ok();
        widget
    }
    pub fn get_live_hook_mut(&mut self) -> &mut LiveHookTrait {
        if self.live_hook.is_none() {
            self.live_hook.replace(LiveHookTrait::default());
        }
        self.live_hook.as_mut().unwrap()
    }
    pub fn get_traits_mut(&mut self) -> &mut WidgetTrait {
        if self.traits.is_none() {
            self.traits.replace(WidgetTrait::default());
        }
        self.traits.as_mut().unwrap()
    }
    pub fn set_as_prop(&mut self, as_prop: bool) -> &mut Self {
        self.as_prop = as_prop;
        self
    }
    pub fn set_is_static(&mut self, is_static: bool) -> &mut Self {
        self.is_static = is_static;
        self
    }
    /// if can not parse by BuiltIn Widget -> panic!
    pub fn set_props(&mut self, props: Option<HashMap<PropsKey, Value>>) -> &mut Self {
        if let Some(props) = props {
            // if self.is_built_in {
            //     self.props = Some(BuiltIn::from(&self.name).props(&props));
            // } else {
            //     todo!("widget props define unsoloved => {:#?}",props);
            // }
            if self.is_built_in {
                self.props = Some(BuiltIn::from(&self.name).props(&props));
            }
        }
        self
    }
    /// ## set widget script
    /// - set prop_ptr
    /// - set event_ptr
    /// - set uses
    /// - set draw_walk
    /// - set handle_event
    pub fn set_script(
        &mut self,
        script: Option<&ScriptModel>,
        replacer: Option<&Replacer>,
    ) -> &mut Self {
        if self.is_root || self.role.is_special() {
            if let Some(sc) = script {
                if let ScriptModel::Gen(sc) = sc {
                    let GenScriptModel {
                        uses,
                        prop_ptr,
                        event_ptr,
                        sub_prop_binds,
                        sub_event_binds,
                        // other,
                        imports,
                        current_instance,
                        instance_opt,
                        other,
                        ..
                    } = sc;
                    // 在这里从prop_ptr中获取结构体所有的field作为后续代码中需要转换的列表
                    // 例如在handle_event中就需要
                    let prop_fields = get_props_fields(prop_ptr.as_ref());
                    // , bind_props.filter_fields(other.as_ref())

                    self.set_uses(uses)
                        .set_imports(imports)
                        .set_prop_ptr(prop_ptr)
                        .set_event_ptr(event_ptr)
                        .before_apply(other.as_ref(), prop_fields.as_ref())
                        .after_apply(
                            sub_prop_binds,
                            current_instance.as_ref(),
                            instance_opt.as_ref(),
                            replacer,
                        )
                        .draw_walk(None) // 暂时先写个None
                        .handle_event(
                            sub_prop_binds,
                            sub_event_binds,
                            current_instance.as_ref(),
                            prop_fields.as_ref(),
                        );
                }
            } else {
                self.is_static = true;
            }
        }

        self
    }
    pub fn before_apply(
        &mut self,
        code: Option<&Vec<Stmt>>,
        fields: Option<&Vec<Ident>>,
    ) -> &mut Self {
        if code.is_none() {
            return self;
        }
        // get check_list --------------------------------------------------------------------------------------
        let check_list = if self.is_root {
            // if is root widget, it should check if it is_static
            if self.is_static {
                None
            } else {
                // means current root widget is define widget, get widget define struct
                fields.map(|x| x.iter().map(|field| field.to_string()).collect())
            }
        } else {
            // if is not, check self.role is special or not, if is special, get for_ident or if_ident
            match &self.role {
                Role::Normal => None,
                Role::If { .. } => None, // temlpate None todo!!!
                Role::For { credential, .. } => {
                    Some(once(credential.iter_ident.to_string()).collect())
                }
            }
        };

        // handle before_apply ----------------------------------------------------------------------------------
        if let Some(before_apply) = let_to_self(code.unwrap(), check_list) {
            self.get_live_hook_mut().before_apply(before_apply);
        }

        self
    }
    /// - prop_binds: 模板中绑定的props，用于对模板中的props进行更新，它能够跟踪到底prop应该如何更新
    /// - events: 模板中绑定的events
    /// - current_instance: 当前实例(属性实例)，用于获取实例名，它需要和prop_fields一起使用，来找到原Gen代码中需要被替换的部分(`current_instance.prop_field`)
    /// - prop_fields: 用于获取prop_ptr中的所有字段，用于在handle_event中找到需要更新的属性部分然后替换
    pub fn handle_event(
        &mut self,
        prop_binds: &Option<Vec<PropFn>>,
        events: &Option<Vec<PropFn>>,
        current_instance: Option<&CurrentInstance>,
        prop_fields: Option<&Vec<Ident>>,
    ) -> &mut Self {
        if !self.is_root {
            return self;
        }

        let instance_name = if let Some(instance) = current_instance {
            instance.name()
        } else {
            None
        };
        let builtin = self.inherits.as_ref().unwrap();
        let handle_event_tk = builtin.handle_event(events, prop_binds, instance_name, prop_fields);
        let _ = self.traits.as_mut().unwrap().handle_event(handle_event_tk);
        self
    }
    pub fn after_apply(
        &mut self,
        prop_binds: &Option<Vec<PropFn>>,
        current_instance: Option<&CurrentInstance>,
        instance_opt: Option<&Vec<Stmt>>,
        replacer: Option<&Replacer>,
    ) -> &mut Self {
        if !self.is_root {
            return self;
        }
        // 将当前实例所涉及的代码转为TokenStream
        // 需要将特定的头部转为self
        let apply_tk = instance_opt.map(|opt| {
            let instance_name = current_instance
                .unwrap()
                .name()
                .expect("current instance must have name")
                .to_string();
            opt.into_iter().fold(TokenStream::new(), |mut acc, item| {
                // 这里我本来可以一点点替换的，但发现似乎这样会错过很多情况，所以转而使用转为String后进行replace
                let item = item.to_token_stream().to_string();

                // let item = item.replacen(&instance_name, "self", 1);
                let item = item.replace(&instance_name, "self");

                acc.extend(parse_str::<TokenStream>(&item));
                acc
            })
        });

        let draw_widget_tk = quote_draw_widget(prop_binds, replacer);

        let apply_tk = combine_option(apply_tk, draw_widget_tk);

        let _ = self.get_live_hook_mut().after_apply(apply_tk);

        self
    }
    pub fn draw_walk(&mut self, draw_walk_tk: Option<TokenStream>) -> &mut Self {
        // 由BuiltIn确定如何draw_walk
        if self.is_root {
            let draw_walk_tk = self.inherits.as_ref().unwrap().draw_walk(&draw_walk_tk);
            let _ = self.get_traits_mut().draw_walk(draw_walk_tk);
        }
        self
    }
    pub fn set_uses(&mut self, uses: &Option<UseMod>) -> &mut Self {
        if let Some(uses) = uses {
            self.uses = WidgetHandler::uses(uses);
        }
        self
    }
    pub fn set_imports(&mut self, imports: &Option<StmtMacro>) -> &mut Self {
        if let Some(imports) = imports {
            // get mac tokens
            self.imports.replace(imports.mac.tokens.clone());
        }
        self
    }
    pub fn set_events(&mut self, events: HashMap<String, TokenStream>) -> &mut Self {
        self.events = Some(events);
        self
    }
    pub fn push_event(&mut self, key: String, value: TokenStream) -> &mut Self {
        if self.events.is_none() {
            self.events.replace(HashMap::new());
        }
        self.events.as_mut().unwrap().insert(key, value);

        self
    }
    pub fn get_inherits(&self) -> Option<&BuiltIn> {
        self.inherits.as_ref()
    }
    pub fn set_prop_ptr(&mut self, prop_ptr: &Option<ItemStruct>) -> &mut Self {
        if !self.is_root {
            return self;
        }

        if let Some(prop_ptr) = prop_ptr {
            self.prop_ptr.replace(WidgetHandler::prop_ptr(
                prop_ptr,
                self.get_inherits().unwrap(),
            ));
        } else {
            // check current widget is root widgert? if true, set prop_ptr to BuiltIn::default_prop_ptr
            if self.is_root {
                self.prop_ptr.replace(
                    self.get_inherits()
                        .expect("root widget must has inherits")
                        .default_deref_ptr(&self.name),
                );
            }
        }

        self
    }
    pub fn set_event_ptr(&mut self, event_ptr: &Option<ItemEnum>) -> &mut Self {
        if let Some(event_ptr) = event_ptr {
            self.event_ptr.replace(WidgetHandler::event_ptr(event_ptr));
        }
        self
    }
    pub fn set_children(&mut self, children: Vec<Widget>) -> &mut Self {
        self.children = Some(children);
        self
    }
    pub fn push_child(&mut self, child: Widget) -> &mut Self {
        if self.children.is_none() {
            self.children.replace(vec![]);
        }
        self.children.as_mut().unwrap().push(child);

        self
    }
    pub fn set_inherits(&mut self, inherits: BuiltIn) -> &mut Self {
        self.inherits = Some(inherits);
        self
    }
    pub fn set_traits(&mut self, traits: WidgetTrait) -> &mut Self {
        self.traits.replace(traits);
        self
    }
    /// ## Set role
    /// if widget bind props has `for` or `if`
    pub fn set_role(
        &mut self,
        bind_props: Option<HashMap<&PropsKey, &Value>>,
        script: Option<&ScriptModel>,
        replacer: Option<&Replacer>,
    ) -> &mut Self {
        self.role = Role::Normal;
        if let Some(bind_props) = bind_props {
            // find `for` or `if` in bind props
            let mut for_flag = 0;
            let mut if_flag = 0;
            let mut if_signal = None;

            let (mut for_ident, mut for_index, mut for_item): (
                Option<String>,
                Option<String>,
                Option<String>,
            ) = (None, None, None);
            let mut if_ident: Option<String> = None;

            for (k, v) in &bind_props {
                // check for or if flag
                if (for_flag > 1 || if_flag > 1) && (for_flag + if_flag) >= 2 {
                    panic!("for or if flag must be one, and can not be both");
                }
                // set for or if flag and get for or if props to handle
                if k.name() == "for" {
                    for_flag += 1;
                    if let Value::Bind(Bind::For(For {
                        iter_ident,
                        index,
                        item,
                    })) = v
                    {
                        for_ident = Some(iter_ident.to_string());
                        for_index = index.as_ref().map(|v| v.to_string());
                        for_item = item.as_ref().map(|v| v.to_string());
                    }
                } else if k.name() == "if" || k.name() == "else" || k.name() == "else_if" {
                    if_flag += 1;
                    if_ident = Some(v.to_string());
                    if_signal = Some(IFSignal::from(k.name()));
                }
            }
            // now check for or if flag and handle wait_checks
            match (for_flag, if_flag) {
                (0_i32, 1_i32) => {
                    // filter bind props value and check if has if_ident
                    let props = bind_props
                        .into_iter()
                        .filter(|(_, v)| v.to_string().contains(if_ident.as_ref().unwrap()))
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect();

                    let if_ulid = if let Some(x) = replacer {
                        // x.get(&(self.name.to_string(), self.id.as_ref().unwrap().to_string()))
                        // only need id match, cause name has been changed, so iter map and find
                        x.iter().find_map(|((_, id), ulid)| {
                            if id == self.id.as_ref().unwrap() {
                                Some(ulid)
                            } else {
                                None
                            }
                        })
                    } else {
                        None
                    };

                    self.role = Role::new_option_if(props, if_signal.unwrap(), if_ulid);
                } // if
                (1_i32, 0_i32) => {
                    let props = bind_props
                        .into_iter()
                        .filter(|(_, v)| {
                            // in here Value mut be Bind and should be contain any of for_ident(maybe someone want to use), for_index, for_item
                            if let Value::Bind(Bind::Normal(n)) = v {
                                n.contains(for_ident.as_ref().unwrap())
                                    || n.contains(for_index.as_ref().unwrap())
                                    || n.contains(for_item.as_ref().unwrap())
                            } else {
                                false
                            }
                        })
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect();

                    // try to find loop_type from script by for_ident
                    let loop_type = if let Some(ScriptModel::Gen(gen)) = script {
                        gen.get_other()
                            .ty(for_ident.as_ref().unwrap())
                            .expect("for_ident must be a type")
                            .to_string()
                    } else {
                        String::new()
                    };
                    self.role = Role::new_for(
                        (for_ident.unwrap(), for_index, for_item).into(),
                        loop_type,
                        props,
                    );
                } // for
                _ => (),
            }
        }
        self
    }
    pub fn clear(&mut self) -> () {
        self.is_built_in = false;
        self.is_static = true;
        self.uses = None;
        // self.id = None;
        self.as_prop = false;
        self.source = None;
        self.imports = None;
        self.props = None;
        self.events = None;
        self.prop_ptr = None;
        self.event_ptr = None;
        self.event_ref = None;
        self.event_set = None;
        self.children = None;
        self.inherits = Some(BuiltIn::Area);
        self.traits = None;
        self.live_hook = None;
    }
    /// ## Handle role
    /// if widget's role is for or if, it will be special widget which need to handle builtin_widget
    /// if is special, the current widget will be replaced
    /// 1. replace widget name to `${widget_name}${ulid}`
    /// 2. directly remove props and children (these will be handled in for or if)
    pub fn handle_role(&mut self) -> &mut Self {
        match &self.role {
            Role::If { id, signal, .. } => {
                match signal {
                    IFSignal::If => {
                        // if signal is If, create a new safe widget from current widget, and replace current widget
                        let mut safety = SafeWidget::new_if_widget(&self);
                        self.id = None;
                        safety.append_tree(format!("if_{}: {}", &self.name, self.to_tree()));
                        self.id = safety.id.clone(); // back set id
                        safety.insert_to_auto();
                        let name = format!("IfWidget{}", id);
                        self.name = name;
                        self.clear();
                    }
                    IFSignal::ElseIf | IFSignal::Else => {
                        // if signal is ElseIf or Else, do not need to create a new widget, append current widget to safe widget(from AUTO_WIDGET) then clear current widget
                        let mut safeties = AUTO_BUILTIN_WIDGETS.lock().unwrap();

                        // find if widget
                        let mut safety = safeties.iter_mut().find(|widget| {
                            let source_match =
                                widget.source.as_ref().unwrap() == self.source.as_ref().unwrap();
                            let if_ulid_match = widget.role.match_role(id);

                            source_match && if_ulid_match.is_some_and(|x| x)
                        });

                        // if find, append else panic!
                        if safety.is_some() {
                            self.id = None;
                            let prefix = self.role.prefix_if().unwrap();
                            safety.as_mut().unwrap().append_tree(format!(
                                "{}_{}: {}",
                                prefix,
                                &self.name,
                                self.to_tree()
                            ));
                            safety
                                .as_mut()
                                .unwrap()
                                .push_child((self as &Widget).into());
                            self.clear();
                        } else {
                            panic!("can not find if widget to append else if widget")
                        }
                    }
                }

                self
            }
            Role::For { id, .. } => {
                let name = format!("{}{}", snake_to_camel(&self.name).unwrap(), id);
                // copy current widget and empty all
                let mut safety = SafeWidget::from(&*self);
                safety.append_tree(self.to_tree().to_string());
                safety.insert_to_auto();
                self.clear();
                self.name = name;
                self
            }
            Role::Normal => self,
        }
    }
    /// convert part or all of widget to live_design tree code, similar to `widget_children_tree`, but it start from self
    pub fn to_tree(&self) -> TokenStream {
        let mut tk = TokenStream::new();
        tk.extend(component_render(
            self.id.as_ref(),
            self.is_root,
            self.is_static,
            self.as_prop,
            &snake_to_camel(&self.name).unwrap(),
            self.props.clone(),
            self.widget_children_tree(),
        ));
        tk
    }
    pub fn widget_children_tree(&self) -> Option<TokenStream> {
        let mut tk = TokenStream::new();
        if let Some(children) = &self.children {
            for child in children {
                let Widget {
                    is_root,
                    is_static,
                    is_built_in,
                    id,
                    as_prop,
                    name,
                    props,
                    ..
                } = child;

                let name = if *is_built_in {
                    snake_to_camel(name).unwrap()
                } else {
                    name.to_string()
                };

                tk.extend(component_render(
                    id.as_ref(),
                    *is_root,
                    *is_static,
                    *as_prop,
                    &name,
                    props.clone(),
                    child.widget_children_tree(),
                ));
            }
            Some(tk)
        } else {
            None
        }
    }
}

impl ToLiveDesign for Widget {
    fn widget_uses(&self) -> Option<TokenStream> {
        let auto_widgets = AUTO_BUILTIN_WIDGETS.lock().unwrap();
        if auto_widgets.is_empty() {
            return None;
        } else {
            let auto_widgets = auto_widgets
                .iter()
                .filter(|widget| {
                    widget.source.as_ref().unwrap().compiled_file
                        == self.source.as_ref().unwrap().compiled_file
                })
                .fold(TokenStream::new(), |mut acc, widget| {
                    let item = parse_str::<TokenStream>(&widget.to_use_import()).unwrap();
                    acc.extend(item);
                    acc
                });

            Some(auto_widgets)
        }
    }
    /// get widget tree
    fn widget_tree(&self) -> Option<TokenStream> {
        let mut tk = TokenStream::new();
        // props and children
        let mut props_children = self.props.clone().unwrap_or_default();
        props_children.extend(self.widget_children_tree().unwrap_or_default());

        let ui = if self.is_static {
            self.id
                .as_ref()
                .expect("root widget need id to get widget tree")
                .to_string()
        } else {
            // self.source.as_ref().unwrap().source_name_lower()
            self.id.as_ref().unwrap_or(&self.name).to_string()
        };

        tk.extend(special_struct(
            &ui,
            &self.name,
            Some(props_children),
            self.is_static,
        ));

        if tk.is_empty() {
            None
        } else {
            Some(tk)
        }
    }
    fn widget_logic(&self) -> Option<TokenStream> {
        if !self.is_static {
            let mut tk = TokenStream::new();
            if let Some(uses_tk) = &self.uses {
                tk.extend(uses_tk.clone());
            }
            if let Some(prop_ptr_tk) = &self.prop_ptr {
                tk.extend(prop_ptr_tk.clone());
            }
            if let Some(event_ptr_tk) = &self.event_ptr {
                tk.extend(event_ptr_tk.clone());
            }
            // tk.extend(
            //     self.traits
            //         .as_ref()
            //         .unwrap()
            //         .to_token_stream(token_tree_ident(&self.name)),
            // );
            if let Some(traits_tk) = &self.traits {
                tk.extend(traits_tk.to_token_stream(ident(&self.name)))
            }

            if let Some(event_set_tk) = &self.event_set {
                tk.extend(event_set_tk.clone());
            }
            if let Some(event_ref_tk) = &self.event_ref {
                tk.extend(event_ref_tk.clone());
            }
            if let Some(live_hook_tk) = &self.live_hook {
                tk.extend(live_hook_tk.to_token_stream(ident(&self.name)));
            }

            if tk.is_empty() {
                None
            } else {
                Some(tk)
            }
        } else {
            None
        }
    }
    fn widget_imports(&self) -> Option<TokenStream> {
        let mut tk = if let Some(imports) = self.imports.as_ref() {
            let imports = imports.to_string();
            let imports = imports
                .split(";")
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();

            let tk = imports.iter().fold(TokenStream::new(), |mut acc, item| {
                let item: TokenStream = parse_str(item).unwrap();
                acc.extend(quote! {import #item;});
                acc
            });

            Some(tk)
        } else {
            None
        };

        // get auto widget import from AUTO_BUILTIN_WIDGETS
        let auto_widgets = AUTO_BUILTIN_WIDGETS.lock().unwrap();
        if auto_widgets.is_empty() {
            return tk;
        } else {
            let auto_widgets = auto_widgets
                .iter()
                .filter(|widget| {
                    widget.source.as_ref().unwrap().compiled_file
                        == self.source.as_ref().unwrap().compiled_file
                })
                .fold(TokenStream::new(), |mut acc, widget| {
                    let item = parse_str::<TokenStream>(&widget.to_live_import()).unwrap();
                    acc.extend(item);
                    acc
                });

            if tk.is_none() {
                tk.replace(auto_widgets);
            } else {
                tk.as_mut().unwrap().extend(auto_widgets);
            }

            tk
        }
    }
    fn to_live_design(&self) -> super::live_design::LiveDesign {
        self.into()
    }
}

impl From<gen_converter::model::Model> for Widget {
    fn from(value: gen_converter::model::Model) -> Self {
        // dbg!(&value);
        let gen_converter::model::Model {
            special,
            template,
            script,
            style,
            // compile,
            // is_entry,
            ..
        } = value;

        let template = template.unwrap();
        let (widget, _) = build_widget(
            Some(&special),
            &template,
            style.as_ref(),
            script.as_ref(),
            true,
            &mut None,
        );

        widget.unwrap()
    }
}

/// ## Replacer
/// <(widget_name, widget_id), ulid>
pub type Replacer = HashMap<(String, String), Ulid>;

/// ## Build widget
/// ### Params
/// - special: widget special
/// - template: widget template
/// - style: widget style
/// - script: widget script
/// - is_root: is root widget
/// - if_ulid: if widget use if to control, it need ulid to replace and pass to same level widget to handle
/// ### IF Ulid Pass Flow
/// ```
/// ┌──────────────┐                                                         
/// │  widget :if  │ ───────► set_role(ulid: None) here ulid will be created
/// └──────────────┘                                                         
///                                             │                            
///                                             ▼                            
///                                       ┌───────────┐                      
///                          ┌─────────── │  if ulid  │                      
///                          │            └───────────┘                      
///                          │                                               
///                          │   set_role()     │                            
///    now finish            │                  ▼                            
///                          │                                               
/// ┌──────────────┐         ▼         ┌──────────────────┐                  
/// │ widget else  │   ◄─────────────  │  widget :elseif  │  use if ulid     
/// └──────────────┘       pass        └──────────────────┘                  
///                                                                      
///  ```
fn build_widget(
    special: Option<&Source>,
    template: &TemplateModel,
    style: Option<&ConvertStyle>,
    script: Option<&ScriptModel>,
    is_root: bool,
    replacer: &mut Option<Replacer>,
) -> (Option<Widget>, Option<((String, String), Ulid)>) {
    let mut widget = Widget::new(
        special,
        template.get_name(),
        template.get_inherits(),
        template.get_id(),
        is_root,
        script,
    );

    // get styles from style by id
    let widget_styles = get_widget_styles(template.get_id(), template.get_class(), style);
    let widget_styles = combine_styles(widget_styles, template.get_unbind_props());
    // before all, check widget role from template  bind props
    widget
        .set_as_prop(template.as_prop)
        .set_props(widget_styles)
        .set_role(template.get_bind_props(), script, replacer.as_ref())
        .handle_role();

    if template.has_children() {
        // let mut child_replacer = None;
        for child in template.get_children().unwrap() {
            let (child_widget, c_replacer) =
                build_widget(special, child, style, script, false, replacer);

            let _ = c_replacer.map(|((name, id), ulid)| {
                // child_replacer.as_mut().unwrap().insert((name.to_string(), id.to_string()), ulid.clone());
                if replacer.is_none() {
                    replacer.replace(once(((name, id), ulid)).collect());
                } else {
                    replacer.as_mut().unwrap().insert((name, id), ulid);
                }
            });

            if let Some(child_widget) = child_widget {
                widget.push_child(child_widget);
            }
        }
    }

    widget.set_script(script, replacer.as_ref());

    // judget current widget role is if?, cause if widget(IFSignal::Else_IF and Else) need to be ignore, and IFSignal::If need to be replace(replace is handle in handle_role fn)
    if let RoleType::If(role) = RoleType::from(&widget.role) {
        let ulid = widget.role.get_if_uild();

        return match role {
            IFSignal::If => {
                let replacer = Some((
                    (widget.name.to_string(), widget.id.as_ref().unwrap().clone()),
                    ulid.unwrap(),
                ));
                (Some(widget), replacer)
            }
            IFSignal::ElseIf => (None, None),
            IFSignal::Else => (None, None),
        };
    }

    return (Some(widget), None);
}

/// get styles from style by id
fn get_widget_styles(
    id: Option<&String>,
    class: Option<&Value>,
    styles: Option<&ConvertStyle>,
) -> Option<HashMap<PropsKey, Value>> {
    match styles {
        Some(styles) => {
            let mut map = HashMap::new();
            if let Some(id) = id {
                if let Some(id_styles) = styles.get(id) {
                    map.extend(id_styles.clone());
                }
            }
            if let Some(class) = class {
                if let Some(class_styles) = styles.get(class.to_string().as_str()) {
                    map.extend(class_styles.clone());
                }
            }
            if map.is_empty() {
                None
            } else {
                Some(map)
            }
            // match id {
            //     Some(id) => match styles.get(id) {
            //         Some(style) => Some(style.clone()),
            //         None => None,
            //     },
            //     None => None,
            // }
        }
        None => None,
    }
}

fn combine_styles(
    l: Option<HashMap<PropsKey, Value>>,
    r: Option<HashMap<&PropsKey, &Value>>,
) -> Option<HashMap<PropsKey, Value>> {
    match (l, r) {
        (Some(l), Some(r)) => {
            let mut styles = l.clone();
            for (k, v) in r {
                styles.insert(k.clone(), v.clone());
            }
            Some(styles)
        }
        (Some(l), None) => Some(l),
        (None, Some(r)) => Some(r.into_iter().map(|(k, v)| (k.clone(), v.clone())).collect()),
        (None, None) => None,
    }
}

fn get_props_fields(prop_ptr: Option<&ItemStruct>) -> Option<Vec<Ident>> {
    if let Some(prop_ptr) = prop_ptr {
        let fields = prop_ptr
            .fields
            .iter()
            .map(|field| field.ident.clone().unwrap())
            .collect();
        Some(fields)
    } else {
        None
    }
}
