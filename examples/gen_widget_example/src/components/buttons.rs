use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    import gen_components::components::*;

    GButtonExample = <ScrollYView>{
        height: 80.0,
        width: Fill,
        flow: Down,
        spacing: 10.0,
        <Label>{
            text: "GButton"
        }
        <GHLayout>{
            height: Fit,
            width: Fill,
            spacing: 6.0,
            <GButton>{}
            <GButton>{theme: Error}
            <GButton>{theme: Warning}
            <GButton>{
                slot: {
                    text: "Default Button"
                }
            }
            
            <GButton>{
                theme: Dark,
                slot: <GHLayout>{
                    height: Fit,
                    width: Fill,
                    align: {x: 0.5, y: 0.0},
                    spacing: 4.0,
                    <GIcon>{
                        theme: Dark,
                        height: 16.0,
                        width: 16.0,
                        icon_type: Code,
                        hover_color: #F00,
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
                border_radius: 11.0,
            }
            <GButton>{
                theme: Dark,
                border_width: 1.5,
                hover_color: #FF0000,
                pressed_color: #00FF00,
            }
        }
    }
}