pub mod comment;
mod nodes;
mod property;
mod result;
mod script;
mod style;
mod tag;

use comment::offline::OfflineComment;
pub use nodes::ASTNodes;

pub use property::*;
pub use result::ParseResult;
pub use script::Script;
#[allow(unused_imports)]
use std::{default, fmt::Display};
pub use style::{Style, StyleType};
pub use tag::{CloseType, Tag};

use self::nodes::asts_to_string;
use crate::{
    ast::comment::position::OfflinePosition, common::parse_all,
};
use gen_utils::{
    error::{Error, ParseError},
    parser::trim,
};

/// Parse Strategy
/// Convert ParseTarget To AST
#[derive(Debug, Clone, Default)]
pub enum Strategy {
    /// an empty file
    None,
    /// only has template tag
    SingleTemplate,
    /// only has rust script
    SingleScript,
    /// only has style tag
    SingleStyle,
    /// no template, rust script, style
    /// only comment (should with signatures)
    SingleComment,
    /// template with rust script
    TemplateScript,
    /// template with style
    TemplateStyle,
    /// template with comment
    TemplateComment,
    /// script with comment
    ScriptComment,
    /// style with comment
    StyleComment,
    TemplateScriptComment,
    TemplateStyleComment,
    /// has all means: TemplateScriptStyle
    #[default]
    All,
    Error(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Targets<'a> {
    Template(&'a str),
    Script { content: &'a str, ast_node: Tag },
    Style(&'a str),
    Comment(OfflineComment),
}

#[allow(dead_code)]
impl<'a> Targets<'a> {
    pub fn is_template(&self) -> bool {
        matches!(self, Targets::Template(_))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ParseCore {
    /// content of template tag
    template: Option<String>,
    /// content of script tag
    script: Option<Script>,
    /// content of style tag
    style: Option<String>,
}

impl From<ParseTarget> for ParseCore {
    fn from(value: ParseTarget) -> Self {
        value.core
    }
}

#[allow(dead_code)]
impl ParseCore {
    pub fn template(&self) -> Option<&String> {
        self.template.as_ref()
    }
    pub fn script(&self) -> Option<&Script> {
        self.script.as_ref()
    }
    pub fn style(&self) -> Option<&String> {
        self.style.as_ref()
    }
    pub fn has_template(&self) -> (bool, bool) {
        has_target(self.template())
    }
    pub fn has_script(&self) -> (bool, bool) {
        match self.script.as_ref() {
            Some(sc) => (!sc.is_empty(), false),
            None => (false, true),
        }
    }
    pub fn has_style(&self) -> (bool, bool) {
        has_target(self.style())
    }
    pub fn set_template_directly(&mut self, template: String) {
        let _ = self.template.replace(template);
    }
    pub fn set_script_directly(&mut self, script: &Script) {
        let _ = self.script.replace(script.clone());
    }
    pub fn set_style_directly(&mut self, style: String) {
        let _ = self.style.replace(style);
    }
    pub fn set_template(&mut self, template: &str) {
        let _ = self.template.replace(template.to_owned());
    }
    pub fn set_script(&mut self, content: &str, lang: Option<String>) -> Result<(), Error> {
        let _ = self.script.replace((content, lang).try_into()?);
        Ok(())
    }
    pub fn set_style(&mut self, style: &str) {
        let _ = self.style.replace(style.to_owned());
    }
    pub fn has(&self) -> (bool, bool, bool) {
        (
            self.has_template().0,
            self.has_script().0,
            self.has_style().0,
        )
    }
    pub fn target_strategy(&self) -> Strategy {
        match self.has() {
            (true, true, true) => Strategy::All,
            (true, true, false) => Strategy::TemplateScript,
            (true, false, true) => Strategy::TemplateStyle,
            (true, false, false) => Strategy::SingleTemplate,
            (false, true, true) => Strategy::Error(String::from(
                "Gen Parse Strategy Error: There is no such strategy `Script` + `Style`",
            )),
            (false, true, false) => Strategy::SingleScript,
            (false, false, true) => Strategy::SingleStyle,
            (false, false, false) => Strategy::None,
        }
    }
}

impl From<ParseResult> for ParseCore {
    fn from(value: ParseResult) -> Self {
        let mut result = ParseCore::default();
        if let Some(t) = value.template() {
            let _ = result.set_template_directly(asts_to_string(t));
        }
        if let Some(sc) = value.script() {
            let _ = result.set_script_directly(sc);
        }
        if let Some(s) = value.style() {
            let _ = result.set_style_directly(asts_to_string(s));
        }
        result
    }
}

/// # Parse Target
/// The target which will be parsed
///
/// Through this structure, you can obtain the page structure
///  
/// ## how to get
/// use nom to split the gen file
/// ## target check
/// When calling to determine the existence of fields in the parsing target, the actual content will be determined to be empty or not
/// > reject cheat syntax
/// ## Example
/// ```rust
/// let input = r#" ... "#;
/// let target = ParseTarget::try_from(input).unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ParseTarget {
    core: ParseCore,
    /// after parse the core 3 tag parser will consider the other remains are comment
    /// try to use comment parser to parse the remains
    /// if not have any allowed comment signatures --> panic!
    comment: Option<Vec<OfflineComment>>,
}

#[allow(dead_code)]
impl ParseTarget {
    pub fn set_template(&mut self, template: &str) {
        let _ = self.core.template.replace(template.to_owned());
    }
    pub fn set_script(&mut self, content: &str, lang: Option<String>)-> Result<(), Error> {
        self.core.set_script(content, lang)
    }
    pub fn set_style(&mut self, style: &str) {
        let _ = self.core.style.replace(style.to_owned());
    }
    pub fn set_comment(&mut self, comment: Vec<OfflineComment>) {
        let _ = self.comment.replace(comment);
    }
    pub fn push_comment(&mut self, comment: OfflineComment) {
        match &mut self.comment {
            Some(c) => c.push(comment),
            None => {
                let _ = self.comment.replace(vec![comment]);
            }
        }
    }
    pub fn template(&self) -> Option<&String> {
        self.core.template.as_ref()
    }
    pub fn script(&self) -> Option<&Script> {
        self.core.script()
    }
    pub fn style(&self) -> Option<&String> {
        self.core.style.as_ref()
    }
    pub fn comment(&self) -> Option<&Vec<OfflineComment>> {
        self.comment.as_ref()
    }
    pub fn has_template(&self) -> (bool, bool) {
        has_target(self.template())
    }
    pub fn has_script(&self) -> (bool, bool) {
        self.core.has_script()
    }
    pub fn has_style(&self) -> (bool, bool) {
        has_target(self.style())
    }
    /// judge whether has other comments
    pub fn has_comment(&self) -> (bool, bool) {
        match self.comment() {
            Some(v) => (!v.is_empty(), false),
            None => (false, true),
        }
    }
    pub fn has(&self) -> (bool, bool, bool, bool) {
        (
            self.has_template().0,
            self.has_script().0,
            self.has_style().0,
            self.has_comment().0,
        )
    }
    /// # handle Self to be better
    /// Call in TryFrom trait
    /// ## which need to handle
    /// is empty but not none
    pub fn handle_self(&mut self) {
        match self.has_template() {
            (false, false) => {
                self.core.template = None;
            }
            _ => {}
        }
        match self.has_script() {
            (false, false) => {
                self.core.script = None;
            }
            _ => {}
        }
        match self.has_style() {
            (false, false) => {
                self.core.style = None;
            }
            _ => {}
        }
    }
    /// Get ParseTarget Convert to AST Strategy
    /// This strategy affects how many threads are used for conversion
    ///
    /// 1. no <template> tag and no <style> tag  -->  parse as rust script (1 thread)
    /// 2. no <template> tag and no rust script has <style> tag  -->  parse as style (1 thread)
    /// 3. no <style> tag and no rust script has <template> tag  -->  parse as template (1 thread)
    /// 4. has <template> tag and rust script no <style> tag --> parse as template_script (2 thread)
    /// 5. has 3 tag --> parse as whole gen (3 thread)
    pub fn target_strategy(&self) -> Strategy {
        match self.has() {
            (true, true, true, true) | (true, true, true, false) => Strategy::All,
            (true, true, false, true) => Strategy::TemplateScriptComment,
            (true, true, false, false) => Strategy::TemplateScript,
            (true, false, true, true) => Strategy::TemplateStyleComment,
            (true, false, true, false) => Strategy::TemplateStyle,
            (true, false, false, true) => Strategy::TemplateComment,
            (true, false, false, false) => Strategy::SingleTemplate,
            (false, true, true, true) | (false, true, true, false) => {
                Strategy::Error(String::from(
                    "Gen Parse Strategy Error: There is no such strategy `Script` + `Style`",
                ))
            }
            (false, true, false, true) => Strategy::ScriptComment,
            (false, true, false, false) => Strategy::SingleScript,
            (false, false, true, true) => Strategy::StyleComment,
            (false, false, true, false) => Strategy::SingleStyle,
            (false, false, false, true) => Strategy::SingleComment,
            (false, false, false, false) => Strategy::None,
        }
    }
}

impl From<ParseCore> for ParseTarget {
    fn from(value: ParseCore) -> Self {
        ParseTarget {
            core: value,
            comment: None,
        }
    }
}

/// parse whole gen file from `Vec<Targets>` to `ParseTarget`
impl<'a> TryFrom<Vec<Targets<'a>>> for ParseTarget {
    type Error = Error;

    fn try_from(value: Vec<Targets>) -> Result<Self, Self::Error> {
        return if value.is_empty() {
            Err(ParseError::template("The current file has no content. It should be removed to ensure your program has clean file tree!").into())
        } else {
            let mut parse_target = ParseTarget::default();
            let mut template_count = 0_u32;
            let mut script_count = 0_u32;
            let mut style_count = 0_u32;
            for target in value {
                if is_multi_nodes(template_count, script_count, style_count) {
                    match target {
                        Targets::Template(t) => {
                            template_count += 1;
                            parse_target.set_template(t);
                        }
                        Targets::Script { content, ast_node } => {
                            script_count += 1;
                            let script_lang = ast_node.get_script_lang();
                            parse_target.set_script(content, script_lang)?;
                        }
                        Targets::Style(s) => {
                            style_count += 1;
                            parse_target.set_style(s);
                        }
                        Targets::Comment(c) => parse_target.push_comment(c),
                    }
                } else {
                    return Err(ParseError::template("Abnormal number of nodes, there is more than one `template` | `script` | `style` node in the file!").into());
                }
            }
            let _ = parse_target.handle_self();
            Ok(parse_target)
        };
    }
}

/// parse whole gen file from `&str` to `ParseTarget`
/// recommended to use this method to parse gen file directly
impl TryFrom<&str> for ParseTarget {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        return if value.trim().is_empty() {
            // Err(crate::error::Error::new("The current file has no content. It should be removed to ensure your program has clean file tree!"))
            Ok(ParseTarget {
                core: Default::default(),
                comment: None,
            })
        } else {
            let (remain, res) = trim(parse_all)(value).unwrap();
            if remain.is_empty() {
                // parse res to ParseTarget
                return ParseTarget::try_from(res);
            } else {
                // dbg!(remain);
                return Err(ParseError::template("Parsing file exception. The current file contains content that is not covered by processed tags. If it is a rust script, please wrap it in a `<script>` tag").into());
            }
        };
    }
}

impl Display for ParseTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let has_comment = self.has_comment().0;
        if has_comment {
            let _ = f.write_fmt(format_args!(
                "{}\n",
                &self
                    .comment()
                    .unwrap()
                    .iter()
                    .filter(|item| item.position() == OfflinePosition::AboveTemplate)
                    .map(|item| item.to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            ));
        }
        if self.has_template().0 {
            let _ = f.write_fmt(format_args!(
                "<template>\n{}</template>\n",
                self.template().unwrap()
            ));
        }
        if has_comment {
            let _ = f.write_fmt(format_args!(
                "\n{}",
                &self
                    .comment()
                    .unwrap()
                    .iter()
                    .filter(|item| item.position() == OfflinePosition::AboveScript)
                    .map(|item| item.to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            ));
        }
        if self.has_script().0 {
            let _ = f.write_fmt(format_args!(
                "\n<script>\n{}</script>\n",
                self.script().unwrap()
            ));
        }
        if has_comment {
            let _ = f.write_fmt(format_args!(
                "\n{}",
                &self
                    .comment()
                    .unwrap()
                    .iter()
                    .filter(|item| item.position() == OfflinePosition::AboveStyle)
                    .map(|item| item.to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            ));
        }
        if self.has_style().0 {
            let _ = f.write_fmt(format_args!(
                "\n<style>\n{}</style>\n",
                self.style().unwrap()
            ));
        }
        if has_comment {
            let _ = f.write_str(
                &self
                    .comment()
                    .unwrap()
                    .iter()
                    .filter(|item| item.position() == OfflinePosition::End)
                    .map(|item| item.to_string())
                    .collect::<Vec<String>>()
                    .join("\n"),
            );
        }
        f.write_str("\n")
    }
}

/// check whether the target is empty
/// ## return
/// `(bool, bool)` means:
/// 1. bool: is empty?
/// 2. bool: is Option::None?
fn has_target(target: Option<&String>) -> (bool, bool) {
    match target {
        Some(v) => (!v.is_empty(), false),
        None => (false, true),
    }
}

fn is_multi_nodes(t: u32, sc: u32, s: u32) -> bool {
    (t <= 1) && (sc <= 1) && (s <= 1)
}