/// ## Example
/// ```
/// #[derive(Live, Widget)]
/// pub struct SiginPage {
///     #[deref]
///     pub super_widget: GCard,
///     #[rust]
///     pub lifetime: Lifetime,
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Lifetime {
    Init,
    InProcess,
    Destroy,
}

impl Default for Lifetime {
    fn default() -> Self {
        Lifetime::Init
    }
}

impl Lifetime {
    pub fn next(&self) -> Self {
        match self {
            Lifetime::Init => Lifetime::InProcess,
            Lifetime::InProcess => Lifetime::Destroy,
            Lifetime::Destroy => Lifetime::Init,
        }
    }
    /// ```
    /// fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
    ///     let _ = self.super_widget.draw_walk(cx, scope, walk);
    ///     self.lifetime
    ///         .init()
    ///         .execute(|| self.get(cx))
    ///         .map(|lifetime| {
    ///             self.lifetime = lifetime;
    ///         });
    ///
    ///     DrawStep::done()
    /// }
    /// ```
    pub fn init(&self) -> LifetimeExecutor {
        LifetimeExecutor {
            current: *self,
            target: Lifetime::Init,
        }
    }
    pub fn in_process(&self) -> LifetimeExecutor {
        LifetimeExecutor {
            current: *self,
            target: Lifetime::InProcess,
        }
    }
    pub fn destroy(&self) -> LifetimeExecutor {
        LifetimeExecutor {
            current: *self,
            target: Lifetime::Destroy,
        }
    }
}

pub trait Executor {
    type Item;
    fn execute<F>(&self, f: F) -> Option<Self::Item>
    where
        F: FnOnce();
}

pub struct LifetimeExecutor {
    current: Lifetime,
    target: Lifetime,
}
impl Executor for LifetimeExecutor {
    type Item = Lifetime;

    fn execute<F>(&self, f: F) -> Option<Self::Item>
    where
        F: FnOnce(),
    {
        if self.current == self.target {
            f();
            //back next
            Some(self.current.next())
        } else {
            None
        }
    }
}
