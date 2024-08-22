use std::fmt::Display;

use gen_parser::Value;
use gen_utils::{error::Errors, props_manul};

use crate::widget::{utils::{bool_prop, string_prop}, BuiltIn};

use super::{play::Play, Ease};

#[derive(Debug, Clone, Default)]
pub struct AnimationItem {
    name: String,
    default: bool,
    option: ItemOption,
}

impl Display for AnimationItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let default_str = if self.default {
            "default: on"
        } else {
            "default: off"
        };
        let off_str = self.option.clone().fmt_off();
        let option_str = format!("on = {{{}}}, off = {{{}}}", self.option, off_str);

        f.write_fmt(format_args!(
            "{} = {{{}, {}}},",
            self.name, default_str, option_str
        ))
    }
}

impl TryFrom<(&str, &Value, BuiltIn)> for AnimationItem {
    type Error = Errors;

    fn try_from(value: (&str, &Value, BuiltIn)) -> Result<Self, Self::Error> {
        let mut anim = AnimationItem::default();
        anim.name = value.0.to_string();
        anim.option.apply = value.0.to_string();
        anim.option.targets = value.2.animation_applys().iter().map(|s| s.to_string()).collect();
        if let Some(an) = value.1.is_animation_and_get() {
            for (key, value) in an {
                match key.name() {
                    props_manul::Animation::DEFAULT => {
                        let _ = bool_prop(value, |b| {
                            anim.default = b;
                        });
                    }
                    props_manul::Animation::EASE => {
                        anim.option.ease = value.try_into()?;
                    }
                    props_manul::Animation::REDRAW => {
                        let _ = bool_prop(value, |b| {
                            anim.option.redraw = Some(b);
                        });
                    }
                    props_manul::Animation::PLAY => {
                        anim.option.play = value.try_into()?;
                    }
                    props_manul::Animation::FROM => {
                        anim.option.from = value.try_into()?;
                    }
                    props_manul::Animation::TARGET => {
                        let _ = string_prop(value, |s| {
                            anim.option.targets.push(
                                s.split_whitespace()
                                    .into_iter()
                                    .map(|s| s.to_string())
                                    .collect(),
                            );
                        });
                    }
                    _ => {
                        return Err(Errors::PropConvertFail(format!(
                            "{} can not convert to Animation",
                            value
                        )));
                    }
                }
            }

            return Ok(anim);
        }
        Err(Errors::PropConvertFail(format!(
            "{} can not convert to Animation",
            value.0
        )))
    }
}

#[derive(Debug, Clone)]
pub struct ItemOption {
    // apply is from AnimationItem's name
    apply: String,
    targets: Vec<String>,
    on_off: bool,
    from: OptionFrom,
    play: Play,
    redraw: Option<bool>,
    ease: Ease,
}

impl ItemOption {
    pub fn is_on(&self) -> bool {
        self.on_off
    }
    pub fn fmt_off(&mut self) -> String {
        self.on_off = false;
        self.to_string()
    }
}

impl Default for ItemOption {
    fn default() -> Self {
        Self {
            apply: Default::default(),
            targets: Default::default(),
            on_off: true,
            from: Default::default(),
            play: Default::default(),
            redraw: Default::default(),
            ease: Default::default(),
        }
    }
}

impl Display for ItemOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn format_targets(targets: &Vec<String>, apply: &str, num: u8) -> String {
            targets
                .iter()
                .map(|item| format!("{}: {{{}: {}.0}}", item, apply, num))
                .collect::<Vec<String>>()
                .join(", ")
        }

        let _ = f.write_fmt(format_args!("from: {{{}: {}}},", self.from, self.play));

        if let Some(redraw) = self.redraw {
            let _ = f.write_fmt(format_args!("redraw: {},", redraw));
        }

        let _ = f.write_fmt(format_args!("ease: {},", self.ease));

        if self.on_off {
            f.write_fmt(format_args!(
                "apply: {{{}}}",
                format_targets(&self.targets, &self.apply, 1)
            ))
        } else {
            f.write_fmt(format_args!(
                "apply: {{{}}}",
                format_targets(&self.targets, &self.apply, 0)
            ))
        }
    }
}

#[derive(Debug, Clone)]
pub enum OptionFrom {
    All,
    Other(String),
}

impl Default for OptionFrom {
    fn default() -> Self {
        Self::All
    }
}

impl Display for OptionFrom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptionFrom::All => f.write_str("All"),
            OptionFrom::Other(s) => f.write_str(s),
        }
    }
}

impl TryFrom<&Value> for OptionFrom {
    type Error = Errors;
    #[allow(unused_assignments)]
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let mut from = Err(Errors::PropConvertFail(format!(
            "{} can not convert to OptionFrom",
            value
        )));

        let _ = string_prop(value, |s| {
            if s == "all" {
                from = Ok(OptionFrom::All);
            }
            from = Ok(OptionFrom::Other(s.to_string()));
        });

        from
    }
}
