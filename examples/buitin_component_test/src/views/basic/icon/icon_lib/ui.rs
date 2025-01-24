use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;

    pub IconLibUi = <ScrollYView>{
        height: Fit,
        width: Fill,
        spacing: 10.0,
        flow: Down,
        <GDivider>{
            height: Fit,
            <GView>{
                height:Fit,
                width: Fit,
                padding: {left: 16.0, right: 16.0,}
                align: {x: 0.5}
                <GLabel>{
                    text: "UI",
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
                    spacing: 0.0,
                    width: 90.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: Exit,
                    }
                    <GLabel>{
                        text: "Exit",
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 0.0,
                    width: 90.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: Expand,
                    }
                    <GLabel>{
                        text: "Expand",
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 0.0,
                    width: 90.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: ExpandTop,
                    }
                    <GLabel>{
                        text: "ExpandTop",
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 0.0,
                    width: 90.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: ExpandBottom,
                    }
                    <GLabel>{
                        text: "ExpandBottom",
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 0.0,
                    width: 90.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: ExpandLeft,
                    }
                    <GLabel>{
                        text: "ExpandLeft",
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 0.0,
                    width: 90.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: ExpandRight,
                    }
                    <GLabel>{
                        text: "ExpandRight",
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 0.0,
                    width: 90.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: Open,
                    }
                    <GLabel>{
                        text: "Open",
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 0.0,
                    width: 90.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: OpenLeft,
                    }
                    <GLabel>{
                        text: "OpenLeft",
                    }
                }
            }
            <GHLayout>{
                height: Fit,
                width: Fill,
                <GVLayout>{
                    height: Fit,
                    spacing: 0.0,
                    width: 90.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: OpenRight,
                    }
                    <GLabel>{
                        text: "OpenRight",
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 0.0,
                    width: 90.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: OpenTop,
                    }
                    <GLabel>{
                        text: "OpenTop",
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 0.0,
                    width: 90.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: OpenBottom,
                    }
                    <GLabel>{
                        text: "OpenBottom",
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 0.0,
                    width: 90.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: Split,
                    }
                    <GLabel>{
                        text: "Split",
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 0.0,
                    width: 90.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: Split2,
                    }
                    <GLabel>{
                        text: "Split2",
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 0.0,
                    width: 90.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: Poweroff,
                    }
                    <GLabel>{
                        text: "Poweroff",
                    }
                }
            }
        }
    }
}
