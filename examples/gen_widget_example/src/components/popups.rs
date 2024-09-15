use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;

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
            <GDropDown>{
                offset: 6.0,
                height: Fit,
                width: Fit,
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
                trigger = <GButton>{slot: {
                    text:"open bottom"
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
                        <GCard>{
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
    }
}
