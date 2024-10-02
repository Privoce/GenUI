pub mod event;
pub mod page;
mod register;
pub mod types;

use crate::{components::view::GViewWidgetExt, utils::HeapLiveIdPathExp};
use event::GRouterEvent;
use makepad_widgets::*;
use page::GPageWidgetRefExt;
pub use register::register;
use types::{PageType, RouterStack, RouterStackItem};

use super::view::{GView, GViewWidgetRefExt};

live_design! {
    GRouterBase = {{GRouter}}{}
}

#[derive(Live, Widget)]
pub struct GRouter {
    #[deref]
    pub deref_widget: GView,
    #[rust(id!(bar_pages)[0])]
    pub active_router: LiveId,
    #[rust(id!(tabbar)[0])]
    pub bar_id: LiveId,
    #[rust]
    pub active_page: Option<HeapLiveIdPath>,
    #[rust]
    pub stack: RouterStack,
    #[rust]
    pub bar_pages: Vec<HeapLiveIdPath>,
    #[rust]
    pub nav_pages: Vec<HeapLiveIdPath>,
    #[rust]
    pub page_type: PageType,
}

impl LiveHook for GRouter {}

impl Widget for GRouter {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.scope_path = scope.path.clone();
        match self.page_type {
            PageType::Bar | PageType::Nav => self
                .widget(&[self.active_router])
                .draw_walk(cx, scope, walk),

            PageType::None => DrawStep::done(),
        }
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.deref_widget.handle_event(cx, event, scope);
    }
}

impl GRouter {
    pub fn handle_nav_back(&mut self, cx: &mut Cx, actions: &Actions) {
        for action in actions {
            if let GRouterEvent::NavBack(_current) = action.as_widget_action().cast() {
                // get last item from stack
                self.stack.pop().map(|last| {
                    self.nav_to_path(cx, &last.path);
                });

                break;
            }
        }
    }
    pub fn set_visible_page(&mut self, cx: &mut Cx, target: &HeapLiveIdPath) {
        // first check route
        self.page_type = self.check_route(target);
        self.active_router = self.page_type.live_id();
        // dbg!(self.page_type, self.active_router, target);

        self.gview(&[self.active_router])
            .borrow()
            .map(|active_router| match self.page_type {
                PageType::Bar => {
                    let bar_pages = active_router.children.iter().zip(self.bar_pages.iter());
                    for ((_id, child), path) in bar_pages {
                        child.as_gview().borrow_mut().map(|mut child| {
                            if path.eq(target) {
                                child.visible = true;
                            } else {
                                child.visible = false;
                            }
                            child.render(cx);
                        });
                    }
                }
                PageType::Nav => {
                    let nav_pages = active_router.children.iter().zip(self.nav_pages.iter());
                    for ((_id, child), path) in nav_pages {
                        child.as_gpage().borrow_mut().map(|mut child| {
                            if path.eq(target) {
                                child.visible = true;
                            } else {
                                child.visible = false;
                            }
                            child.render(cx);
                        });
                    }
                }
                PageType::None => {}
            });

        // after all change active_page
        self.active_page.replace(target.clone());
    }
    fn get_visible_page(&self) -> Option<HeapLiveIdPath> {
        // find the visible page
        if let Some(active_router) = self.gview(&[self.active_router]).borrow() {
            let mut res = None;
            for (id, child) in active_router.children.iter() {
                if child.is_visible() && id != &self.bar_id {
                    let mut p = self.scope_path.clone();
                    p.push(*id);
                    res.replace(p);
                    break;
                }
            }
            res
        } else {
            None
        }
    }
    pub fn nav_to(&mut self, cx: &mut Cx, path: &[LiveId]) {
        let path = self.bar_scope_path(path);
        self.active_page.as_ref().map(|path| {
            // push stack
            self.stack.push(RouterStackItem {
                path: path.clone(),
                ty: self.page_type,
            });
        });
        self.set_visible_page(cx, &path);
    }
    fn nav_to_path(&mut self, cx: &mut Cx, path: &HeapLiveIdPath) {
        self.active_page.as_ref().map(|path| {
            // push stack
            self.stack.push(RouterStackItem {
                path: path.clone(),
                ty: self.page_type,
            });
        });
        self.set_visible_page(cx, path);
    }
    pub fn check_route(&mut self, path: &HeapLiveIdPath) -> PageType {
        if !self.bar_pages.iter().any(|x| x.contains(path).unwrap()) {
            if self.nav_pages.iter().any(|x| x.contains(path).unwrap()) {
                PageType::Nav
            } else {
                panic!("unregister page path!: {:?}", path);
            }
        } else {
            PageType::Bar
        }
    }
    pub fn check_route_live_id(&mut self, path: &LiveId) -> PageType {
        if !self.bar_pages.iter().any(|x| x.contains_id(path)) {
            if self.nav_pages.iter().any(|x| x.contains_id(path)) {
                PageType::Nav
                // self.active_router = id!(nav_pages)[0].clone();
            } else {
                panic!("unregister page path!: {:?}", path);
            }
        } else {
            PageType::Bar
        }
    }
    pub fn bar_scope_path(&self, child: &[LiveId]) -> HeapLiveIdPath {
        let mut path = self.scope_path.clone();
        child.into_iter().for_each(|x| {
            path.push(*x);
        });
        path
    }
    pub fn nav_scope_path(&self, child: &[LiveId]) -> HeapLiveIdPath {
        let mut path = self.scope_path.clone();
        child.into_iter().for_each(|x| {
            // path.push(id!(nav_pages)[0]);
            path.push(*x);
        });
        path
    }
    /// ## Init Router
    /// This fn help you init a router by setting bar_pages and nav_pages
    /// ### Example (in `draw_walk()`)
    /// ```rust
    /// self.lifetime
    /// .init()
    /// .execute(|| {
    ///     let router = self.grouter(id!(app_router));
    ///
    ///     router.borrow_mut().map(|mut router| {
    ///         let _ = router.init(ids!(page1, page2, page3), Some(ids!(nav_page1)));
    ///     });
    /// })
    /// .map(|_| {
    ///     let router = self.grouter(id!(app_router));
    ///     router.borrow().map(|router| {
    ///         if !router.scope_path.is_empty() {
    ///             // if is empty do not do next
    ///             self.lifetime.next();
    ///         }
    ///     })
    /// });
    /// ```
    pub fn init(
        &mut self,
        bar_pages: &[&[LiveId]],
        nav_pages: Option<&[&[LiveId]]>,
        tabbar_id: Option<&[LiveId]>,
    ) -> &mut Self {
        if !self.scope_path.is_empty() {
            self.nav_pages.clear();
            self.bar_pages.clear();
            bar_pages.iter().for_each(|x| {
                let bar_path = self.bar_scope_path(x);
                self.bar_pages.push(bar_path);
            });
            nav_pages.map(|x| {
                x.iter().for_each(|x| {
                    let nav_path = self.nav_scope_path(x);
                    self.nav_pages.push(nav_path);
                });
            });
            tabbar_id.map(|bar_id| {
                self.bar_id = bar_id[0].clone();
            });
            self.after_init_check();
        }
        self
    }
    /// ## Auto init Router by inner children (bar_pages, nav_pages)
    /// this fn consider bar_id is tabbar(it will never change)
    pub fn init_auto(&mut self) -> &mut Self {
        // do loop to get need children
        if !self.scope_path.is_empty() {
            self.nav_pages.clear();
            self.bar_pages.clear();
            let mut flag = true; // let it do only once
            self.gview(id!(bar_pages)).borrow().map(|bar| {
                for (id, child) in bar.children.iter() {
                    if id != &self.bar_id {
                        let bar_path = self.bar_scope_path(&[id.clone()]);
                        if child.is_visible() && flag {
                            self.ty(PageType::Bar);
                            self.active_page.replace(bar_path.clone());
                            flag = false;
                        }
                        self.bar_pages.push(bar_path);
                    }
                }
            });
            self.gview(id!(nav_pages)).borrow().map(|nav| {
                for (id, child) in nav.children.iter() {
                    let nav_path = self.nav_scope_path(&[id.clone()]);
                    if child.is_visible() && flag {
                        self.ty(PageType::Nav);
                        self.active_page.replace(nav_path.clone());
                        flag = false;
                    }
                    self.nav_pages.push(nav_path);
                }
            });
        }
        self
    }
    /// ## check router page type and active page
    fn after_init_check(&mut self) -> () {
        // let it do only once
        let mut flag = true;
        // loop bar_pages and nav_pages
        for bar in self.bar_pages.clone().iter() {
            if flag {
                self.gview(id!(bar_pages)).borrow().map(|container| {
                    if container.widget(&[bar.last()]).is_visible() {
                        self.ty(PageType::Bar);
                        self.active_page.replace(bar.clone());
                        flag = false;
                    }
                });
            } else {
                break;
            }
        }
        for nav in self.nav_pages.clone().iter() {
            if flag {
                self.gview(id!(nav_pages)).borrow().map(|container| {
                    if container.widget(&[nav.last()]).is_visible() {
                        self.ty(PageType::Nav);
                        self.active_page.replace(nav.clone());
                    }
                    flag = false;
                });
            } else {
                break;
            }
        }
    }
    /// ## Set default active page
    /// set page as active page, you can use this if you need to control
    pub fn active(&mut self, id: &[LiveId]) -> &mut Self {
        // if scope is empty, do nothing
        if !self.scope_path.is_empty() {
            let mut path = self.scope_path.clone();
            path.push(id[0].clone());
            self.active_page.replace(path);
        }
        self
    }
    /// ## Finish Router Build
    pub fn build(&mut self, cx: &mut Cx) -> () {
        if self.active_page.as_ref().is_none() {
            // do get_visible_page and set as active_page
            self.get_visible_page()
                .map(|page| self.active_page.replace(page));
        } else {
            // do set visible page
            let active = self.active_page.clone().unwrap();
            let _ = self.set_visible_page(cx, &active);
        }
    }
    pub fn ty(&mut self, ty: PageType) -> &mut Self {
        self.page_type = ty;
        self
    }
}
