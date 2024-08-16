use syn::{token::Brace, Block, ExprBlock};

pub trait SynEmpty {
    fn empty() -> Self;
}

impl SynEmpty for Block{
    fn empty() -> Self {
        Block {
            brace_token: Brace::default(),
            stmts: Default::default(),
        }
    }
}

impl SynEmpty for ExprBlock {
    fn empty() -> Self {
        ExprBlock {
            attrs: Default::default(),
            label: Default::default(),
            block: Block::empty(),
        }
    }
}