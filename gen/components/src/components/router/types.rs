use makepad_widgets::{id, HeapLiveIdPath, LiveId};

#[derive(Default, Debug)]
pub enum ActiveRouter {
    #[default]
    None,
    Active(LiveId),
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum PageType {
    #[default]
    Bar,
    Nav,
    /// no default display page
    None,
}

impl PageType {
    pub fn live_id(&self) -> LiveId {
        match self {
            PageType::Bar => id!(bar_pages)[0].clone(),
            PageType::Nav => id!(nav_pages)[0].clone(),
            PageType::None => id!(nav_pages)[0].clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RouterStackItem {
    pub path: HeapLiveIdPath,
    pub ty: PageType,
}

#[derive(Clone, Debug, Default)]
pub struct RouterStack(pub Vec<RouterStackItem>);

impl RouterStack {
    /// check item is bar or not
    /// - true: do clear to clean the stack and set current as first
    /// - false: push
    pub fn check(item: &RouterStackItem) -> bool {
        PageType::Bar == item.ty
    }
    pub fn clear(&mut self) -> () {
        self.0.clear();
    }
    pub fn push(&mut self, item: RouterStackItem) {
        if RouterStack::check(&item) {
            self.clear();
        }

        self.0.push(item);
    }
    pub fn pop(&mut self) -> Option<RouterStackItem> {
        self.0.pop()
    }
    pub fn rev(&self) -> Vec<RouterStackItem> {
        let mut res = self.0.clone();
        res.reverse();
        res
    }
    pub fn first(&self) -> Option<&RouterStackItem> {
        self.0.first()
    }
    pub fn last(&self) -> Option<&RouterStackItem> {
        self.0.last()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
