use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;

    IconLibTool = <ScrollYView>{
        height: Fit,
        width: Fill,
        spacing: 10.0,
        flow: Down,
        <GDivider>{
            height: Fit,
            <GCard>{
                height:Fit,
                width: Fit,
                padding: {left: 16.0, right: 16.0,}
                align: {x: 0.5}
                <GLabel>{
                    text: "Tool",
                }
            }
        }
        <GVLayout>{
            height: Fit,
            spacing: 8.0,
            scroll_bars: <GScrollBars> {}
            <GHLayout>{
                height: Fit,
                width: Fill,
                <GVLayout>{
                    height: Fit,
                    spacing: 4.0,
                    width: 110.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: Search,
                    }
                    <GLabel>{
                        text: "Search",
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 4.0,
                    width: 110.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: ZoomIn,
                    }
                    <GLabel>{
                        text: "ZoomIn",
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 4.0,
                    width: 110.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: ZoomOut,
                    }
                    <GLabel>{
                        text: "ZoomOut",
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 4.0,
                    width: 110.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: Share,
                    }
                    <GLabel>{
                        text: "Share",
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 4.0,
                    width: 110.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: Rss,
                    }
                    <GLabel>{
                        text: "Rss",
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 4.0,
                    width: 110.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: AI,
                    }
                    <GLabel>{
                        text: "AI",
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 4.0,
                    width: 110.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: VR,
                    }
                    <GLabel>{
                        text: "VR",
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 4.0,
                    width: 110.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: Notice,
                    }
                    <GLabel>{
                        text: "Notice",
                    }
                }
            }

        }
        <GVLayout>{
            height: Fit,
            spacing: 8.0,
            scroll_bars: <GScrollBars> {}
            <GHLayout>{
                height: Fit,
                width: Fill,
                <GVLayout>{
                    height: Fit,
                    spacing: 4.0,
                    width: 110.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: NoticeNone,
                    }
                    <GLabel>{
                        text: "NoticeNone",
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 4.0,
                    width: 110.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: Bind,
                    }
                    <GLabel>{
                        text: "Bind",
                    }
                }
            }
        }
    }
}