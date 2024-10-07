use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    import gen_components::components::*;

    GSelectExample = <ScrollYView>{
        height: 180.0,
        width: Fill,
        flow: Down,
        spacing: 10.0,
        <Label>{
            text: "GSelect",
        }
        <GVLayout>{
            height: Fit,
            width: 300,
            spacing: 10.0,
            
        }
    }
}