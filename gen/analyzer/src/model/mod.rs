mod poll;
mod script;
mod strategy;
mod style;
mod template;

use nom::{
    bytes::complete::{tag, take_until},
    combinator::opt,
    multi::many0,
    IResult,
};
pub use poll::*;
pub use script::*;
use std::{
    str::FromStr, sync::{
        mpsc::{self},
        Arc, RwLock,
    }, thread
};
pub use strategy::*;
pub use style::*;
use gen_utils::{
    common::{fs, Source},
    error::{Error, ParseError},
    parser::trim,
};
pub use template::*;

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
    /// 池化属性和回调
    pub polls: Arc<RwLock<Polls>>,
}

impl FromStr for Model {
    type Err = gen_utils::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut model = Model::default();
        model.parse(s)?;
        Ok(model)
    }
}

impl Model {
    pub fn new(source: Source, is_entry: bool) -> Result<Self, gen_utils::error::Error> {
        let content = fs::read(source.from_path())?;
        let mut model = Model::default();
        model.parse(&content)?;
        model.special = source;
        model.is_entry = is_entry;
        Ok(model)
    }

    pub fn is_empty(&self) -> bool {
        self.template.is_none() && self.script.is_none() && self.style.is_none()
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
                let (input, template_str) =
                    trim(take_until(format!("</{}>", name).as_str()))(input)?;
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
        let (input, script) =
            opt(parse_tag("script"))(input).map_err(|e| Error::from(e.to_string()))?;
        let (input, style) =
            opt(parse_tag("style"))(input).map_err(|e| Error::from(e.to_string()))?;
        if !input.trim().is_empty() {
            return Err(
                ParseError::template(&format!("parse error! Still remain: {}", input)).into(),
            );
        }

        let template = template.flatten();
        let style = style.flatten();
        let script = script.flatten();
        match (template, style, script) {
            (Some(t), Some(s), Some(sc)) => {
                self.strategy = Strategy::All;
                let (template_sender, template_receiver) = mpsc::channel();
                let (style_sender, style_receiver) = mpsc::channel();
                let poll = Arc::clone(&self.polls);
                let _ = thread::spawn(move || -> Result<(), Error> {
                    let res = Template::parse(&t, poll);
                    template_sender.send(res).expect("send template error");
                    Ok(())
                });

                let _ = thread::spawn(move || -> Result<(), Error> {
                    let res = crate::parse::style::parse(&s);
                    style_sender.send(res).expect("send style error");
                    Ok(())
                });

                // wait for parse result
                match (template_receiver.recv(), style_receiver.recv()) {
                    (Ok(template), Ok(style)) => {
                        self.template.replace(template?);
                        self.style.replace(style?);
                        self.script.replace(sc.into());
                    }
                    (Ok(_), Err(e)) => {
                        return Err(Error::from(format!("receive style error: {}", e)));
                    }
                    (Err(e), Ok(_)) => {
                        return Err(Error::from(format!("receive template error: {}", e)));
                    }
                    (Err(e_t), Err(e_s)) => {
                        return Err(Error::from(format!("receive template and style error:\ntemplate error: {}\nstyle error: {}", e_t, e_s)));
                    }
                }
            }
            (Some(t), Some(s), None) => {
                self.strategy = Strategy::TemplateStyle;
                let poll = Arc::clone(&self.polls);
                let (sender, receiver) = mpsc::channel();
                let _ = thread::spawn(move || -> Result<(), Error> {
                    let res = Template::parse(&t, poll);
                    sender.send(res).expect("send template error");
                    Ok(())
                });
                let _ = receiver.recv().map_or_else(
                    |e| Err(Error::from(format!("receive template error: {}", e))),
                    |t| {
                        self.template.replace(t?);
                        Ok(())
                    },
                )?;

                self.style.replace(crate::parse::style::parse(&s)?);
            }
            (Some(t), None, Some(sc)) => {
                self.strategy = Strategy::TemplateScript;
                let poll = Arc::clone(&self.polls);
                let (sender, receiver) = mpsc::channel();
                let _ = thread::spawn(move || -> Result<(), Error> {
                    let res = Template::parse(&t, poll);
                    sender.send(res).expect("send template error");
                    Ok(())
                });
                let _ = receiver.recv().map_or_else(
                    |e| Err(Error::from(format!("receive template error: {}", e))),
                    |t| {
                        self.template.replace(t?);
                        Ok(())
                    },
                )?;
                self.script.replace(sc.into());
            }
            (Some(t), None, None) => {
                self.strategy = Strategy::SingleTemplate;
                self.template
                    .replace(Template::parse(&t, Arc::clone(&self.polls))?);
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
