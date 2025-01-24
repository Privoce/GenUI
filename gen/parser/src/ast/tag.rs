use crate::{PropsKey, Value};
use gen_utils::common::tokenizer::{END_SIGN, END_START_SIGN, SELF_END_SIGN, TAG_START};
use std::{collections::HashMap, fmt::Display};

use super::{props_to_template_string, ASTNodes, Props};

/// # CloseType for Tag
/// - SelfClosed
/// - Normal
/// ## SelfClosed
/// `<input />`
/// ## Normal
/// `<input></input>`
#[derive(Debug, Clone, PartialEq)]
pub enum CloseType {
    /// <xxx /> -> `/>`
    SelfClosed,
    /// <xxx></xxx> -> `>`
    Normal,
}

#[allow(dead_code)]
impl CloseType {
    pub fn is_normal(&self) -> bool {
        matches!(self, CloseType::Normal)
    }
    pub fn is_self_close(&self) -> bool {
        !self.is_normal()
    }
}

impl Default for CloseType {
    fn default() -> Self {
        CloseType::Normal
    }
}

impl Display for CloseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            CloseType::SelfClosed => SELF_END_SIGN,
            CloseType::Normal => END_SIGN,
        };
        f.write_str(res)
    }
}

impl From<&str> for CloseType {
    fn from(value: &str) -> Self {
        match value {
            SELF_END_SIGN => CloseType::SelfClosed,
            END_SIGN => CloseType::Normal,
            _ => panic!("Invalid close type"),
        }
    }
}

/// # Tag
/// This struct is used to represent a tag in the AST
/// ## Tag Type
/// - self close tag: `<tag_name key="value" />`
/// - normal tag: `<tag_name key="value">..[nested tags]</tag_name>`
#[derive(Debug, Clone, PartialEq)]
pub struct Tag {
    /// tag name
    name: String,
    /// tag close type: self close or normal
    ty: CloseType,
    /// tag props and value
    props: Props,
    /// children tags
    children: Option<Vec<ASTNodes>>,
    /// parent tag
    parent: Option<Box<ASTNodes>>,
}

#[allow(dead_code)]
impl Tag {
    pub fn new(
        name: &str,
        props: Props,
        ty: CloseType,
        children: Option<Vec<ASTNodes>>,
        parent: Option<ASTNodes>,
    ) -> Self {
        let name = name.to_string();
        let parent = match parent {
            Some(p) => Some(Box::new(p)),
            None => None,
        };
        Tag {
            name: name.to_string(),
            ty,
            props,
            children,
            parent,
        }
    }
    /// ## new tag start
    /// new a tag start without props, children and parent
    pub fn new_tag_start(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ty: Default::default(),
            props: None,
            children: None,
            parent: None,
        }
    }
    /// ## new tag props
    /// new a tag with props, without children and parent
    pub fn new_tag_props(name: &str, props: Props) -> Self {
        Self {
            name: name.to_string(),
            ty: Default::default(),
            props,
            children: None,
            parent: None,
        }
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    pub fn set_ty(&mut self, ty: CloseType) {
        self.ty = ty;
    }
    pub fn set_props(&mut self, props: Props) {
        self.props = props;
    }
    pub fn set_children(&mut self, children: Vec<ASTNodes>) {
        match self.children {
            Some(_) => {
                let _ = self.children.replace(children);
            }
            None => self.children = Some(children),
        }
    }
    /// push a child to children
    pub fn push_children(&mut self, child: ASTNodes) {
        match self.children {
            Some(ref mut children) => {
                children.push(child);
            }
            None => {
                self.children = Some(vec![child]);
            }
        }
    }
    /// extend children to children
    pub fn extend_children(&mut self, children: Vec<ASTNodes>) {
        match self.children {
            Some(ref mut c) => {
                c.extend(children);
            }
            None => {
                self.children = Some(children);
            }
        }
    }
    pub fn set_parent(&mut self, parent: ASTNodes) {
        match self.parent {
            Some(_) => {
                let _ = self.parent.replace(Box::new(parent));
            }
            None => self.parent = Some(Box::new(parent)),
        }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_type(&self) -> CloseType {
        self.ty.clone()
    }
    pub fn has_children(&self) -> bool {
        self.children.is_some()
    }
    pub fn get_children(&self) -> Option<&Vec<ASTNodes>> {
        self.children.as_ref()
    }
    pub fn has_props(&self) -> bool {
        self.props.is_some()
    }
    pub fn get_props(&self) -> Option<&HashMap<PropsKey, Value>> {
        self.props.as_ref()
    }
    /// is current tag is self closed or not
    pub fn is_self_closed(&self) -> bool {
        self.ty.is_self_close()
    }
    pub fn extend_props(&mut self, props: HashMap<PropsKey, Value>) {
        match self.props {
            Some(ref mut p) => {
                p.extend(props);
            }
            None => {
                self.props = Some(props);
            }
        }
    }
    /// ## get script lang from Tag
    /// if tag is script tag(`<script lang="xxx">`), return script lang
    ///
    /// if tag is not script return None or return default lang rust
    /// ### Attention
    /// - if tag is `<script>`, return lang is rust
    /// - lang is in props field
    pub fn get_script_lang(&self) -> Option<String> {
        if self.get_name() == "script" {
            match self.props.as_ref() {
                Some(props) => {
                    return props
                        .get(&PropsKey::new_tag_normal("lang"))
                        .map_or(Some("rust".to_string()), |lang| Some(lang.to_string()));
                }
                None => return Some("rust".to_string()),
            }
        }
        None
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f.write_fmt(format_args!("{}{}", TAG_START, self.get_name(),));

        let props_str = props_to_template_string(self.props.clone());
        if !props_str.is_empty() {
            let _ = f.write_fmt(format_args!(" {}", props_str));
        }
        match self.ty {
            CloseType::SelfClosed => f.write_str(" />"),
            CloseType::Normal => {
                let _ = f.write_str(END_SIGN);
                // add children
                if self.has_children() {
                    let _ = f.write_fmt(format_args!(
                        "\n{}",
                        self.children
                            .as_ref()
                            .unwrap()
                            .into_iter()
                            .map(|item| item.to_string())
                            .collect::<Vec<String>>()
                            .join("\n")
                    ));
                    let _ = f.write_str("\n");
                }
                f.write_fmt(format_args!(
                    "{}{}{}",
                    END_START_SIGN,
                    self.get_name(),
                    END_SIGN
                ))
            }
        }
    }
}