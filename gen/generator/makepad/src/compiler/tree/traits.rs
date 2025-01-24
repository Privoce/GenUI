use std::path::Path;

pub trait TreePathExt {
    fn level(&self) -> usize;
}

impl<P> TreePathExt for P where P: AsRef<Path> {
    fn level(&self) -> usize {
        self.as_ref().components().count()
    }
}