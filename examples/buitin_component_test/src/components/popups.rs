use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;

    GPopupExample = <ScrollYView>{
        height: 600.0,
        width: Fill,
        spacing: 10.0,
        flow: Down,
        <GLabel>{
            text: "GPopup"
        }
        <GHLayout>{
            height: 200.0,
            width: Fill,
            spacing: 10.0,
            align: {
                x: 0.5
            }
            <GDropDown>{
                offset: 6.0,
                height: Fit,
                width: Fit,
                // trigger_mode: Hover,
                trigger = <GButton>{
                    slot: {
                        text:"open"
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
                        <GButton>{
                            theme: Dark,
                        }
                        <View>{
                            show_bg: true,
                            draw_bg: {color: #f00},
                            height: 40.0,
                            width: 40.0,
                        }
                    }
                }
            }
        }
        <GLabel>{
            text: "ToolTip"
        }
        <GHLayout>{
            height: 200.0,
            width: Fill,
            spacing: 8.0,
            align: {
                x: 0.5
            }
            <GDropDown>{
                mode: ToolTip,
                height: Fit,
                width: Fit,
                trigger = <GIcon>{
                    icon_type: Add,
                },
                popup :<GToolTip> {
                    height: 150.0,
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
            <GDropDown>{
                mode: ToolTip,
                height: Fit,
                width: Fit,
                position: BottomRight,
                trigger = <GButton>{slot: {
                    text:"open bottom right"
                }},
                popup :<GToolTip> {
                    height: 150.0,
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
            <GDropDown>{
                mode: ToolTip,
                height: Fit,
                width: Fit,
                position: BottomLeft,
                trigger = <GButton>{slot: {
                    text:"open bottom left"
                }},
                popup :<GToolTip> {
                    height: 150.0,
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
            <GDropDown>{
                mode: ToolTip,
                height: Fit,
                width: Fit,
                position: Top,
                trigger = <GButton>{slot: {
                    text:"open top"
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
            <GDropDown>{
                mode: ToolTip,
                height: Fit,
                width: Fit,
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
            <GDropDown>{
                mode: ToolTip,
                height: Fit,
                width: Fit,
                position: TopRight,
                trigger = <GButton>{slot: {
                    text:"open top right"
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
        <GHLayout>{
            height: 200.0,
            width: Fill,
            spacing: 8.0,
            align: {
                x: 0.5
            }
            <GDropDown>{
                mode: ToolTip,
                height: Fit,
                width: Fit,
                position: Left,
                trigger = <GButton>{slot: {
                    text:"open left"
                }},
                popup :<GToolTip> {
                    theme: Dark,
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
            <GDropDown>{
                mode: ToolTip,
                height: Fit,
                width: Fit,
                position: LeftTop,
                trigger = <GButton>{slot: {
                    text:"open left top"
                }},
                popup :<GToolTip> {
                    theme: Dark,
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
            <GDropDown>{
                mode: ToolTip,
                height: Fit,
                width: Fit,
                position: LeftBottom,
                trigger = <GButton>{slot: {
                    text:"open left bottom"
                }},
                popup :<GToolTip> {
                    theme: Dark,
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
            <GDropDown>{
                mode: ToolTip,
                height: Fit,
                width: Fit,
                position: Right,
                trigger = <GButton>{slot: {
                    text:"open right"
                }},
                popup :<GToolTip> {
                    theme: Dark,
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
            <GDropDown>{
                mode: ToolTip,
                height: Fit,
                width: Fit,
                position: RightTop,
                trigger = <GButton>{slot: {
                    text:"open right top"
                }},
                popup :<GToolTip> {
                    theme: Dark,
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
            <GDropDown>{
                mode: ToolTip,
                height: Fit,
                width: Fit,
                position: RightBottom,
                trigger = <GButton>{slot: {
                    text:"open right bottom"
                }},
                popup :<GToolTip> {
                    theme: Dark,
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
        <GLabel>{
            text: "GDialog"
        }
        <GHLayout>{
            height: 200.0,
            width: Fill,
            spacing: 10.0,
            <GDropDown>{
                mode: Dialog,
                height: Fit,
                width: Fit,
                trigger = <GButton>{
                    slot: {
                        text:"open"
                    }
                },
                popup :<GDialog> {
                    container: {
                        height: 200.0,
                        width: 300.0,
                        flow: Down,
                        spacing: 10.0,
                        padding: 10.0,
                        <GView>{
                            height: Fill,
                            width: Fill,
                            spread_radius: 4.6,
                            blur_radius: 4.6,
                            spacing: 12.0,
                            flow: Down,
                            clip_x: false,
                            clip_y: false,
                            padding: 6.0,
                            shadow_offset: vec2(0.0, 2.0),
                            <GLabel>{
                                text:"This is a popup",
                            }
                            <GButton>{
                                theme: Dark,
                            }
                            <View>{
                                show_bg: true,
                                draw_bg: {color: #f00},
                                height: 40.0,
                                width: 40.0,
                            }
                        }
                    }
                }
            }
        }
        <GLabel>{
            text: "GDrawer"
        }
        <GHLayout>{
            height: 200.0,
            width: Fill,
            spacing: 10.0,
            <GDropDown>{
                mode: Drawer,
                height: Fit,
                width: Fit,
                trigger = <GButton>{
                    slot: {
                        text:"bottom"
                    }
                },
                popup :<GDrawer> {
                    container: {
                        flow: Down,
                        spacing: 10.0,
                        padding: 10.0,
                        <GLabel>{
                            text:"This is a popup",
                        }
                        <GButton>{
                            theme: Dark,
                        }
                    }
                }
            }
            <GDropDown>{
                mode: Drawer,
                height: Fit,
                width: Fit,
                position: Top,
                trigger = <GButton>{
                    slot: {
                        text:"top"
                    }
                },
                popup :<GDrawer> {
                    container: {
                        flow: Down,
                        spacing: 10.0,
                        padding: 10.0,
                        background_visible: false,
                        <GLabel>{
                            text:"This is a popup",
                            abs_pos: vec2(132.0, 120.0),
                        }
                        <GLabel>{
                            text:"666",
                            abs_pos: vec2(262.0, 60.0),
                        }
                        <GLabel>{
                            text:"amazing....",
                            abs_pos: vec2(445.0, 84.0),
                        }
                        <GLabel>{
                            text:"I want see more!",
                            abs_pos: vec2(32.0, 100.0),
                        }
                    }
                }
            }
            <GView>{
                width: Fill,
            }
            <GDropDown>{
                mode: Drawer,
                height: Fit,
                width: Fit,
                position: Left,
                trigger = <GButton>{
                    slot: {
                        text:"left"
                    }
                },
                popup :<GDrawer> {
                    container: {
                        flow: Down,
                        spacing: 10.0,
                        padding: 10.0,
                        <GLabel>{
                            text:"This is a popup",
                        }
                        <GButton>{
                            theme: Dark,
                        }
                    }
                }
            }
            <GDropDown>{
                mode: Drawer,
                height: Fit,
                width: Fit,
                position: Right,
                trigger = <GButton>{
                    slot: {
                        text:"right"
                    }
                },
                popup :<GDrawer> {
                    container: {
                        flow: Down,
                        spacing: 10.0,
                        padding: 10.0,
                        <GLabel>{
                            text:"This is a popup",
                        }
                        <GButton>{
                            theme: Dark,
                        }
                    }
                }
            }
        }
    }
}
