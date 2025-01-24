use makepad_widgets::*;

live_design! {
    use link::widgets::*;
     
    use link::gen_components::*;

    GDividerExample = <ScrollYView>{
        height: 180.0,
        width: Fill,
        flow: Down,
        spacing: 10.0,
        <Label>{
            text: "GDivider",
        }
        <GVLayout>{
            height: Fit,
            width: 300,
            spacing: 10.0,
            
            <GDivider>{
                height: 2.0,
                width: Fill;
                stroke_width: 1.0,
            }
            <GDivider>{
                theme: Dark,
                height: 40.0;
                width: Fill;
                <GView>{
                    height: Fit,
                    width: Fit,
                    background_color: #888888,
                    padding: 2.0,
                    border_radius: 0.0,
                    <GLabel>{
                        text: "Hello",
                        color: #0,
                        margin: 0.0,
                    }
                }
            }
            <GDivider>{
                theme: Error,
                height: Fit;
                width: Fill;
                <GButton>{}
            }
            // <GHLayout>{
            //     height: Fit,
            //     width: 300.0,
            //     spacing: 0.0,
            //     align: {
            //         x: 0.5,
            //         y: 0.5,
            //     },
            //     <GView>{
            //         height: 4.0,
            //         width: 60.0,
            //         border_radius: 0.0,
            //     }
            //     <GLabel>{
            //         text: "Hello",
            //     }
            //     <GView>{
            //         height: 4.0,
            //         width: 60.0,
            //         border_radius: 0.0,
            //     }
            // }
        }
    }
}