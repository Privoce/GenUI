use gen_components::components::{
    button::GButtonWidgetExt,
    router::{event::GRouterEvent, page::GPageWidgetExt, GRouterWidgetExt},
    view::{GView, GViewWidgetExt},
};
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;
    TPage = {{TPage}}{
        height: Fill,
        width: Fill,
        flow: Down,
        menu = <GView>{
            height: 36.0,
            width: Fill,
            spacing: 8.0,
            to_a = <GButton>{
                slot: {
                    text: "a"
                }
            }
            to_b =<GButton>{
                slot: {
                    text: "b"
                }
            }
            to_c =<GButton>{
                slot: {
                    text: "c"
                }
            }
            to_d = <GButton>{
                slot: {
                    text: "d"
                }
            }
        }
        app_router = <GRouter>{
            bar_pages = <GView>{
                height: Fill,
                width: Fill,
                border_radius: 0.0,
                // flow: Overlay,
                background_visible: false,
                page1 = <GView>{
                    visible: true,
                    height: Fill,
                    width: Fill,
                    theme: Warning,
                    border_radius: 0.0,
                    <GLabel>{
                        text: "APP PAGE1"
                    }
                },
                page2 = <GView>{
                    visible: false,
                    height: Fill,
                    width: Fill,
                    theme: Error,
                    border_radius: 0.0,
                    <GLabel>{
                        text: "APP PAGE2"
                    }
                },
                page3 = <GView>{
                    visible: false,
                    height: Fill,
                    width: Fill,
                    theme: Success,
                    border_radius: 0.0,
                    <GLabel>{
                        text: "APP PAGE3"
                    }
                }
            }
            nav_pages = <GView>{
                height: Fill,
                width: Fill,
                border_radius: 0.0,
                // flow: Overlay,
                background_visible: false,
                nav_page1 = <GPage>{
                    visible: false,
                    height: Fill,
                    width: Fill,
                    border_radius: 0.0,
                    header = {
                        title_wrap = {
                            title = {
                                text: "Page1"
                            }
                        }
                        tool_wrap = {
                            <GIcon>{
                                theme: Dark,
                                icon_type: OpenBottom,
                                stroke_width: 1.2
                            }
                        }
                    }
                    body = {
                        theme: Warning,
                        <GLabel>{
                            text: "APP PAGE1"
                        }
                        <GButton>{}
                    }
                },
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct TPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for TPage {}

impl Widget for TPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        let router = self.grouter(id!(app_router));

        router.borrow_mut().map(|mut router| {
            let _ = router.init(ids!(page1, page2, page3), Some(
                ids!(nav_page1)
            ));
        });

        if self.gbutton(id!(to_a)).clicked(&actions).is_some() {
            router.borrow_mut().map(|mut x| {
                let page1_path = x.bar_scope_path(id!(page1));
                x.nav_to(cx, &page1_path);
            });
        }
        if self.gbutton(id!(to_b)).clicked(&actions).is_some() {
            router.borrow_mut().map(|mut x| {
                let page2_path = x.bar_scope_path(id!(page2));
                x.nav_to(cx, &page2_path);
            });
        }
        if self.gbutton(id!(to_c)).clicked(&actions).is_some() {
            router.borrow_mut().map(|mut x| {
                let page3_path = x.bar_scope_path(id!(page3));
                x.nav_to(cx, &page3_path);
            });
        }
        if self.gbutton(id!(to_d)).clicked(&actions).is_some() {
            router.borrow_mut().map(|mut x| {
                let nav1 = x.nav_scope_path(id!(nav_page1));
                x.nav_to(cx, &nav1);
            });
        }
        // if !actions.is_empty(){
        //     dbg!(&actions);
        // }
        router.handle_event(cx, event, scope);
    }
}
