#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ResultType {
    /// 无需处理，需要直接放到bridge的others中
    Ignore,
    /// 已经处理
    Handled,
}

impl ResultType {
    pub fn is_ignore_then<F, R>(&self, default: R, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        if let ResultType::Ignore = self {
            return f();
        } else {
            default
        }
    }
    pub fn is_ignore(&self) -> bool{
        matches!(self, ResultType::Ignore)
    }
    // pub fn is_fn_then<F, R>(&self, default: R, f: F) -> R
    // where
    //     F: FnOnce() -> R,
    // {
    //     if let ResultType::Fn = self {
    //         return f();
    //     } else {
    //         default
    //     }
    // }
}
