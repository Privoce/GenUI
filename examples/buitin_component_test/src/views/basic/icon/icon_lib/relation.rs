use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;

    IconLibRelation = <ScrollYView>{
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
                    text: "Relation",
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
                        icon_type: Connect,
                    }
                    <GLabel>{
                        text: "Connect",
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 0.0,
                    width: 90.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: Disconnect,
                    }
                    <GLabel>{
                        text: "Disconnect",
                    }
                }
            }
        }
    }
}
