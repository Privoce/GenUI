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
                height: 32.0,
                width: 200.0,
                background_visible: false,
                <GInput>{
                    height: Fill,
                    width: Fill,
                    text: "esdsadasdsa"
                }
            }
            <GCard>{
                height: 36.0,
                width: 200.0,
                background_visible: false,
                <GInput>{
                    spread_radius: 2.0,
                    shadow_offset: vec2(2.0, 2.0),
                    cursor_width: 3.0,
                    border_width: 1.0,
                    theme: Error,
                    height: 32.0,
                    width: Fill,
                    placeholder: "Place Input"
                }
            }
        }
    }
}