use makepad_widgets::*;

live_design! {
    use link::widgets::*;
     
    use link::gen_components::*;

    GLinkExample = <ScrollYView>{
        height: 100.0,
        width: Fill,
        spacing: 10.0,
        flow: Down,
        <GLabel>{
            text: "GLink"
        }
        <GHLayout>{
            height: Fit,
            width: Fill,
            spacing: 6.0,
            <GLink>{
                text: "Link",
            }
            <GLink>{
                theme: Dark,
                text: "Theme Dark",
            }
            <GLink>{
                theme: Error,
                text: "Define hover color and pressed color",
                hover_color: #FF00FF,
                pressed_color: #00FF00,
            }
            <GLink>{
                width: 200.0,
                theme: Success,
                text: "No underline",
                underline: false,
                align: {x: 0.4, y: 0.5},
            }
            <GLink>{
                theme: Warning,
                text: "Custom More",
                font_size: 12.0,
                hover_color: #FF00FF,
                background_color: #00FF00,
                margin: 10.0,
                padding: 10.0,
                color: #FF0000,
                underline_width: 2.0,
                font_family: dep("E:/Rust/try/makepad/Gen-UI/examples/gen_widget_example/resources/GoNotoKurrent-Bold.ttf"),
            }
        }
    }
}