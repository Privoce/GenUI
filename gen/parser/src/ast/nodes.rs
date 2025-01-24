use std::{collections::HashMap, fmt::Display};

use crate::Value;

use super::{comment::Comments, tag::CloseType, Props, PropsKey, Style, Tag};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum ASTNodes {
    /// ### template tag
    /// - `<template>`
    /// - `<script>`
    /// - `<style>`
    /// - `<any_component>`
    /// - ...
    Tag(Box<Tag>),
    /// ### Comment
    /// display everywhere
    /// - `///`
    /// - `//`
    /// - `//!`
    Comment(Box<Comments>),
    /// ### Style (Properties)
    /// - `.`
    /// - `#`
    /// - `&::`
    Style(Box<Style>),
}

#[allow(dead_code)]
impl ASTNodes {
    pub fn is_tag(&self) -> bool {
        matches!(self, Self::Tag(_))
    }
    pub fn is_comment(&self) -> bool {
        matches!(self, Self::Comment(_))
    }
    pub fn is_style(&self) -> bool {
        matches!(self, Self::Style(_))
    }
    pub fn set_tag_type(&mut self, ty: CloseType) {
        match self {
            ASTNodes::Tag(t) => t.set_ty(ty),
            _ => panic!("only ASTNodes::Tag can use `set_tag_type()`"),
        }
    }
    pub fn set_tag_properties(&mut self, props: Props) {
        match self {
            ASTNodes::Tag(t) => t.set_props(props),
            _ => panic!("only ASTNodes::Tag can use `set_tag_properties()`"),
        }
    }

    pub fn set_style_properties(&mut self, props: Props) {
        match self {
            ASTNodes::Style(s) => s.set_props(props),
            _ => panic!("only ASTNodes::Style can use `set_style_properties()`"),
        }
    }
    pub fn set_properties(&mut self, props: Props) {
        match self {
            ASTNodes::Tag(t) => t.set_props(props),
            ASTNodes::Comment(_) => {}
            ASTNodes::Style(s) => s.set_props(props),
        }
    }
    pub fn extend_properties(&mut self, props: HashMap<PropsKey, Value>) {
        match self {
            ASTNodes::Tag(t) => t.extend_props(props),
            ASTNodes::Comment(_) => {}
            ASTNodes::Style(s) => s.extend_props(props),
        }
    }
    pub fn get_tag_name(&self) -> &str {
        match self {
            ASTNodes::Tag(t) => t.get_name(),
            _ => panic!("only ASTNodes::Tag can use `get_tag_name()`"),
        }
    }
    pub fn set_tag_children(&mut self, children: Vec<ASTNodes>) {
        match self {
            ASTNodes::Tag(t) => t.set_children(children),
            _ => panic!("only ASTNodes::Tag can use `set_tag_children()`"),
        }
    }
    pub fn push_tag_children(&mut self, child: ASTNodes) {
        match self {
            ASTNodes::Tag(t) => t.push_children(child),
            _ => panic!("only ASTNodes::Tag can use `push_tag_children()`"),
        }
    }
    pub fn extend_tag_children(&mut self, children: Vec<ASTNodes>) {
        match self {
            ASTNodes::Tag(t) => t.extend_children(children),
            _ => panic!("only ASTNodes::Tag can use `extend_tag_children()`"),
        }
    }
    pub fn set_style_children(&mut self, children: Vec<ASTNodes>) {
        match self {
            ASTNodes::Style(s) => s.set_children(children),
            _ => panic!("only ASTNodes::Style can use `set_style_children()`"),
        }
    }
    pub fn set_children(&mut self, children: Vec<ASTNodes>) {
        match self {
            ASTNodes::Tag(t) => t.set_children(children),
            ASTNodes::Comment(_) => {}
            ASTNodes::Style(s) => s.set_children(children),
        }
    }
    pub fn set_tag_parent(&mut self, parent: ASTNodes) {
        match self {
            ASTNodes::Tag(t) => t.set_parent(parent),
            _ => panic!("only ASTNodes::Tag can use `set_tag_parent()`"),
        }
    }
    pub fn set_parent(&mut self, parent: ASTNodes) {
        match self {
            ASTNodes::Tag(t) => t.set_parent(parent),
            ASTNodes::Comment(_) => {}
            ASTNodes::Style(s) => s.set_parent(parent),
        }
    }
    /// first bool is tag?
    /// second bool is self closed?
    pub fn is_tag_close(&self) -> (bool, bool) {
        match self {
            ASTNodes::Tag(t) => (true, t.is_self_closed()),
            _ => (false, false),
        }
    }
    pub fn get_name(&self) -> &str{
        match self {
            ASTNodes::Tag(t) => t.get_name(),
            ASTNodes::Comment(_) => panic!("Comment has no name"),
            ASTNodes::Style(s) => s.get_name(),
        }
    }
    pub fn is_style_and_prefix(&self) -> String {
        match self {
            ASTNodes::Style(s) => {
                format!("{}{}", s.get_type().to_string(), s.get_name()) 
            },
            _ => panic!("only AST style can use `is_style_and_prefix()`"),
        }
    }
    pub fn is_tag_and_get(&self) -> Option<&Tag> {
        match self {
            ASTNodes::Tag(t) => Some(t),
            _ => None,
        }
    }
    // pub fn parse_template(input:&str) -> Vec<ASTNodes>{
    //     parse_template(input)
    // }

    // pub fn parse(input:&str)->IResult<Vec<>>{

    // }
}

impl From<Tag> for ASTNodes {
    fn from(value: Tag) -> Self {
        ASTNodes::Tag(Box::new(value))
    }
}

impl From<Comments> for ASTNodes {
    fn from(value: Comments) -> Self {
        ASTNodes::Comment(Box::new(value))
    }
}

impl From<Style> for ASTNodes {
    fn from(value: Style) -> Self {
        ASTNodes::Style(Box::new(value))
    }
}

impl Display for ASTNodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            ASTNodes::Tag(t) => t.to_string(),
            ASTNodes::Comment(c) => c.to_string(),
            ASTNodes::Style(s) => s.to_string(),
        };
        f.write_str(&res)
    }
}

/// convert Vec<ASTNodes> to String
pub fn asts_to_string(asts: &Vec<ASTNodes>) -> String {
    asts.into_iter().map(|x| x.to_string()).collect::<String>()
}
