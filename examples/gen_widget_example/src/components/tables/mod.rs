use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    import gen_components::components::*;

    GTableExample = <ScrollYView>{
        height: 200.0,
        width: Fill,
        flow: Down,
        spacing: 10.0,
        <Label>{
            text: "GTable",
        }
        <GVLayout>{
            spacing: 6.0,
            height: 200.0,
            width: Fill,
            <GTable>{
                height: Fit,
                width: Fill,
                body: <GTBody>{
                    height: 120.0,
                    width: Fill,
                    <GTRow>{
                        height: 60.0,
                        width: Fill,
                        <GTCell>{
                            height: Fill,
                            width: 160.0,
                            <GLabel>{
                                text: "Table column 1-1",
                            }  
                        }
                        <GTCell>{
                            height: Fill,
                            width: 160.0,
                            <GLabel>{
                                text: "Table column 1-2",
                            }
                        }
                        <GTCell>{
                            height: Fill,
                            width: 160.0,
                            <GLabel>{
                                text: "Table column 1-3",
                            }
                        }
                    }
                    <GTRow>{
                        height: 60.0,
                        width: Fill,
                        <GTCell>{
                            height: Fill,
                            width: 160.0,
                            <GLabel>{
                                text: "Table column 2-1",
                            }  
                        }
                        <GTCell>{
                            height: Fill,
                            width: 160.0,
                            <GLabel>{
                                text: "Table column 2-2",
                            }
                        }
                        <GTCell>{
                            height: Fill,
                            width: 160.0,
                            <GLabel>{
                                text: "Table column 2-3",
                            }
                        }
                    }
                }
            }
        }
    }
}