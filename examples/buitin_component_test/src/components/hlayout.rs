use makepad_widgets::*;

live_design! {
    use link::widgets::*;
     
    use link::gen_components::*;

    GHLayoutExample = <ScrollYView>{
        height: 100.0,
        width: Fill,
        flow: Down,
        spacing: 10.0,
        <Label>{
            text: "GHLayout",
        }
        <GHLayout>{
            height: Fit,
            width: 300,
            background_color: #FFFFFF,
            spacing: 10.0,
            <GLabel>{
                text: "Hello",
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