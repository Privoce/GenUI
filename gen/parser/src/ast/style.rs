use std::{collections::HashMap, fmt::Display};

use gen_utils::common::tokenizer::{FUNCTION_SIGN, HOLDER_END, HOLDER_START, IMPORT, STYLE_CLASS, STYLE_ID, STYLE_PESUDO};

use super::{props_to_style_string, ASTNodes, Props};

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum StyleType {
    // class: `.`
    Class,
    // id: `#`
    Id,
    // Pseudo: `::`
    Pseudo,
    // Import: `import`
    Import,
    // Identifier: `@`
    Identifier,
}

impl Default for StyleType {
    fn default() -> Self {
        Self::Class
    }
}

#[allow(dead_code)]
impl StyleType {
    pub fn is_class(&self) -> bool {
        matches!(self, Self::Class)
    }
    pub fn is_id(&self) -> bool {
        matches!(self, Self::Id)
    }
    pub fn is_pseudo(&self) -> bool {
        matches!(self, Self::Pseudo)
    }
}

impl From<&str> for StyleType {
    fn from(value: &str) -> Self {
        match value {
            STYLE_CLASS => StyleType::Class,
            STYLE_ID => StyleType::Id,
            STYLE_PESUDO => StyleType::Pseudo,
            IMPORT => StyleType::Import,
            FUNCTION_SIGN => StyleType::Identifier,
            _ => panic!("Invalid style"),
        }
    }
}

impl Display for StyleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            StyleType::Class => STYLE_CLASS,
            StyleType::Id => STYLE_ID,
            StyleType::Pseudo => STYLE_PESUDO,
            StyleType::Import => IMPORT,
            StyleType::Identifier => FUNCTION_SIGN,
        };
        f.write_str(res)
    }
}

/// # Style for ASTNodes
#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    name: String,
    ty: StyleType,
    props: Props,
    children: Option<Vec<ASTNodes>>,
    parent: Option<ASTNodes>,
}

#[allow(dead_code)]
impl Style {
    pub fn new(
        name: &str,
        props: Props,
        ty: StyleType,
        children: Option<Vec<ASTNodes>>,
        parent: Option<ASTNodes>,
    ) -> Self {
        Style {
            name: name.to_string(),
            ty,
            props,
            children,
            parent,
        }
    }
    pub fn new_style_start(name: &str, ty: StyleType) -> Self {
        Style {
            name: name.to_string(),
            ty,
            props: None,
            children: None,
            parent: None,
        }
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    pub fn set_ty(&mut self, ty: StyleType) {
        self.ty = ty;
    }
    pub fn set_props(&mut self, props: Props) {
        self.props = props;
    }
    pub fn extend_props(&mut self, props: HashMap<super::PropsKey, crate::Value>) {
        match self.props {
            Some(ref mut p) => {
                p.extend(props);
            }
            None => self.props = Some(props),
        }
    }
    pub fn set_children(&mut self, children: Vec<ASTNodes>) {
        match self.children {
            Some(_) => {
                let _ = self.children.replace(children);
            }
            None => self.children = Some(children),
        }
    }
    pub fn set_parent(&mut self, parent: ASTNodes) {
        match self.parent {
            Some(_) => {
                let _ = self.parent.replace(parent);
            }
            None => self.parent = Some(parent),
        }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_type(&self) -> StyleType {
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
    pub fn get_props(&self) -> Option<&HashMap<crate::PropsKey, crate::Value>> {
        self.props.as_ref()
    }
    pub fn get_parent(&self) -> Option<&ASTNodes> {
        self.parent.as_ref()
    }
    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // name and type
        let _ = f.write_fmt(format_args!(
            "{}{}{}",
            self.get_type().to_string(),
            self.get_name(),
            HOLDER_START
        ));

        // properties
        let props_str = props_to_style_string(self.props.clone());
        if !props_str.is_empty() {
            let _ = f.write_fmt(format_args!("{}", props_str));
        }
        // children

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
        f.write_str(HOLDER_END)
    }
}
