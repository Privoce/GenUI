use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;

    IconLibEmoji = <ScrollYView>{
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
                    text: "Emoji",
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
                        icon_type: Emoji,
                    }
                    <GLabel>{
                        text: "Emoji",
                    }
                }
            
                <GVLayout>{
                    height: Fit,
                    spacing: 0.0,
                    width: 90.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: Hot,
                    }
                    <GLabel>{
                        text: "Hot",
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 0.0,
                    width: 90.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: Heart,
                    }
                    <GLabel>{
                        text: "Heart",
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 0.0,
                    width: 90.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: HeartBroken,
                    }
                    <GLabel>{
                        text: "HeartBroken",
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 0.0,
                    width: 90.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: Dislike,
                    }
                    <GLabel>{
                        text: "Dislike",
                    }
                }
            }
        }
    }
}
