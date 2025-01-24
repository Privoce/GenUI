use makepad_widgets::*;

live_design! {
    use link::widgets::*;
     
    use link::gen_components::*;

    GProgressExample = <ScrollYView>{
        height: 200.0,
        width: Fill,
        spacing: 10.0,
        flow: Down,
        <GLabel>{
            text: "GProgress"
        }
        <GProgress>{
            value: 0.5,
        }
        <GProgress>{
            theme: Dark,
            height: 20.0,
            border_radius: 2.0,
            value: 0.36,
            read_only: false,
        }
        <GHLayout>{
            height: Fit,
            width: Fill,
            spacing: 10.0,
            <GProgress>{
                progress_type: Vertical,
                height: 200.0,
                width: 16.0,
                value: 0.8,
                read_only: false,
            }
            <GProgress>{
                theme: Error,
                visible: true,
                background_visible: false,
                progress_type: Vertical,
                height: 200.0,
                width: 16.0,
                value: 0.8,
                read_only: false,
            }
        }
    }
}