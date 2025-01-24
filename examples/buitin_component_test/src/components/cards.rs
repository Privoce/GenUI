use makepad_widgets::*;

live_design! {
    use link::widgets::*;
     
    use link::gen_components::*;

    GViewExample = <ScrollYView>{
        height: 200.0,
        width: Fill,
        flow: Down,
        spacing: 10.0,
        <Label>{
            text: "GView",
        }
        <GHLayout>{
            height:Fit,
            width: Fill,
            spacing: 10.0,
            <GView>{
                height: 30.0,
                width: 30.0,
            }
            <GView>{
                theme: Dark,
                height: 30.0,
                width: 30.0,
            }
            <GView>{
                theme: Error,
                height: 30.0,
                width: 30.0,
            }
            <GView>{
                theme: Warning,
                height: 30.0,
                width: 30.0,
                animation_open: true,
                blur_radius: 4.0,
                spread_radius: 5.0,
                clip_x: false,
                clip_y: false,
                cursor: Hand
            }
            <GView>{
                theme: Success,
                height: 30.0,
                width: 160.0,
                cursor: Help,
                align: {x: 0.5, y: 0.5},
                <GLabel>{
                    text: "cursor: Help",
                }
            }
            <GView>{
                theme: Error,
                height: Fit,
                width: 180.0,
                background_visible: true,
                border_width: 1.0,
                border_radius: 0.0,
                align: {x: 0.5, y: 0.5},
                <GLabel>{
                    margin: 20.0,
                    text: "Transparent GView",
                }
            }
            <GView>{
                theme: Success,
                height: 60.0,
                width: 60.0,
                border_color: #FF0000,
                border_width: 1.0,
                border_radius: 15.0,
            }
        }
        
        
        <GHLayout>{
            height:Fit,
            width: Fill,
            spacing: 10.0,
            <GView>{
                height: Fit,
                width: 300,
                flow: Down,
                background_color: #FFFFFF,
                spacing: 10.0,
                <GLabel>{
                    text: "GView flow Down",
                    color: #0,
                    margin: 10.0,
                }
                <GView>{
                    theme: Error,
                    height: 30.0,
                    width: 30.0,
                }
                <GView>{
                    theme: Warning,
                    height: 30.0,
                    width: 30.0,
                }
                <GButton>{
                    
                }
            }
            
            <GView>{
                height: 100.0,
                width: 300,
                flow: Down,
                background_color: #FF0000,
                spacing: 10.0,
                // background_visible: true,
                scroll_bars: <GScrollBars> {}
                <GLabel>{
                    text: "Card can scroll",
                    color: #0,
                    margin: 10.0,
                }
                <GView>{
                    theme: Error,
                    height: 30.0,
                    width: 30.0,
                }
                <GView>{
                    theme: Warning,
                    height: 30.0,
                    width: 30.0,
                }
                <GButton>{
                    
                }
            }
        }
    }
}