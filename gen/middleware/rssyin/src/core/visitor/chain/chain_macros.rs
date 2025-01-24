#[macro_export]
macro_rules! impl_basic_visitor {
    ($Visitor: ty) => {
        impl BasicVisitor for $Visitor {
            fn new() -> Self
            where
                Self: Sized,
            {
                Self::default()
            }

            fn is_worked(&self) -> bool {
               false
            }
            fn reset(&mut self) {
                for visitor in self.visitors.iter_mut() {
                    visitor.reset();
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_basic_chain_visitor {
    ($VisitorChain: ty : $Visitor: ty) => {
        impl BasicChainVisitor for $VisitorChain {
            type Visitor = $Visitor;

            fn push(&mut self, visitor: Self::Visitor) {
                self.visitors.push(visitor);
            }

            fn is_empty(&self) -> bool {
                self.visitors.is_empty()
            }

            fn get(&self, index: usize) -> &Self::Visitor {
                &self.visitors[index]
            }

            fn get_mut(&mut self, index: usize) -> &mut Self::Visitor {
                &mut self.visitors[index]
            }
        }
    };
}
