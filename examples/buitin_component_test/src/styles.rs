use makepad_widgets::*;

live_design!{
    use link::widgets::*;
    
    use link::gen_components::*;
    pub BOLD_FONT = dep("crate://self/resources/OPPOSans-Bold.ttf");
    pub CBox = <GVLayout>{
        height: Fit,
        box_wrap = <GView>{
            height: Fit,
            padding: 16.0,
            width: Fill,
            flow: Down,
        }
        code = <GCollapse>{
            width: Fill,
            height: Fit,
            header: {
                padding: 12.0,
                width: Fill,
                <GIcon>{
                    height: 12.0,
                    width: 12.0,
                    icon_type: Code,
                }
                <GLabel>{
                    text: "Example Code"
                }
            },
            body: {
                height: Fit,
                width: Fill,
                scroll_bars: <GScrollBars>{},
    
            }
        }
    }
}