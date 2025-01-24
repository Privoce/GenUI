mod virtuals;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
     
    use link::gen_components::*;

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
                width: 480.0,
                header: {
                    height: Fit,
                    width: 480.0,
                    <GTRow>{
                        height: 32.0,
                        width: Fit,
                        <GTCell>{
                            height: Fill,
                            width: 160.0,
                            <GLabel>{
                                color: #667085,
                                text: "Table header 1",
                            }  
                        }
                        <GTCell>{
                            height: Fill,
                            width: 160.0,
                            <GLabel>{
                                color: #667085,
                                text: "Table header 2",
                            }
                        }
                        <GTCell>{
                            height: Fill,
                            width: 160.0,
                            <GLabel>{
                                color: #667085,
                                text: "Table header 3",
                            }
                        }
                    }
                }
                body: <GTBody>{
                    height: 120.0,
                    width: Fill,
                    <GTRow>{
                        height: 32.0,
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
                        height: 32.0,
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