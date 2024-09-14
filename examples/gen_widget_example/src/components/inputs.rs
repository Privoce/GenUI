use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    import gen_components::components::*;

    GInputExample = <ScrollYView>{
        height: 160.0,
        width: Fill,
        spacing: 10.0,
        flow:Down,
        <Label>{
            text: "GInput"
        }
        <GHLayout>{
            height: Fit,
            width: Fill,
            spacing: 20.0,
            <GCard>{
                height: 62.0,
                width: 200.0,
                background_visible: false,
                <GInput>{
                    height: Fill,
                }
            }
            <GCard>{
                height: 36.0,
                width: 200.0,
                <TextInput>{
                    height: Fill,
                    // border_radius: 4.0,
                    // border_width: 1.0,
                    // input_type: Pwd,
                }
            }
        }
    }
}