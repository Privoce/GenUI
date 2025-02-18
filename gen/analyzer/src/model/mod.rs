mod script;
mod strategy;
// mod style;
mod template;

use nom::{
    bytes::complete::{tag, take_until}, combinator::opt, multi::many0, IResult
};
pub use script::*;
use std::{
    collections::HashMap,
    sync::mpsc::{self, RecvError},
    thread,
};
pub use strategy::*;

use crate::value::Value;

// use self::style::handle_styles;
// use gen_parser::{ParseResult, ParseTarget, Script, Strategy};
use gen_utils::{
    common::{fs, Source},
    error::{Error, ParseError}, parser::trim,
};

pub use template::*;

#[derive(Debug, Clone)]
pub enum ConvertResult {
    Template(Template),
    Style(Style),
}

pub type StyleVal = HashMap<PropKey, Value>;
/// also name Style
/// in gen-ui no difference between style and props
/// so we use the same struct to represent them
/// `<id|class, HashMap<prop, value>>`
pub type Style = HashMap<String, StyleVal>;

/// # GenUI文件模型
/// 用于表示一个.gen文件，这个文件会被解析为一个模型
/// 每个.gen文件会由解析器解析为一个AST，然后根据AST生成一个模型
/// 在解析器进行解析时，这个文件会被标识一个策略，这个策略会决定这个文件的转换方式
/// ## 策略说明
/// - 如果这个文件只有一个模版，那么这个文件会被标识为SingleTemplate策略
/// - 如果这个文件有模版和脚本，那么这个文件会被标识为TemplateScript策略
/// - ...
/// 通过策略,转换器可以知道如何处理这个文件
/// ## Example
/// ```rust
/// let source = Source::new(
///     "/Users/shengyifei/projects/gen_ui/GenUI/examples/ract/test1",
///     "hello/views/multi_fns.gen",
///     "src_gen_0/src/views/multi_fns.rs",
/// );
///
/// let model = Model::new(source, false).unwrap();
/// println!("{:?}", model);
/// ```
#[derive(Debug, Clone, Default)]
pub struct Model {
    /// 模型的唯一标识，通常被认为是该模型的文件路径，根据文件路径可以找到这个模型
    /// 这个字段在模型生成时会被设置
    pub special: Source,
    /// 模型的模版部分，即.gen文件的<template>标签包裹的部分
    pub template: Option<Template>,
    /// 模型的脚本部分，即.gen文件的<script>标签包裹的部分
    /// 这会是个Block代码块，后续会使用middleware/rssyin进行语义分析，处理代码替换（延时操作）
    /// 这里我注意到过早生成Gen脚本模型会导致无法获取到绑定的属性，所以这里不生成Gen脚本模型，而是在generator层生成
    pub script: Option<Script>,
    /// 模型的样式部分，即.gen文件的<style>标签包裹的部分
    /// 也可以认为是模型的属性部分，在GenUI中并没有属性与样式的区别
    /// Style实际上是被平展的样式列表
    pub style: Option<Style>,
    /// 模型是否需要被编译
    /// 在项目中可能存在一个文件被编写，但没有在项目中使用到
    /// 表现为这个文件没有使用Rust的use语句进行引入
    pub compile: bool,
    /// 是否是入口文件
    pub is_entry: bool,
    /// 转换策略
    pub strategy: Strategy,
}

impl Model {
    pub fn new(source: Source, is_entry: bool) -> Result<Self, gen_utils::error::Error> {
        let content = fs::read(source.from_path())?;
        let mut model = Model::default();
        // let ast = ParseResult::try_from(ParseTarget::try_from(content.as_str())?)?;
        model.parse(&content)?;
        model.special = source;

        // model.strategy = ast.strategy();
        // let _ = Model::convert(&mut model, ast);
        model.is_entry = is_entry;
        Ok(model)
    }

    pub fn is_empty(&self) -> bool {
        self.template.is_none() && self.script.is_none() && self.style.is_none()
    }

    pub fn set_template(&mut self, template: Template) -> () {
        let _ = self.template.replace(template);
    }
    pub fn set_style(&mut self, style: Style) -> () {
        let _ = self.style.replace(style);
    }
    pub fn is_component(&self) -> bool {
        if self.template.is_some() {
            return self.template.as_ref().unwrap().is_component();
        }
        false
    }
    /// if model is compoent return None else Some(root_name)
    pub fn is_component_and_root(&self) -> Option<String> {
        if self.is_component() {
            None
        } else {
            match self.template.as_ref() {
                Some(t) => t.id.clone(),
                None => None,
            }
        }
    }
    // pub fn get_binds_tree(&self) -> Option<(PropTree, PropTree)> {
    //     match self.template.as_ref() {
    //         Some(template) => Some(template.get_props_tree()),
    //         None => None,
    //     }
    // }

    // /// 通过parser层解析的结果和文件路径生成converter层模型
    // /// 这一层只需要处理template和style部分，script不变
    // fn convert(model: &mut Model, ast: ParseResult) -> Result<(), gen_utils::error::Error> {
    //     // get strategy
    //     match &ast.strategy() {
    //         Strategy::None => Ok(()),
    //         Strategy::SingleTemplate => {
    //             let _ =
    //                 model.set_template(Template::convert(&ast.template().unwrap()[0], true)?);
    //             Ok(())
    //         }
    //         Strategy::SingleScript => {
    //             model.script = ast.script;
    //             Ok(())
    //         }
    //         Strategy::SingleStyle => handle_styles(ast.style().unwrap()).map_or_else(
    //             || {
    //                 Err(ConvertError::FromTo {
    //                     from: "GenUI Common AST (Style)".to_string(),
    //                     to: "GenUI AST, Invaild Style".to_string(),
    //                 }
    //                 .into())
    //             },
    //             |res| {
    //                 model.set_style(res);
    //                 Ok(())
    //             },
    //         ),
    //         Strategy::TemplateScript => {
    //             let (sender, receiver) = mpsc::channel();
    //             let template = ast.template().unwrap()[0].clone();

    //             let _ = thread::spawn(move || {
    //                 let convert_res = Template::convert(&template, true);
    //                 sender.send(convert_res).expect("send template error");
    //             });

    //             match receiver
    //                 .recv()
    //                 .expect("gen_converter: receive failed when convert!")
    //             {
    //                 Ok(t) => {
    //                     model.set_template(t);
    //                 }
    //                 Err(e) => {
    //                     return Err(e);
    //                 }
    //             }
    //             // 处理script部分
    //             model.script = ast.script;
    //             Ok(())
    //         }
    //         Strategy::TemplateStyle => {
    //             let (sender, receiver) = mpsc::channel();
    //             let template = ast.template().unwrap()[0].clone();
    //             let styles = ast.style().unwrap().clone();
    //             let _ = thread::spawn(move || {
    //                 let convert_res = handle_styles(&styles);
    //                 sender
    //                     .send(ConvertResult::Style(convert_res))
    //                     .expect("send style error");
    //             });

    //             match receiver
    //                 .recv()
    //                 .expect("gen_converter: receive failed when convert!")
    //             {
    //                 ConvertResult::Style(s) => {
    //                     if s.is_some() {
    //                         model.set_style(s.unwrap());
    //                     }
    //                     // else {
    //                     //     panic!("style cannot be none in Strategy::TemplateStyle")
    //                     // }
    //                 }
    //                 _ => panic!("Invalid strategy!"),
    //             }

    //             let convert_template = Template::convert(&template, true);
    //             let _ = model.set_template(
    //                 convert_template.expect("template cannot be none in Strategy::TemplateStyle"),
    //             );
    //             Ok(())
    //         }
    //         Strategy::All => {
    //             let (sender, receiver) = mpsc::channel();
    //             let template_sender = sender.clone();
    //             let style_sender = sender.clone();
    //             let template = ast.template().unwrap()[0].clone();
    //             let styles = ast.style().unwrap().clone();
    //             let _ = thread::spawn(move || {
    //                 let convert_res = Template::convert(&template, true);
    //                 template_sender
    //                     .send(ConvertResult::Template(convert_res))
    //                     .expect("send template error");
    //             });
    //             let _ = thread::spawn(move || {
    //                 let convert_res = handle_styles(&styles);
    //                 style_sender
    //                     .send(ConvertResult::Style(convert_res))
    //                     .expect("send style error");
    //             });

    //             for _ in 0..2 {
    //                 match receiver
    //                     .recv()
    //                     .expect("gen_converter: receive failed when convert!")
    //                 {
    //                     ConvertResult::Template(t) => match t {
    //                         Ok(t) => {
    //                             model.set_template(t);
    //                         }
    //                         Err(e) => {
    //                             return Err(e);
    //                         }
    //                     },
    //                     ConvertResult::Style(s) => {
    //                         if s.is_some() {
    //                             model.set_style(s.unwrap());
    //                         }
    //                     }
    //                 }
    //             }
    //             // 处理script部分
    //             model.script = ast.script;
    //             Ok(())
    //         }
    //         _ => Err(ConvertError::FromTo {
    //             from: "GenUI Common AST".to_string(),
    //             to: "GenUI AST, Invaild Strategy!".to_string(),
    //         }
    //         .into()),
    //     }
    // }

    // pub fn set_special(&mut self, special: &PathBuf, source: &PathBuf) -> () {
    //     if self.special.as_os_str().is_empty() {
    //         self.special = (special, source).into();
    //     } else {
    //         panic!("special is already set");
    //     }
    // }

    /// parse gen file
    /// try parse `<template>...</template>`, `<style>...</style>`, `<script>...</script>`
    /// use nom take till
    pub fn parse(&mut self, input: &str) -> Result<(), Error> {
        fn parse_tag<'a>(name: &'a str) -> impl Fn(&'a str) -> IResult<&'a str, Option<String>> {
            move |input: &str| {
                let (input, _) = many0(Comment::parse)(input)?;
                let (input, _) = trim(tag(format!("<{}>", name).as_str()))(input)?;
                let (input, template_str) = trim(take_until(format!("</{}>", name).as_str()))(input)?;
                let (input, _) = trim(tag(format!("</{}>", name).as_str()))(input)?;
                if template_str.is_empty() {
                    Ok((input, None))
                } else {
                    Ok((input, Some(template_str.to_string())))
                }
            }
        }

        let (input, template) =
            opt(parse_tag("template"))(input).map_err(|e| Error::from(e.to_string()))?;
        let (input, style) =
            opt(parse_tag("style"))(input).map_err(|e| Error::from(e.to_string()))?;
        let (input, script) =
            opt(parse_tag("script"))(input).map_err(|e| Error::from(e.to_string()))?;
        if !input.trim().is_empty() {
            return Err(ParseError::template("parse error!").into());
        }

        let template = template.flatten();
        let style = style.flatten();
        let script = script.flatten();

        match (template, style, script) {
            (Some(t), Some(s), Some(sc)) => {
                self.strategy = Strategy::All;
                let (sender, receiver) = mpsc::channel();
                let style_sender = sender.clone();
                let _ = thread::spawn(move || -> Result<(), Error> {
                    let res = Template::parse(&t)?;
                    sender
                        .send(ConvertResult::Template(res))
                        .expect("send template error");
                    Ok(())
                });
                let _ = thread::spawn(move || -> Result<(), Error> {
                    let res = crate::parse::style::parse(&s)?;
                    style_sender
                        .send(ConvertResult::Style(res))
                        .expect("send style error");
                    Ok(())
                });
                let _ = receiver
                    .recv()
                    .and_then(|t| {
                        if let ConvertResult::Template(t) = t {
                            self.template.replace(t);
                            Ok(())
                        } else {
                            Err(RecvError)
                        }
                    })
                    .map_err(|_| ParseError::template("template parse error!"))?;
                let _ = receiver
                    .recv()
                    .and_then(|s| {
                        if let ConvertResult::Style(s) = s {
                            self.style.replace(s);
                            Ok(())
                        } else {
                            Err(RecvError)
                        }
                    })
                    .map_err(|_| ParseError::template("style parse error!"))?;
                self.script.replace(sc.into());
            }
            (Some(t), Some(s), None) => {
                self.strategy = Strategy::TemplateStyle;
                let (sender, receiver) = mpsc::channel();
                let _ = thread::spawn(move || -> Result<(), Error> {
                    let res = Template::parse(&t)?;
                    sender.send(res).expect("send template error");
                    Ok(())
                });
                let _ = receiver.recv().map(|t| self.template.replace(t));
                self.style.replace(crate::parse::style::parse(&s)?);
            }
            (Some(t), None, Some(sc)) => {
                self.strategy = Strategy::TemplateScript;
                let (sender, receiver) = mpsc::channel();
                let _ = thread::spawn(move || -> Result<(), Error> {
                    let res = Template::parse(&t)?;
                    sender.send(res).expect("send template error");
                    Ok(())
                });
                let _ = receiver.recv().map(|t| self.template.replace(t));
                self.script.replace(sc.into());
            }
            (Some(t), None, None) => {
                self.strategy = Strategy::SingleTemplate;
                self.template.replace(Template::parse(&t)?);
            }
            (None, Some(s), None) => {
                self.strategy = Strategy::SingleStyle;
                self.style.replace(crate::parse::style::parse(&s)?);
            }
            (None, None, Some(sc)) => {
                self.strategy = Strategy::SingleScript;
                self.script.replace(sc.into());
            }
            (None, None, None) => {
                self.strategy = Strategy::None;
            }
            _ => {
                return Err(ParseError::template("the parse strategy is invalid!").into());
            }
        }

        Ok(())
    }
}
