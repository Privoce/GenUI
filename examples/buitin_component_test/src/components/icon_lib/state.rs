use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;

    IconLibState = <ScrollYView>{
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
                    text: "State",
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
                        icon_type: Info,
                    }
                    <GLabel>{
                        text: "Info",
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 4.0,
                    width: 110.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: Help,
                    }
                    <GLabel>{
                        text: "Help",
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 4.0,
                    width: 110.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: Warn,
                    }
                    <GLabel>{
                        text: "Warn",
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 4.0,
                    width: 110.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: Wifi,
                    }
                    <GLabel>{
                        text: "Wifi",
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 4.0,
                    width: 110.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: WifiNone,
                    }
                    <GLabel>{
                        text: "WifiNone",
                    }
                }
            }
        }
    }
}