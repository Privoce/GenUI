use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    import gen_components::components::*;

    GCollapseExample = <ScrollYView>{
        height: 400.0,
        width: Fill,
        flow: Down,
        spacing: 10.0,
        <Label>{
            text: "GCollapse",
        }
        <GVLayout>{
            spacing: 6.0,
            height: Fill,
            width: Fill,
            <GHLayout>{
                height: Fit,
                width: Fill,
                spacing: 6.0,
                <GCollapse>{
                    height: 100.0,
                    width: 300.0,
                    header: {
                        height: Fit,
                        <GLabel>{
                            text: "Open Collapse !!!",
                        }
                    },
                    body: {
                        height: Fit,
                        // width: 240.0,
                        theme: Dark,
                        <GButton>{
                            theme: Error,
                            slot:{
                                text: "Close!"
                            }
                        }
                    }
                }
                <GCollapse>{
                    height: 100.0,
                    width: 300.0,
                    opened: true,
                    position: Right,
                    header: {
                        <GLabel>{
                            text: "Right",
                        }
                        width: 60.0,
                        height: Fill,
                    }
                    body: {
                        height: Fill,
                        width: 240.0,
                        theme: Dark,
                        <GButton>{
                            theme: Error,
                            slot:{
                                text: "Close!"
                            }
                        }
                    }
                }
            }
            <GHLayout>{
                height: Fit,
                width: Fill,
                spacing: 6.0,
                <GCollapse>{
                    height: 180.0,
                    width: 300.0,
                    opened: false,
                    position: Top,
                    header: {
                        <GLabel>{
                            text: "Open Top",
                        }
                        width: Fill,
                        height: 40.0,
                    }
                    body: {
                        height: 140.0,
                        width: Fill,
                        theme: Dark,
                        
                    }
                }
                <GCollapse>{
                    height: 180.0,
                    width: 300.0,
                    opened: false,
                    position: Left,
                    header: {
                        <GLabel>{
                            text: "Left",
                        }
                        width: 60.0,
                        height: Fill,
                    }
                    body: {
                        height: Fill,
                        width: 240.0,
                        theme: Dark,
                        
                    }
                }
            }
        }
    }
}