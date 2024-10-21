use gen_components::components::view::GView;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;
    BOLD_FONT = dep("crate://self/resources/OPPOSans-Bold.ttf");
    CPreview = <GView>{
        height: 120.0,
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
            height: Fill,
            width: Fill,
            align: {
                x: 0.5,
                y: 0.5,
            }
        }
    }

    OverallPage = {{OverallPage}}{
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
                            src: dep("crate://self/resources/rust.png"),
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
                        <GCheckBox>{selected: true}
                        <GCheckBox>{checkbox_type: Tick, selected: true}
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
            <GLabel>{
                font_size: 12.0,
                font_family: (BOLD_FONT),
                text: "Nav Components(导航组件)",
            }
            <GLabel>{
                font_size: 12.0,
                font_family: (BOLD_FONT),
                text: "Feedback Components(反馈组件)",
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
