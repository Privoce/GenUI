use makepad_widgets::*;

live_design! {
    use link::widgets::*;
     
    use link::gen_components::*;

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