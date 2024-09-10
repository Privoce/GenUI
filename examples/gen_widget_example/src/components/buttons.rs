use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    import gen_components::components::*;

    GButtonExample = <ScrollYView>{
        height: 120.0,
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
            <GButton>{
                text: "Default Button"
            }
            <GButton>{
                theme: Dark,
                text: "Theme Dark",
            }
            <GButton>{
                theme: Success,
                text: "Theme Success"
            }
            <GButton>{
                theme: Warning,
                text: "Theme Warning"
            }
            <GButton>{
                theme: Error,
                text: "Theme Error",
            }
            <GButton>{
                theme: Error,
                text: "unvisible button!",
                visible: false,
            }
            <GButton>{
                round: true,
                text: "Round Button",
            }
        }
        
        <GHLayout>{
            height: Fit,
            width: Fill,
            spacing: 6.0,
            <GButton>{
                height: 46,
                width: 160,
                theme: Success,
                border_width: 1.4,
                border_color: #FFF,
                border_radius: 11.0,
                text: "Rounded Button!",
            }
            <GButton>{
                theme: Dark,
                border_width: 1.2,
                hover_color: #FF0000,
                pressed_color: #00FF00,
                text: "GButton!",
                font_family: dep("E:/Rust/try/makepad/Gen-UI/examples/gen_widget_example/resources/GoNotoKurrent-Bold.ttf"),
                font_size: 12.0,
                color: #000,
            }
        }
    }
}