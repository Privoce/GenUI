use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum CompilerError {
    /// Compiler Env Check Error
    EnvCheck {
        env: String,
        recommend: String,
        other: Option<String>,
    },
    /// Compiler Runtime Error
    Runtime {
        target: String,
        msg: String,
    },
    Conf(String),
}

impl CompilerError {
    pub fn env_check(env: &str, recommend: &str, other: Option<&str>) -> Self {
        CompilerError::EnvCheck {
            env: env.to_string(),
            recommend: recommend.to_string(),
            other: other.map(|s| s.to_string()),
        }
    }
    pub fn runtime(target: &str, msg: &str) -> Self {
        CompilerError::Runtime {
            target: target.to_string(),
            msg: msg.to_string(),
        }
    }
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilerError::EnvCheck {
                env,
                recommend,
                other,
            } => {
                let _ = f.write_fmt(format_args!(
                    "Env Check Error: {env}\nRecommend Message:\n{}",
                    recommend
                ));
                if let Some(other) = other {
                    let _ = f.write_fmt(format_args!("\nOther Message:\n{}", other));
                }
                Ok(())
            }
            CompilerError::Runtime { target, msg } => {
                f.write_fmt(format_args!("Runtime Error: {target}\nMessage:\n{}", msg))
            }
            CompilerError::Conf(msg) => f.write_str(msg),
        }
    }
}