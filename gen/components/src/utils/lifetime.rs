/// ## Example
/// ```
/// #[derive(Live, Widget)]
/// pub struct SiginPage {
///     #[deref]
///     pub super_widget: GView,
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
    pub fn next(&mut self) -> () {
        let _ = std::mem::replace(
            self,
            match self {
                Lifetime::Init => Lifetime::InProcess,
                Lifetime::InProcess => Lifetime::Destroy,
                Lifetime::Destroy => Lifetime::Init,
            },
        );
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
    fn execute<F>(&self, f: F) -> Option<()>
    where
        F: FnOnce();
}

pub struct LifetimeExecutor {
    current: Lifetime,
    target: Lifetime,
}
impl Executor for LifetimeExecutor {
    fn execute<F>(&self, f: F) -> Option<()>
    where
        F: FnOnce(),
    {
        if self.current == self.target {
            f();
            Some(())
        } else {
            None
        }
    }
}
