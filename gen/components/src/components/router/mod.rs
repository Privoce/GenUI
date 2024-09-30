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
    #[rust]
    screen_width: f64,
    #[rust(id!(bar_pages)[0])]
    pub active_router: LiveId,
    #[rust]
    pub stack: RouterStack,
    #[rust]
    bar_pages: Vec<HeapLiveIdPath>,
    #[rust]
    nav_pages: Vec<HeapLiveIdPath>,
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
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        for action in actions {
            if let GRouterEvent::NavBack(current) = action.as_widget_action().cast() {
                // get last item from stack
                self.stack.pop().map(|last| {
                    // if exist back to last and then push current (before do check)
                    let ty = self.check_route_live_id(&current.last());

                    self.nav_to_path(cx, &last.path);
                    let mut path = self.scope_path.clone();
                    // path.push(current.last());
                    // self.stack.push(RouterStackItem { path, ty });
                    // dbg!(&self.stack);
                });

                break;
            }
            // if let GRouterEvent::NavTo(to_path) = action.as_widget_action().cast() {
            //     let to_path = to_path.trim_matches(&scope.path);
            //     self.gview(&to_path.as_slice())
            //         .set_visible_and_redraw(cx, true);
            //     break;
            // }
        }
    }
}

impl GRouter {
    pub fn set_visible(&mut self, cx: &mut Cx, target: &HeapLiveIdPath) {
        // first check route
        self.page_type = self.check_route(target);
        self.active_router = self.page_type.live_id();
        // dbg!(self.page_type, self.active_router, target);

        self.gview(&[self.active_router])
            .borrow()
            .map(|active_router| match self.page_type {
                PageType::Bar => {
                    let mut bar_pages = active_router.children.iter().zip(self.bar_pages.iter());
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
                    let mut nav_pages = active_router.children.iter().zip(self.nav_pages.iter());
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
    }
    fn get_visible(&self) -> Option<HeapLiveIdPath> {
        // find the visible page
        if let Some(active_router) = self.gview(&[self.active_router]).borrow() {
            let mut res = None;
            for (id, child) in active_router.children.iter() {
                if child.is_visible() {
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
        self.get_visible().map(|path| {
            // push stack
            self.stack.push(RouterStackItem {
                path,
                ty: self.page_type,
            });
        });

        self.set_visible(cx, &path);
    }
    fn nav_to_path(&mut self, cx: &mut Cx, path: &HeapLiveIdPath) {
        self.get_visible().map(|path| {
            // push stack
            self.stack.push(RouterStackItem {
                path,
                ty: self.page_type,
            });
        });

        self.set_visible(cx, path);
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
    pub fn init(&mut self, bar_pages: &[&[LiveId]], nav_pages: Option<&[&[LiveId]]>) -> &mut Self {
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
        self
    }
    pub fn ty(&mut self, ty: PageType) {
        self.page_type = ty;
    }
}
