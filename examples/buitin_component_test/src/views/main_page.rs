use gen_components::{
    shader::manual::RouterIndicatorMode,
    utils::lifetime::{Executor, Lifetime},
};
use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::views::overall::all::*;
    use crate::views::overall::color::*;
    use crate::views::overall::font::*;
    use crate::views::guide::install::*;
    use crate::views::guide::quickstart::*;
    use crate::views::guide::start::*;
    use crate::views::basic::label::*;
    use crate::views::basic::button::*;
    use crate::views::basic::view::*;
    use crate::views::basic::svg::*;
    use crate::views::basic::image::*;
    use crate::views::basic::icon::*;
    use crate::views::basic::divider::*;
    use crate::views::basic::link::*;
    use crate::views::basic::scroll::*;
    use crate::views::form::radio::*;
    use crate::views::form::checkbox::*;
    use crate::views::form::toggle::*;
    use crate::views::basic::shader::*;
    use crate::views::form::progress::*;
    use crate::views::form::upload::*;
    use crate::views::data::tag::*;
    use crate::views::data::loading::*;
    use crate::views::data::splitter::*;
    use crate::views::data::collapse::*;
    use crate::views::nav::tool_btn::*;
    use crate::views::nav::window::*;
    use crate::views::nav::breadcrumb::*;
    use crate::views::nav::router::*;
    use crate::views::nav::menu::*;
    use crate::views::nav::tabbar::*;
    use crate::views::feedback::state::*;
    use crate::views::feedback::popup::*;
    use crate::views::feedback::tool_tip::*;
    use crate::views::feedback::dialog::*;
    use crate::views::feedback::drawer::*;
    use crate::views::form::input::*;

    BOLD_FONT = dep("crate://self/resources/OPPOSans-Bold.ttf");
    pub AppMainPage = {{AppMainPage}}{
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
                visible: false
            }
            body: {
                scroll_bars: <GScrollBars>{},
                sub1 = <GSubMenu>{
                    title: {
                        <GLabel>{
                            font_size: 11.0,
                            text: "Guide",
                        }
                    }
                    items: {
                        tab_get_start = <GMenuItem>{
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
                        tab_label = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Label",
                            }
                        }
                        tab_view = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "View",
                            }
                        }
                        tab_button = <GMenuItem>{

                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Button",
                            }
                        }
                        tab_svg = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Svg",
                            }
                        }
                        tab_image = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Image",
                            }
                        }
                        tab_icon_lib = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Icon Lib",
                            }
                        }
                        tab_divider = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Divider",
                            }
                        }
                        tab_link = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Link",
                            }
                        }
                        tab_scroll = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Scroll",
                            }
                        }
                        tab_shader = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Shader",
                            }
                        }
                    }
                }
                sub4 = <GSubMenu>{
                    title: {
                        <GLabel>{
                            font_size: 11.0,
                            text: "Form",
                        }
                    }
                    items: {
                        tab_radio = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Radio",
                            }
                        }
                        tab_checkbox = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Checkbox",
                            }
                        }
                        tab_toggle = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Toggle",
                            }
                        }
                        tab_input = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Input",
                            }
                        }
                        tab_progress = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Progress",
                            }
                        }
                        tab_upload = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Upload",
                            }
                        }
                    }
                }
                sub5 = <GSubMenu>{
                    title: {
                        <GLabel>{
                            font_size: 11.0,
                            text: "Data",
                        }
                    }
                    items: {
                        tab_tag = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Tag",
                            }
                        }
                        tab_loading = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Loading",
                            }
                        }
                        tab_splitter = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Splitter",
                            }
                        }
                        tab_collapse = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Collapse",
                            }
                        }
                    }
                }
                sub6 = <GSubMenu>{
                    title: {
                        <GLabel>{
                            font_size: 11.0,
                            text: "Nav",
                        }
                    }
                    items: {
                        tab_window = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Window",
                            }
                        }
                        tab_tool_btn = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Tool Button",
                            }
                        }
                        tab_breadcrumb = <GMenuItem>{
                            selected: true,
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Breadcrumb",
                            }
                        }
                        tab_menu = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Menu",
                            }
                        }
                        tab_tabbar = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "TabBar",
                            }
                        }
                        tab_router = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Router",
                            }
                        }
                    }
                }
                sub7 = <GSubMenu>{
                    title: {
                        <GLabel>{
                            font_size: 11.0,
                            text: "Feedback",
                        }
                    }
                    items: {
                        tab_state = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "State",
                            }
                        }
                        tab_popup = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Popup",
                            }
                        }
                        tab_tool_tip = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "ToolTip",
                            }
                        }
                        tab_dialog = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Dialog",
                            }
                        }
                        tab_drawer = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Drawer",
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
                    label_page = <GBarPage>{
                        <LabelPage>{}
                    }
                    button_page = <GBarPage>{
                        <ButtonPage>{}
                    }
                    view_page = <GBarPage>{
                        <ViewPage>{}
                    }
                    svg_page = <GBarPage>{
                        <SvgPage>{}
                    }
                    image_page = <GBarPage>{
                        <ImagePage>{}
                    }
                    icon_page = <GBarPage>{
                        <IconPage>{}
                    }
                    divider_page = <GBarPage>{
                        <DividerPage>{}
                    }
                    link_page = <GBarPage>{
                        <LinkPage>{}
                    }
                    scroll_page = <GBarPage>{
                        <ScrollPage>{}
                    }
                    radio_page = <GBarPage>{
                        <RadioPage>{}
                    }
                    checkbox_page = <GBarPage>{
                        <CheckboxPage>{}
                    }
                    input_page = <GBarPage>{
                        <InputPage>{}
                    }
                    toggle_page = <GBarPage>{
                        <TogglePage>{}
                    }
                    shader_page = <GBarPage>{
                        <ShaderPage>{}
                    }
                    progress_page = <GBarPage>{
                        <ProgressPage>{}
                    }
                    upload_page = <GBarPage>{
                        <UploadPage>{}
                    }
                    tag_page = <GBarPage>{
                        <TagPage>{}
                    }
                    loading_page = <GBarPage>{
                        <LoadingPage>{}
                    }
                    tool_btn_page = <GBarPage>{
                        <ToolBtnPage>{}
                    }
                    window_page = <GBarPage>{
                        <WindowPage>{}
                    }
                    split_page = <GBarPage>{
                        <SplitterPage>{}
                    }
                    state_page = <GBarPage>{
                        <StatePage>{}
                    }
                    popup_page = <GBarPage>{
                        <PopupPage>{}
                    }
                    tool_tip_page = <GBarPage>{
                        <ToolTipPage>{}
                    }
                    dialog_page = <GBarPage>{
                        <DialogPage>{}
                    }
                    drawer_page = <GBarPage>{
                        <DrawerPage>{}
                    }
                    breadcrumb_page = <GBarPage>{
                        <BreadCrumbPage>{}
                    }
                    menu_page = <GBarPage>{
                        <MenuPage>{}
                    }
                    tabbar_page = <GBarPage>{
                        <TabbarPage>{}
                    }
                    router_page = <GBarPage>{
                        <RouterPage>{}
                    }
                    collapse_page = <GBarPage>{
                        <CollapsePage>{}
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
                            ids!(
                                start_page,
                                overall_page,
                                color_page,
                                font_page,
                                install_page,
                                qs_page,
                                label_page,
                                button_page,
                                view_page,
                                svg_page,
                                image_page,
                                icon_page,
                                divider_page,
                                link_page,
                                scroll_page,
                                radio_page,
                                checkbox_page,
                                toggle_page,
                                shader_page,
                                progress_page,
                                upload_page,
                                tag_page,
                                loading_page,
                                tool_btn_page,
                                window_page,
                                split_page,
                                state_page,
                                popup_page,
                                tool_tip_page,
                                dialog_page,
                                drawer_page,
                                breadcrumb_page,
                                menu_page,
                                tabbar_page,
                                router_page,
                                input_page,
                                collapse_page
                            ),
                            None,
                            Some(RouterIndicatorMode::Define),
                        )
                        .active(id!(start_page))
                        .build(cx);
                });
            })
            .map(|_| {
                let router = self.grouter(id!(app_router));
                router.borrow().map(|router| {
                    if router.scope_path.is_some() {
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
            } else if e.selected_id == id!(tab_label)[0] {
                router.nav_to(cx, id!(label_page));
            } else if e.selected_id == id!(tab_button)[0] {
                router.nav_to(cx, id!(button_page));
            } else if e.selected_id == id!(tab_view)[0] {
                router.nav_to(cx, id!(view_page));
            } else if e.selected_id == id!(tab_svg)[0] {
                router.nav_to(cx, id!(svg_page));
            } else if e.selected_id == id!(tab_image)[0] {
                router.nav_to(cx, id!(image_page));
            } else if e.selected_id == id!(tab_icon_lib)[0] {
                router.nav_to(cx, id!(icon_page));
            } else if e.selected_id == id!(tab_divider)[0] {
                router.nav_to(cx, id!(divider_page));
            } else if e.selected_id == id!(tab_link)[0] {
                router.nav_to(cx, id!(link_page));
            } else if e.selected_id == id!(tab_scroll)[0] {
                router.nav_to(cx, id!(scroll_page));
            } else if e.selected_id == id!(tab_radio)[0] {
                router.nav_to(cx, id!(radio_page));
            } else if e.selected_id == id!(tab_checkbox)[0] {
                router.nav_to(cx, id!(checkbox_page));
            } else if e.selected_id == id!(tab_toggle)[0] {
                router.nav_to(cx, id!(toggle_page));
            } else if e.selected_id == id!(tab_shader)[0] {
                router.nav_to(cx, id!(shader_page));
            } else if e.selected_id == id!(tab_progress)[0] {
                router.nav_to(cx, id!(progress_page));
            } else if e.selected_id == id!(tab_upload)[0] {
                router.nav_to(cx, id!(upload_page));
            } else if e.selected_id == id!(tab_tag)[0] {
                router.nav_to(cx, id!(tag_page));
            } else if e.selected_id == id!(tab_loading)[0] {
                router.nav_to(cx, id!(loading_page));
            } else if e.selected_id == id!(tab_tool_btn)[0] {
                router.nav_to(cx, id!(tool_btn_page));
            } else if e.selected_id == id!(tab_window)[0] {
                router.nav_to(cx, id!(window_page));
            } else if e.selected_id == id!(tab_splitter)[0] {
                router.nav_to(cx, id!(split_page));
            } else if e.selected_id == id!(tab_state)[0]{
                router.nav_to(cx, id!(state_page));
            } else if e.selected_id == id!(tab_popup)[0]{
                router.nav_to(cx, id!(popup_page));
            } else if e.selected_id == id!(tab_tool_tip)[0]{
                router.nav_to(cx, id!(tool_tip_page));
            } else if e.selected_id == id!(tab_dialog)[0]{
                router.nav_to(cx, id!(dialog_page));
            } else if e.selected_id == id!(tab_drawer)[0]{
                router.nav_to(cx, id!(drawer_page));
            } else if e.selected_id == id!(tab_breadcrumb)[0] {
                router.nav_to(cx, id!(breadcrumb_page));
            } else if e.selected_id == id!(tab_menu)[0] {
                router.nav_to(cx, id!(menu_page));
            } else if e.selected_id == id!(tab_tabbar)[0] {
                router.nav_to(cx, id!(tabbar_page));
            } else if e.selected_id == id!(tab_router)[0] {
                router.nav_to(cx, id!(router_page));
            } else if e.selected_id == id!(tab_collapse)[0] {
                router.nav_to(cx, id!(collapse_page));
            }else if e.selected_id == id!(tab_input)[0] {
                router.nav_to(cx, id!(input_page));
            }
        }

        router.borrow_mut().map(|mut route| {
            route.handle_nav_events(cx, &actions);
        });
    }
}
