pub mod event;
pub mod page;
mod register;
pub mod types;

use crate::{components::view::GViewWidgetExt, utils::HeapLiveIdPathExp};
use event::GRouterEvent;
use makepad_widgets::*;
use page::GPageWidgetRefExt;
pub use register::register;
use types::PageType;

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
    // #[rust]
    // register_routers: Option<Vec<LiveId>>
    #[rust]
    pub pre_router: Option<Vec<LiveId>>,
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
                dbg!("do back");
                self.pre_router.as_ref().map(|pre| {
                    // show pre
                    self.gview(pre.as_slice()).set_visible(true);
                });
                // scope.path.trim_matches(&current)
                self.pre_router.replace(current.trim_matches(&scope.path));
                break;
            }
            if let GRouterEvent::NavTo(to_path) = action.as_widget_action().cast() {
                dbg!("nav to");
                let to_path = to_path.trim_matches(&scope.path);
                self.gview(&to_path.as_slice())
                    .set_visible_and_redraw(cx, true);
                break;
            }
        }
    }
}

impl GRouter {
    pub fn set_visible(&mut self, cx: &mut Cx, target: &HeapLiveIdPath) {
        // first check route
        let _ = self.check_route(target);
        dbg!(self.page_type, &self.active_router);
        self.gview(&[self.active_router])
            .borrow_mut()
            .map(|mut active_router| match self.page_type {
                PageType::Bar => {
                    let mut bar_pages = active_router.children.iter().zip(self.bar_pages.iter());
                    for ((id, child), path) in bar_pages {
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
                    for ((id, child), path) in nav_pages {
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
    pub fn nav_to(&mut self, cx: &mut Cx, path: &HeapLiveIdPath) {
        // let to_path = path.trim_matches(&self.scope_path);
        self.set_visible(cx, path);
        // dbg!(to_path);
    }
    pub fn check_route(&mut self, path: &HeapLiveIdPath) {
        if !self.bar_pages.iter().any(|x| x.contains(path)) {
            if self.nav_pages.iter().any(|x| x.contains(path)) {
                self.page_type = PageType::Nav;
                self.active_router = id!(nav_pages)[0].clone();
            } else {
                panic!("unregister page path!: {:?}", path)
            }
        } else {
            self.page_type = PageType::Bar;
            self.active_router = id!(bar_pages)[0].clone();
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
