use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;
    BOLD_FONT = dep("crate://self/resources/OPPOSans-Bold.ttf");
    CBox = <GVLayout>{
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