use makepad_widgets::*;

live_design! {
    use link::widgets::*;
     
    use link::gen_components::*;
    
    GButtonExample = <ScrollYView>{
        height: 120.0,
        width: Fill,
        flow: Down,
        spacing: 10.0,
        <Label>{
            text: "GButton"
        }
        <GVLayout>{
            height: Fit,
            width: Fill,
            spacing: 6.0,
            <GHLayout>{
                height: Fit,
                width: Fill,
                spacing: 6.0,
                <Button>{
                    text: "makepad button"
                }
                <GButton>{}
                <GButton>{theme: Error}
                <GButton>{theme: Warning}
                <GButton>{theme: Success}
                <GButton>{theme: Dark}
                <GButton>{
                    slot: {
                        text: "Default Button"
                    }
                }

            }
            <GHLayout>{
                height: Fit,
                width: Fill,
                spacing: 6.0,
                <GButton>{
                    theme: Dark,
                    slot: <GHLayout>{
                        height: Fit,
                        width: Fill,
                        spacing: 4.0,
                        <GIcon>{
                            theme: Dark,
                            height: 16.0,
                            width: 18.0,
                            icon_type: Code,
                            stroke_width: 1.4,
                            animation_open: true,
                        }
                        <GIcon>{
                            theme: Dark,
                            height: 16.0,
                            width: 18.0,
                            icon_type: Add,
                            stroke_width: 1.4,
                            animation_open: true,
                        }
                        <GLabel>{
                            font_size: 12.0,
                            text: "DarkButton"
                        }
                    }
                }
                <GButton>{
                    height: 46,
                    width: 160,
                    theme: Success,
                    border_width: 1.4,
                    border_color: #FFF,
                    border_radius: 10.0,
                    blur_radius: 10.0, // if you donot want shadow, set bigger than 10.0
                }
                <GButton>{
                    theme: Dark,
                    border_width: 1.5,
                    hover_color: #FF0000,
                    focus_color: #00FF00,
                }
            }
        }
    }
}