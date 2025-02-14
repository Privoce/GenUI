use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    BOLD_FONT = dep("crate://self/resources/OPPOSans-Bold.ttf");
    pub CPreview = <GView>{
        height: 140.0,
        width: Fill,
        border_radius: 4.0,
        theme: Dark,
        flow: Down,
        header = <GView>{
            height: Fit,
            width: Fill,
            padding: 8.0,
            background_color: #1D1E1F,
            title = <GLabel>{
                font_family: (BOLD_FONT),
            }
        }
        body = <GView>{
            clip_x: true,
            clip_y: true,
            background_color: #393C48,
            height: Fill,
            width: Fill,
            align: {
                x: 0.5,
                y: 0.5,
            }
        }
    }

    pub OverallPage = {{OverallPage}}{
        height: Fill,
        width: Fill,
        flow: Down,
        background_visible: false,
        border_radius: 0.0,
        spacing: 12.0,
        padding: 12.0,
        scroll_bars: <GScrollBars>{},
        clip_x: true,
        clip_y: true,
        <GLabel>{
            font_size: 14.0,
            font_family: (BOLD_FONT),
            text: "Overall Page(组件总览)",
        }
        <GVLayout>{
            height: Fit,
            spacing: 8.0,
            <GLabel>{
                font_size: 12.0,
                font_family: (BOLD_FONT),
                text: "Basic Components(基础组件)",
            }
            <GHLayout>{
                height: Fit,
                spacing: 8.0,
                <CPreview>{
                    header = {title = {text: "Label"}}
                    body = {
                        spacing: 8.0,
                        <GLabel>{
                            font_size: 12.0,
                            text: "Text"
                        }
                        <GLabel>{
                            font_size: 10.0,
                            text: "Text"
                        }
                        <GLabel>{
                            font_size: 8.0,
                            text: "Text"
                        }
                    }
                }
                <CPreview>{
                    header = {title = {text: "View|HLayout|VLayout"}}
                    body = {
                        padding: 16.0,
                        <GView>{
                            height: Fill,
                            width: Fill,
                            theme: Primary
                        }
                    }
                }
                <CPreview>{
                    header = {title = {text: "Button"}}
                    body = {
                        spacing: 8.0,
                        <GButton>{}
                        <GButton>{theme: Success}
                    }
                }
            }
            <GHLayout>{
                height: Fit,
                spacing: 8.0,
                <CPreview>{
                    header = {title = {text: "Svg"}}
                    body = {
                        <GSvg>{
                            theme: Dark,
                            height: 32.0,
                            width: 32.0,
                            src: dep("crate://self/resources/upload.svg"),
                        }
                    }
                }
                <CPreview>{
                    header = {title = {text: "Image"}}
                    body = {
                        padding: 16.0,
                        <GImage>{
                            height: 36.0,
                            width: 42.0,
                            src: Live(dep("crate://self/resources/rust.png")),
                        }
                    }
                }
                <CPreview>{
                    header = {title = {text: "Icon(Icon Lib)"}}
                    body = {
                        <GIcon>{
                            theme: Dark,
                            height: 36.0,
                            width: 36.0,
                            icon_type: Setting3,
                            stroke_width: 1.4
                        }
                    }
                }
            }
            <GHLayout>{
                height: Fit,
                spacing: 8.0,
                <CPreview>{
                    header = {title = {text: "Divider"}}
                    body = {
                        flow: Down,
                        spacing: 16.0,
                        padding: 8.0,
                        <GDivider>{
                            height: 6.0,
                            stroke_width: 1.2,
                            theme: Primary,
                        }
                        <GDivider>{
                            height: 6.0,
                            stroke_width: 1.2,
                            theme: Error,
                        }
                    }
                }
                <CPreview>{
                    header = {title = {text: "Link"}}
                    body = {
                        padding: 16.0,
                        <GLink>{
                           text: "GenUI Builtin Components",
                           href: "https://github.com/Privoce/GenUI/tree/components/gen/components"
                        }
                    }
                }
                <CPreview>{
                    header = {title = {text: "Color"}}
                    body = {
                        <GColor>{
                            width: Fill,
                            item: {
                                width: 24.0,
                            }
                            header: {
                                visible: false,
                            }
                            theme: Dark,
                        }
                    }
                }
            }
            <GHLayout>{
                height: Fit,
                spacing: 8.0,
                <CPreview>{
                    header = {title = {text: "ScrollBars"}}
                    body = {
                        padding: 8.0,
                        <GHLayout>{
                            scroll_bars: <GScrollBars>{},
                            spacing:16.0,
                            <GView>{
                                theme: Info
                            }
                            <GView>{
                                width: 60.0,
                                height: 60.0,
                                theme: Success,
                            }
                            <GView>{
                                width: 300.0,
                                theme: Error
                            }
                        }
                    }
                }
                <CPreview>{
                    header = {title = {text: "Shader"}}
                    body = {
                        padding: 8.0,
                        <GShader>{
                            height: 200.0,
                            width: 200.0,
                            animation_key: false,
                            draw_shader:{
                                fn pixel(self) -> vec4 {
                                                
                                    let uv = self.pos - 0.5;
                                    uv.x *= self.rect_size.x / self.rect_size.y;
                
                                    let radius = length(uv);
                                    let wave = sin(radius * 10.0 - 1.2 * 2.0);
                                    let intensity = wave * 0.5 + 0.5;
                                    let col = vec3(intensity);
                
                                    return vec4(col, 1.0);
                                }
                            }
                        }
                    }
                }
            }
            <GLabel>{
                font_size: 12.0,
                font_family: (BOLD_FONT),
                text: "Form Components(表单组件)",
            }
            <GHLayout>{
                height: Fit,
                spacing: 8.0,
                <CPreview>{
                    header = {title = {text: "Radio"}}
                    body = {
                        spacing: 16.0,
                        <GRadio>{selected: true}
                        <GRadio>{
                            selected: true,
                            radio_type: Tick,
                        }
                    }
                }
                <CPreview>{
                    header = {title = {text: "CheckBox"}}
                    body = {
                        spacing: 16.0,
                        <GCheckbox>{selected: true}
                        <GCheckbox>{checkbox_type: Tick, selected: true}
                    }
                }
                <CPreview>{
                    header = {title = {text: "Toggle"}}
                    body = {
                        spacing: 16.0,
                        <GToggle>{
                            selected: true
                        }
                        <GToggle>{
                            toggle_type: Rect,
                        }
                    }
                }
            }
            <GHLayout>{
                height: Fit,
                spacing: 8.0,
                <CPreview>{
                    header = {title = {text: "Progress"}}
                    body = {
                        spacing: 16.0,
                        padding: {left: 16.0, right: 16.0},
                        <GProgress>{
                            width: 140.0,
                            value: 0.6
                        }
                        
                    }
                }
                <CPreview>{
                    header = {title = {text: "Upload"}}
                    body = {
                        spacing: 16.0,
                        <GUpload>{
                            height: 70.0,
                            mode: Folder
                        }
                    }
                }
                <CPreview>{
                    header = {title = {text: "Input"}}
                    body = {
                        spacing: 16.0,
                        padding: {left: 16.0, right: 16.0},
                        <GInput>{
                            width: Fill,
                            height: 32.0,
                        }
                    }
                }
            }
            <GHLayout>{
                height: Fit,
                spacing: 8.0,
                <CPreview>{
                    header = {title = {text: "Select"}}
                    body = {
                        spacing: 16.0,
                        padding: {left: 16.0, right: 16.0},
                    
                    }
                }
            }
            <GLabel>{
                font_size: 12.0,
                font_family: (BOLD_FONT),
                text: "Data Components(数据组件)",
            }
            <GHLayout>{
                height: Fit,
                spacing: 8.0,
                <CPreview>{
                    header = {title = {text: "Tag"}}
                    body = {
                        spacing: 4.0,
                        padding: 6.0,
                        flow: Down,
                        <GTag>{
                            theme: Success,
                            text: "badge tag1",
                        }
                        <GTag>{
                            theme: Info,
                            src: dep("crate://self/resources/config.svg"),
                            text: "other",
                        }
                        <GTag>{
                            theme: Error,
                            text: "closeable",
                            closeable: true
                        }
                    }
                }
                <CPreview>{
                    header = {title = {text: "Loading"}}
                    body = {
                        spacing: 16.0,
                        padding: {left: 16.0, right: 16.0},
                        <GLoading>{
                            height: 64.0,
                            width: 64.0,
                            theme: Error,
                            animation_key: true,
                            visible: true,
                        }
                    }
                }
                <CPreview>{
                    header = {title = {text: "Splitter"}}
                    body = {
                        padding: 12.0,
                        <GSplitter>{
                            height: Fill,
                            align: FromA(60),
                            a: <GView>{
                                height: Fill,
                                width: 100.0,
                                theme: Error
                            },
                            b: <GView>{
                                height: Fill,
                                width: 100.0,
                                theme: Success
                            }
                        }
                    }
                }
            }
            <GHLayout>{
                height: Fit,
                spacing: 8.0,
                <CPreview>{
                    header = {title = {text: "Collapse"}}
                    body = {
                        spacing: 4.0,
                        padding: 6.0,
                        flow: Down,
                        <GCollapse>{
                            height: 100.0,
                            width: 300.0,
                            opened: true,
                            position: Right,
                            header: {
                                background_color: #DDD,
                                <GLabel>{
                                    text: "Right",
                                }
                                width: 60.0,
                                height: Fill,
                            }
                            body: {
                                background_color: #888,
                                height: Fill,
                                width: 240.0,
                                theme: Dark,
                            }
                        }
                    }
                }
                
            }
            <GLabel>{
                font_size: 12.0,
                font_family: (BOLD_FONT),
                text: "Nav Components(导航组件)",
            }
            <GHLayout>{
                height: Fit,
                spacing: 8.0,
                <CPreview>{
                    header = {title = {text: "Window"}}
                    body = {
                        spacing: 16.0,
                        padding: 12.0,
                        <GView>{
                            border_width: 1.0,
                            height: Fill,
                            width: Fill,
                            background_color: #DDD,
                            window_bar = <GHLayout>{
                                height: 32.0,
                                width: Fill,
                                background_color: #1F1E25,
                                background_visible: true,
                                align: {
                                    x: 0.0, y: 0.5
                                }
                                spacing: 0.0,
                                mac_btns_wrap = <GHLayout>{
                                    visible: true
                                    height: 32.0,
                                    width: Fit,
                                    spacing: 6.0,
                                    align: {x: 0.0, y: 0.5},
                                    padding: {left: 6.0},
                                    close = <GToolButton> {icon_type: Close, os_type: Mac}
                                    max = <GToolButton> {icon_type: Max, os_type: Mac}
                                    min = <GToolButton> {icon_type: Min, os_type: Mac}
                                }
                                window_title = <GHLayout>{
                                    height: Fill,
                                    width: Fill,
                                    align: {x: 0.5, y: 0.5},
                                    spacing: 6.0,
                                    icon = <GImage>{
                                        
                                        src: Live(dep("crate://self/resources/rust.png")),
                                        height: 16.0,
                                        width: 16.0,
                                    },
                                    title = <GLabel>{
                                        height: Fit,
                                        text: "Window",
                                        font_size: 9.0,
                                    },
                                }
                            }
                        }
                    }
                }
                <CPreview>{
                    header = {title = {text: "Tool Button"}}
                    body = {
                        spacing: 16.0,
                        padding: {left: 16.0, right: 16.0},
                        flow: Down,
                        <GHLayout>{
                            align: {x: 0.5},
                            height: Fit,
                            spacing: 8.0,
                            <GToolButton>{
                                os_type: Mac,
                                icon_type: Min
                            }
                            <GToolButton>{
                                os_type: Mac,
                                icon_type: Max
                            }
                            <GToolButton>{
                                os_type: Mac,
                                icon_type: FullScreen
                            }
                            <GToolButton>{
                                os_type: Mac,
                                icon_type: Close
                            }
                        }
                        <GHLayout>{
                            height: Fit,
                            align: {x: 0.5},
                            spacing: 8.0,
                            <GToolButton>{
                                os_type: Windows,
                                icon_type: Min
                            }
                            <GToolButton>{
                                os_type: Windows,
                                icon_type: Max
                            }
                            <GToolButton>{
                                os_type: Windows,
                                icon_type: FullScreen
                            }
                            <GToolButton>{
                                os_type: Windows,
                                icon_type: Close
                            }
                        }
                    }
                }
            }
            <GHLayout>{
                height: Fit,
                spacing: 8.0,
                <CPreview>{
                    header = {title = {text: "BreadCrumb"}}
                    body = {
                        padding: 4.0,
                        <GBreadCrumb>{
                            theme: Info,
                            path: ["home", "components", "button"],
                        }
                    }
                }
                <CPreview>{
                    header = {title = {text: "Tabbar"}}
                    body = {
                        padding: 4.0,
                        <GTabbar>{
                            theme: Info,
                            align: {x: 0.5},
                            width: Fill,
                            <GTabbarItem>{
                                icon_slot: {
                                    src: dep("crate://self/resources/all.svg"),
                                }
                                text_slot: {
                                    text: "All",
                                }
                            }
                            <GTabbarItem>{
        
                            }
                            <GTabbarItem>{
                                icon_slot: {
                                    src: dep("crate://self/resources/lightning.svg"),
                                }
                                text_slot: {
                                    text: "Lightning",
                                }
                            }
                        }
                    }
                }
            }
            <GHLayout>{
                height: Fit,
                spacing: 8.0,
                
                <CPreview>{
                    header = {title = {text: "Menu"}}
                    body = {
                        spacing: 0.0,
                        padding: 12.0,
                        flow: Down,
                        <GMenuItem>{
                            text_slot: {text: "Menu item 1"},
                            icon_slot: {visible: false}
                        }
                        <GMenuItem>{
                            theme: Success
                            text_slot: {text: "Menu item 2"},
                            icon_slot: {src: dep("crate://self/resources/dislike.svg")}
                        }
                    }
                }
                <CPreview>{
                    header = {title = {text: "Router"}}
                    body = {
                        spacing: 0.0,
                        padding: 4.0,
                        flow: Down,
                        <GLabel>{
                            width: Fill,
                            text: r#"
                            {
                               path: "/home",
                               component: <HomePage>,
                            }
                            "#,
                        }
                    }
                }
            }
            <GLabel>{
                font_size: 12.0,
                font_family: (BOLD_FONT),
                text: "Feedback Components(反馈组件)",
            }
            <GHLayout>{
                height: Fit,
                spacing: 8.0,
                <CPreview>{
                    header = {title = {text: "State"}}
                    body = {
                        padding: 4.0,
                        <GVLayout>{
                            theme: Success,
                            height: Fill,
                            width: Fill,
                            spacing: 8.0,
                            align: {x: 0.5, y: 0.5},
                            <GIcon>{
                                color: #DDD,
                                height: 32.0,
                                width: 32.0,
                                icon_type: Help,
                                stroke_width: 1.6,
                            }
                            <GLabel>{
                                text: "This a help message"
                            }
                            <GHLayout>{
                                height: Fit,
                                spacing: 16.0,
                                align: {x: 0.5},
                                <GButton>{theme: Info, slot: {text: "Cancel"}}
                                <GButton>{theme: Success, slot: {text: "OK"}}
                            }
                        }
                    }
                }
                <CPreview>{
                    header = {title = {text: "Popup"}}
                    body = {
                        spacing: 16.0,
                        padding: 16.0,
                        flow: Down,
                        <GDropDown>{
                            offset: 6.0,
                            height: Fit,
                            width: Fit,
                            trigger = <GButton>{
                                slot: {
                                    text:"Click to open"
                                }
                            },
                            popup :<GPopup> {
                                height: 150.0,
                                width: 200.0,
                                container: <GPopupContainer> {
                                    height: Fill,
                                    width: Fill,
                                    flow: Down,
                                    spacing: 10.0,
                                    padding: 10.0,
                                    <GLabel>{
                                        text:"This is a popup",
                                    }
                                }
                            }
                        }
                    }
                }
                <CPreview>{
                    header = {title = {text: "ToolTip"}}
                    body = {
                        spacing: 16.0,
                        padding: 16.0,
                        flow: Down,
                        <GDropDown>{
                            position: TopLeft,
                            trigger = <GButton>{slot: {
                                text:"open top left"
                            }},
                            popup :<GToolTip> {
                                height: 100.0,
                                width: 200.0,
                                container: {
                                    height: Fill,
                                    width: Fill,
                                    flow: Down,
                                    spacing: 10.0,
                                    padding: 10.0,
                                    <GLabel>{
                                        text:"This is a popup",
                                    }
                                }
                            }
                        }
                    }
                }
            }
            <GHLayout>{
                height: Fit,
                spacing: 8.0,
                <CPreview>{
                    header = {title = {text: "Dialog"}}
                    body = {
                        padding: 4.0,
                        <GView>{
                            height: Fill,
                            width: Fill,
                            spread_radius: 4.6,
                            blur_radius: 4.6,
                            spacing: 8.0,
                            flow: Down,
                            clip_x: false,
                            clip_y: false,
                            padding: 8.0,
                            shadow_offset: vec2(0.0, 2.0),
                            header = <GHLayout>{
                                height: 24.0,
                                align: { y: 0.5},
                                <GLabel>{
                                    text: "Dialog",
                                    font_size: 12.0,
                                    font_family: (BOLD_FONT),
                                }
                                <GHLayout>{
                                    align: {x: 1.0, y: 0.5},
                                    close_icon = <GIcon>{
                                        height: 10.0,
                                        width: 10.0,
                                        animation_key: true,
                                        stroke_hover_color: #FF0000,
                                        icon_type: Close,
                                    }
                                }
                            }
                            body = <GVLayout>{
                                height: Fill,
                                <GLabel>{
                                    text: "This is a dialog",
                                }
                            }
                            footer = <GHLayout>{
                                height: 60.0,
                                align: {x: 1.0, y: 0.5},
                                spacing: 16.0,
                                cancel = <GButton>{
                                    theme: Info,
                                    slot: {
                                        text: "Cancel"
                                    }
                                }
                                confirm = <GButton>{
                                    slot: {
                                        text: "Confirm"
                                    }
                                }
                            }
                        }
                    }
                }
                <CPreview>{
                    header = {title = {text: "Drawer"}}
                    body = {
                        spacing: 16.0,
                        padding: 16.0,
                        flow: Down,
                        <GView>{
                            theme: Info,
                            height: Fill,
                            width: Fill,
                            border_width: 1.0,
                            align: {x: 1.0}
                            <GView>{
                                background_color: #DDD,
                                width: 60.0,
                                height: Fill,

                            }
                        }
                    }
                }
                
            }
            <GLabel>{
                font_size: 12.0,
                font_family: (BOLD_FONT),
                text: "Virtual Components(虚拟组件)",
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct OverallPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for OverallPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for OverallPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let _ = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
    }
}
