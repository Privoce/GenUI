use event::EventVisitor;
use fn_closure::{ClosureVisitor, FnVisitor};
use imports::ImportVisitor;
use instance::InstanceVisitor;

use prop::PropVisitor;

use super::visitor::chain::{
    traits::{BasicChainVisitor, BasicVisitor, ChainVisitor},
    VisitorChain,
};

pub mod event;
pub mod fn_closure;
pub mod imports;
pub mod instance;
pub mod prop;

pub trait MakepadChainExpand {
    fn build() -> Self
    where
        Self: Sized;
}

impl MakepadChainExpand for VisitorChain {
    fn build() -> Self
    where
        Self: Sized,
    {
        let mut chain = VisitorChain::new();
        // macro -----------------------------------------------------------------
        //      | import ---------------------------------------------------------
        chain.macros.push(Box::new(ImportVisitor::new()));
        // item ------------------------------------------------------------------
        //     | prop ------------------------------------------------------------
        chain.items.structs.push(Box::new(PropVisitor::new()));
        //     | event -----------------------------------------------------------
        chain.items.enums.push(Box::new(EventVisitor::new()));
        //     | fn --------------------------------------------------------------
        chain.items.fns.push(Box::new(FnVisitor::new()));
      
        // 废弃 -----------------------------------------------------------------
        // local -----------------------------------------------------------------
        // //     | instance --------------------------------------------------------
        // chain.locals.push(Box::new(InstanceVisitor::new()));
        //     | closure ---------------------------------------------------------
        chain.locals.push(Box::new(ClosureVisitor::new()));
        chain
    }
}
