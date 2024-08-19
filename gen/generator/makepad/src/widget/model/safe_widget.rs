use std::collections::HashMap;

use gen_utils::common::{snake_to_camel, Source};

use crate::{compiler::AUTO_BUILTIN_WIDGETS, widget::BuiltIn};

use super::{
    role::Role,
    safe_traits::{SafeLiveHookTrait, SafeWidgetTrait},
    widget::Widget,
};

/// copy from Widget struct but it need to be used in lazy_static: AUTO_BUILTIN_WIDGETS
/// so this struct should be sync + send + safe
/// It replace String to String
#[derive(Debug, Default, Clone)]
pub struct SafeWidget {
    pub is_root: bool,
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
    pub imports: Option<String>,
    pub uses: Option<String>,
    // pub compiled_source: Option<PathBuf>,
    /// props in live_design
    pub props: Option<String>,
    /// events called in makepad
    pub events: Option<HashMap<String, String>>,
    pub prop_ptr: Option<String>,
    pub event_ptr: Option<String>,
    pub event_ref: Option<String>,
    pub event_set: Option<String>,
    pub children: Option<Vec<SafeWidget>>,
    pub inherits: Option<BuiltIn>,
    pub traits: Option<SafeWidgetTrait>,
    pub live_hook: Option<SafeLiveHookTrait>,
    // still need, maybe the for | if widgets are nested
    pub role: Role,
    /// the widget tree code, it should be set when SafeWidget is created (from Widget::handle_role()!!!)
    pub tree: Option<String>,
}

impl SafeWidget {
    pub fn insert_to_auto(self) -> () {
        let mut auto_widgets = AUTO_BUILTIN_WIDGETS.lock().unwrap();
        auto_widgets.push(self);
    }
    fn to_import(&self, prefix: &str)->String{
        let id = match &self.role {
            Role::If { id, .. } => id,
            Role::For { id, .. } => id,
            Role::Normal => panic!("normal widget not need to transform to safe widget!"),
        };

        format!(
            "{} crate::auto::{}_{}::*;",
            prefix,
            snake_to_camel(&self.name),
            id
        )
    }
    pub fn to_live_import(&self) -> String {
        self.to_import("import")
    }
    pub fn to_use_import(&self) -> String {
        self.to_import("use")
    }
    /// ## create a new Ifwidget(SafeWidget) from a Widget
    pub fn new_if_widget(widget: &Widget) -> Self {
        SafeWidget {
            is_root: widget.is_root,
            is_built_in: false,
            is_static: true,
            id: widget.id.clone(),
            as_prop: widget.as_prop,
            name: "IfWidget".to_string(),
            source: widget.source.clone(),
            imports: None,
            uses: None,
            props: None,
            events: None,
            prop_ptr: None,
            event_ptr: None,
            event_ref: None,
            event_set: None,
            children: Some(vec![widget.into()]),
            inherits: Some(BuiltIn::Area),
            traits: None,
            live_hook: None,
            role: widget.role.clone(),
            tree: None,
        }
    }
    /// ## append tree code to SafeWidget tree
    pub fn append_tree(&mut self, tree: String) -> () {
        match self.tree.as_mut() {
            Some(t) => {
                t.push_str(format!(", {}", tree).as_str());
            }
            None => {
                let _ = self.tree.replace(tree);
            }
        }
    }
    pub fn push_child(&mut self, child: SafeWidget) -> () {
        match self.children.as_mut() {
            Some(children) => {
                children.push(child);
            }
            None => {
                let _ = self.children.replace(vec![child]);
            }
        }
    }
}


impl From<&Widget> for SafeWidget {
    fn from(value: &Widget) -> Self {
        let Widget {
            is_root,
            is_built_in,
            is_static,
            id,
            as_prop,
            name,
            source,
            imports,
            uses,
            props,
            events,
            prop_ptr,
            event_ptr,
            event_ref,
            event_set,
            children,
            inherits,
            traits,
            live_hook,
            role,
        } = value;

        SafeWidget {
            is_root: *is_root,
            is_built_in: *is_built_in,
            is_static: *is_static,
            id: id.clone(),
            as_prop: *as_prop,
            name: name.clone(),
            source: source.clone(),
            imports: imports.as_ref().map(|x| x.to_string()),
            uses: uses.as_ref().map(|x| x.to_string()),
            props: props.as_ref().map(|x| x.to_string()),
            events: events.as_ref().map(|x| {
                x.into_iter()
                    .map(|(k, v)| (k.clone(), v.to_string()))
                    .collect()
            }),
            prop_ptr: prop_ptr.as_ref().map(|x| x.to_string()),
            event_ptr: event_ptr.as_ref().map(|x| x.to_string()),
            event_ref: event_ref.as_ref().map(|x| x.to_string()),
            event_set: event_set.as_ref().map(|x| x.to_string()),
            children: children
                .as_ref()
                .map(|x| x.iter().map(|item| item.into()).collect()),
            inherits: inherits.clone(),
            traits: traits.as_ref().map(|x| x.into()),
            live_hook: live_hook.as_ref().map(|x| x.into()),
            role: role.clone(),
            tree: None,
        }
    }
}
