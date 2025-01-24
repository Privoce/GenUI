use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;

    pub IconLibPerson = <ScrollYView>{
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
                    text: "Person",
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
                        icon_type: Male,
                    }
                    <GLabel>{
                        text: "Male",
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 0.0,
                    width: 90.0,
                    align: {x: 0.5},
                    <GIcon>{
                        icon_type: Female,
                    }
                    <GLabel>{
                        text: "Female",
                    }
                }
            }
        }
    }
}
