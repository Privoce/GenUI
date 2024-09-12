use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    import gen_components::components::*;

    GCardExample = <ScrollYView>{
        height: 200.0,
        width: Fill,
        flow: Down,
        spacing: 10.0,
        <Label>{
            text: "GCard",
        }
        <GHLayout>{
            height:Fit,
            width: Fill,
            spacing: 10.0,
            <GCard>{
                height: 30.0,
                width: 30.0,
            }
            <GCard>{
                theme: Dark,
                height: 30.0,
                width: 30.0,
            }
            <GCard>{
                theme: Error,
                height: 30.0,
                width: 30.0,
            }
            <GCard>{
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
            <GCard>{
                theme: Success,
                height: 30.0,
                width: 160.0,
                cursor: Help,
                align: {x: 0.5, y: 0.5},
                <GLabel>{
                    text: "cursor: Help",
                }
            }
            <GCard>{
                theme: Error,
                height: Fit,
                width: 180.0,
                background_visible: true,
                border_width: 1.0,
                border_radius: 0.0,
                align: {x: 0.5, y: 0.5},
                <GLabel>{
                    margin: 20.0,
                    text: "Transparent GCard",
                }
            }
            <GCard>{
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
            <GCard>{
                height: Fit,
                width: 300,
                flow: Down,
                background_color: #FFFFFF,
                spacing: 10.0,
                <GLabel>{
                    text: "GCard flow Down",
                    color: #0,
                    margin: 10.0,
                }
                <GCard>{
                    theme: Error,
                    height: 30.0,
                    width: 30.0,
                }
                <GCard>{
                    theme: Warning,
                    height: 30.0,
                    width: 30.0,
                }
                <GButton>{
                    
                }
            }
            
            <GCard>{
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
                <GCard>{
                    theme: Error,
                    height: 30.0,
                    width: 30.0,
                }
                <GCard>{
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