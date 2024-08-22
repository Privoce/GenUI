use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IFSignal {
    /// if
    If,
    /// else if
    ElseIf,
    /// else
    Else,
}

impl IFSignal {
    pub fn new() -> Self {
        IFSignal::If
    }
    pub fn next(&mut self) {
        *self = match self {
            IFSignal::If => IFSignal::ElseIf,
            IFSignal::ElseIf => IFSignal::Else,
            IFSignal::Else => IFSignal::If,
        }
    }
}

impl From<&str> for IFSignal {
    fn from(signal: &str) -> Self {
        match signal {
            "if" => IFSignal::If,
            "else_if" => IFSignal::ElseIf,
            "else" => IFSignal::Else,
            _ => panic!("signal not found!"),
        }
    }
}

impl Display for IFSignal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            IFSignal::If => "if",
            IFSignal::ElseIf => "else_if",
            IFSignal::Else => "else",
        };
        f.write_str(res)
    }
}