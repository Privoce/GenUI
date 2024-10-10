use gen_components::{
    components::{menu::GMenuWidgetExt, router::GRouterWidgetExt, view::GView},
    shader::manual::RouterIndicatorMode,
    utils::{
        lifetime::{Executor, Lifetime},
        HeapLiveIdPathExp,
    },
};
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;
    import crate::views::overall::all::*;
    import crate::views::overall::color::*;
    import crate::views::overall::font::*;
    import crate::views::guide::install::*;
    import crate::views::guide::quickstart::*;
    import crate::views::guide::start::*;
    BOLD_FONT = dep("crate://self/resources/OPPOSans-Bold.ttf");
    AppMainPage = {{AppMainPage}}{
        height: Fill,
        width: Fill,
        flow: Right,
        background_visible: true,
        background_color: #2C313A,
        border_radius: 0.0,
        menu = <GMenu>{
            padding: 8.0,
            width: 180.0,
            header: {
                height: 32.0,
                width: Fill,
                visible: true,
                align: {
                    x: 0.0,
                    y: 0.5
                },
                padding: {left: 4.0},
                <GLabel>{
                    font_family: (BOLD_FONT),
                    font_size: 12.0,
                    text: "GenUI Components",
                }
            }
            body: {

                sub1 = <GSubMenu>{
                    title: {
                        <GLabel>{
                            font_size: 11.0,
                            text: "Guide",
                        }
                    }
                    items: {
                        tab_get_start = <GMenuItem>{
                            selected: true,
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Getting Started",
                            }
                        }
                        tab_install = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Install",
                            }
                        }
                        tab_qs = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "QuickStart",
                            }
                        }
                    }
                }
                sub2 = <GSubMenu>{
                    title: {
                        <GLabel>{
                            font_size: 11.0,
                            text: "Overall",
                        }
                    }
                    items: {
                        tab_overall = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Overall",
                            }
                        }
                        tab_font = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "System Font",
                            }
                        }
                        tab_color = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Theme Color",
                            }
                        }
                    }
                }
                sub3 = <GSubMenu>{
                    title: {
                        <GLabel>{
                            font_size: 11.0,
                            text: "Basic",
                        }
                    }
                    items: {
                        <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Label",
                            }
                        }
                        <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "View",
                            }
                        }
                        <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Button",
                            }
                        }
                        <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Svg",
                            }
                        }
                        <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Image",
                            }
                        }
                        <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Icon Lib",
                            }
                        }
                    }
                }
            }
        }
        <GVLayout>{
            app_router = <GRouter>{
                bar_pages = {
                    flow: Down,
                    start_page = <GBarPage>{
                        <StartPage>{}
                    }
                    install_page = <GBarPage>{
                        <InstallPage>{}
                    }
                    qs_page = <GBarPage>{
                        <QSPage>{}
                    }
                    overall_page = <GBarPage>{
                        <OverallPage>{}
                    }
                    font_page = <GBarPage>{
                        <FontPage>{}
                    }
                    color_page = <GBarPage>{
                        <ColorPage>{}
                    }
                }
            }
            <GView>{
                width: Fill,
                height: 100.0,
                spacing: 12.0,
                padding: 16.0,
                border_radius: 0.0,
                <GVLayout>{
                    spacing: 4.0,
                    <GLabel>{
                        text: "Links"
                    }
                    <GLink>{
                        text: "GenUI Github",
                        href: "https://github.com/Privoce/GenUI"
                    }
                    <GLink>{
                        text: "Makepad Github",
                        href: "https://github.com/makepad/makepad"
                    }
                    <GLink>{
                        text: "Update Log"
                    }
                }
                <GVLayout>{
                    spacing: 4.0,
                    <GLabel>{
                        text: "Community"
                    }
                    <GLink>{
                        text: "GenUI Discord",
                        href: "https://discord.gg/jVEJDhE75Y"
                    }
                    <GLink>{
                        text: "Makepad Discord",
                        href: "https://discord.gg/adqBRq7Ece"
                    }
                }
                <GVLayout>{
                    spacing: 4.0,
                    <GLabel>{
                        text: "Videos"
                    }
                    <GLink>{
                        text: "GenUI",
                        href: "https://www.bilibili.com/video/BV1PYsbe3EbW/?spm_id_from=333.337.search-card.all.click"
                    }
                    <GLink>{
                        text: "Makepad",
                        href: "https://www.youtube.com/watch?v=rC4FCS-oMpg"
                    }
                }
            }
        }

    }
}

#[derive(Live, Widget)]
pub struct AppMainPage {
    #[deref]
    pub deref_widget: GView,
    #[rust]
    lifetime: Lifetime,
}

impl LiveHook for AppMainPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for AppMainPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);
        self.lifetime
            .init()
            .execute(|| {
                let router = self.grouter(id!(app_router));

                router.borrow_mut().map(|mut router| {
                    let _ = router
                        .init(
                            ids!(start_page, overall_page, color_page, font_page, install_page, qs_page),
                            None,
                            Some(RouterIndicatorMode::Define),
                        )
                        .active(id!(install_page))
                        .build(cx);
                });
            })
            .map(|_| {
                let router = self.grouter(id!(app_router));
                router.borrow().map(|router| {
                    if !router.scope_path.is_empty() {
                        // if is empty do not do next
                        self.lifetime.next();
                    }
                })
            });
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        let router = self.grouter(id!(app_router));

        if let Some(e) = self.gmenu(id!(menu)).changed(&actions) {
            if e.selected_id == id!(tab_overall)[0] {
                router.nav_to(cx, id!(overall_page));
            } else if e.selected_id == id!(tab_color)[0] {
                router.nav_to(cx, id!(color_page));
            } else if e.selected_id == id!(tab_font)[0] {
                router.nav_to(cx, id!(font_page));
            } else if e.selected_id == id!(tab_install)[0] {
                router.nav_to(cx, id!(install_page));
            } else if e.selected_id == id!(tab_qs)[0] {
                router.nav_to(cx, id!(qs_page));
            } else if e.selected_id == id!(tab_get_start)[0] {
                router.nav_to(cx, id!(start_page));
            }
        }

        router.borrow_mut().map(|mut route| {
            route.handle_nav_events(cx, &actions);
        });
    }
}
