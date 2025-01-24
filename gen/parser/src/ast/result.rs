use std::{fmt::Display, sync::mpsc, thread};

use gen_utils::error::Error;

use crate::target::{parse_script, parse_style, template::html::parse_template};

use super::{ASTNodes, ParseCore, ParseTarget, Script, Strategy};

/// # Parse Result for GenUI DSL
///
/// ## Example
/// ```rust
/// let input = r#"...input string..."#;
/// let _ = ParseResult::try_from(ParseTarget::try_from(input).unwrap()).unwrap();
/// ```
#[derive(Debug, Clone, Default)]
pub struct ParseResult {
    pub template: Option<Vec<ASTNodes>>,
    pub style: Option<Vec<ASTNodes>>,
    pub script: Option<Script>,
}

impl ParseResult {
    pub fn set_template(&mut self, t: Vec<ASTNodes>) {
        let _ = self.template.replace(t);
    }
    pub fn set_script(&mut self, sc: Script) {
        let _ = self.script.replace(sc);
    }
    pub fn set_style(&mut self, s: Vec<ASTNodes>) {
        let _ = self.style.replace(s);
    }
    pub fn template(&self) -> Option<&Vec<ASTNodes>> {
        self.template.as_ref()
    }
    pub fn style(&self) -> Option<&Vec<ASTNodes>> {
        self.style.as_ref()
    }
    pub fn script(&self) -> Option<&Script> {
        self.script.as_ref()
    }
    pub fn has_template(&self) -> bool {
        self.template().is_some()
    }
    pub fn has_script(&self) -> bool {
        self.script().is_some()
    }
    pub fn has_style(&self) -> bool {
        self.style().is_some()
    }
    fn has(&self) -> (bool, bool, bool) {
        (self.has_template(), self.has_script(), self.has_style())
    }
    pub fn strategy(&self) -> Strategy {
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

impl TryFrom<ParseTarget> for ParseResult {
    type Error = Error;

    fn try_from(value: ParseTarget) -> Result<Self, Self::Error> {
        ParseCore::from(value).try_into()
    }
}

impl TryFrom<ParseCore> for ParseResult {
    type Error = Error;

    fn try_from(value: ParseCore) -> Result<Self, Self::Error> {
        match value.target_strategy() {
            Strategy::All => {
                let mut result = ParseResult::default();
                // channel
                let (sender, receiver) = mpsc::channel();
                let t_input = value.template().unwrap().clone();
                let s_input = value.style().unwrap().clone();
                result.script = value.script;
                let sender_t = sender.clone();
                // new thread to handle template
                thread::spawn(move || {
                    // let res_t = parse_template(&t_input);
                    // let res_t = match parse_template(&t_input) {
                    //     Ok(ast) => Ok(ast),
                    //     Err(_) => {
                    //         // if failed, try to parse ark template
                    //         match parse_ark_template(&t_input) {
                    //             Ok(ast) => Ok(ast),
                    //             Err(e) => Err(ParseError::template(&format!(
                    //                 "can not parse both Html like or ArkUI like: {}",
                    //                 e
                    //             ))
                    //             .into()),
                    //         }
                    //     }
                    // };
                    let res_t = parse_template(&t_input);
                    sender_t
                        .send((res_t, true))
                        .expect("failed to send template");
                });

                thread::spawn(move || {
                    let res_s = parse_style(&s_input);
                    // dbg!(&res_s);
                    sender.send((res_s, false)).expect("failed to send style");
                });
                for _ in 0..2 {
                    match receiver.recv().expect("failed to receive template") {
                        (Ok(ast), true) => result.set_template(ast),
                        (Ok(ast), false) => result.set_style(ast),
                        (Err(e), true) | (Err(e), false) => return Err(e),
                    };
                }

                Ok(result)
            }
            Strategy::TemplateScript => {
                let mut result = ParseResult::default();
                // channel
                let (sender, receiver) = mpsc::channel();
                let t_input = value.template().unwrap().clone();

                result.script = value.script;

                // new thread to handle template
                thread::spawn(move || {
                    // let res_t = match parse_template(&t_input) {
                    //     Ok(ast) => Ok(ast),
                    //     Err(_) => {
                    //         // if failed, try to parse ark template
                    //         match parse_ark_template(&t_input) {
                    //             Ok(ast) => Ok(ast),
                    //             Err(e) => Err(ParseError::template(&format!(
                    //                 "can not parse both Html like or ArkUI like: {}",
                    //                 e
                    //             ))
                    //             .into()),
                    //         }
                    //     }
                    // };
                    let res_t = parse_template(&t_input);
                    sender.send(res_t).expect("failed to send template");
                });
                match receiver.recv().expect("failed to receive template") {
                    Ok(ast) => {
                        let _ = result.set_template(ast);
                    }
                    Err(e) => {
                        return Err(e);
                    }
                };

                Ok(result)
            }
            Strategy::TemplateStyle => {
                let mut result = ParseResult::default();
                // channel
                let (sender, receiver) = mpsc::channel();
                let s_input = value.style().unwrap().clone();
                let t_input = value.template().unwrap();

                if let Err(e) = handle_template(&mut result, t_input) {
                    return Err(e);
                }
                // new thread to handle style
                thread::spawn(move || {
                    let res_s = parse_style(&s_input);
                    sender.send(res_s).expect("failed to send style");
                });
                match receiver.recv().expect("failed to receive style") {
                    Ok(ast) => {
                        let _ = result.set_style(ast);
                    }
                    Err(e) => {
                        return Err(e);
                    }
                };

                Ok(result)
            }
            Strategy::SingleTemplate => {
                let mut result = ParseResult::default();
                match handle_template(&mut result, value.template.unwrap().as_str()) {
                    Ok(_) => Ok(result),
                    Err(e) => Err(e),
                }
            }
            Strategy::Error(e) => {
                Err(format!("Gen Parse Strategy Error: {}, no such combination!", e).into())
            }
            Strategy::SingleScript => {
                let mut result = ParseResult::default();
                result.script = value.script;
                Ok(result)
            }
            Strategy::SingleStyle => {
                let mut result = ParseResult::default();
                match handle_style(&mut result, value.style.unwrap().as_str()) {
                    Ok(_) => Ok(result),
                    Err(e) => Err(e),
                }
            }
            Strategy::None => Ok(ParseResult::default()),
            _ => Err("The conversion strategy is temporarily not allowed to be processed".into()),
        }
    }
}

impl Display for ParseResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // only need to convert back ParseCore
        write!(
            f,
            "{}",
            ParseTarget::from(ParseCore::from(self.clone())).to_string()
        )
    }
}
/// ## handle template
/// use parse_template to parse template(string in <template> tag) to AST
/// - if success, set template to result and return Ok(())
/// - if failed, return Err(Error)
fn handle_template(result: &mut ParseResult, input: &str) -> Result<(), Error> {
    let ast = parse_template(input)?;
    result.set_template(ast);
    Ok(())
    // match parse_template(input) {
    //     Ok(ast) => {
    //         result.set_template(ast);
    //         Ok(())
    //     }
    //     Err(e) => {
    //         // 暂时不开启ark模板
    //         // if failed, try to parse ark template
    //         // match parse_ark_template(input) {
    //         //     Ok(ast) => {
    //         //         result.set_template(ast);
    //         //         Ok(())
    //         //     }
    //         //     Err(e) => Err(ParseError::template(&format!(
    //         //         "can not parse both Html like or ArkUI like: {}",
    //         //         e
    //         //     ))
    //         //     .into()),
    //         // }
    //         Err(e)
    //     }
    // }
}
/// ## handle script
#[deprecated]
#[allow(dead_code)]
fn handle_script(result: &mut ParseResult, input: &str) -> Result<(), Error> {
    match parse_script(input) {
        Ok(ast) => {
            result.set_script(ast.into());
            Ok(())
        }
        Err(e) => Err(e),
    }
}

/// ## handle style
/// use parse_style to parse style(string in <style> tag) to AST
fn handle_style(result: &mut ParseResult, input: &str) -> Result<(), Error> {
    match parse_style(input) {
        Ok(ast) => {
            result.set_style(ast);
            Ok(())
        }
        Err(e) => Err(e),
    }
}
