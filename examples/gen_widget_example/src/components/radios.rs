use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    import gen_components::components::*;

    GRadioExample = <ScrollYView>{
        height: 60.0,
        width: Fill,
        spacing: 10.0,
        flow: Down,
        <GLabel>{
            text: "GRadio"
        }
        <GHLayout>{
            height: Fit,
            width: Fill,
            spacing: 10.0,
            <GRadio>{
                height: 30.0,
                width: 60.0,
            }
            <GRadio>{
                theme: Warning,
                height: 30.0,
                width: 60.0,
                radio_type: Tick,
            }
            <GRadio>{
                theme: Warning,
                height: 30.0,
                width: 60.0,
                radio_type: Tick,
                background_visible: false,
            }
            <GRadio>{
                theme: Success,
                height: 30.0,
                width: 60.0,
                size: 12.0,
                scale: 0.6,
                border_width: 2.0,
                radio_type: Round,
                background_color: #000,
                selected_color: #42A5F5,
                hover_color: #FF7043,
                border_color: #76828F,
            }
            <GRadio>{
                theme: Dark,
                height: 30.0,
                width: 60.0,
                size: 12.0,
                radio_type: Tick,
            }
            <GRadio>{
                theme: Error,
                height: 30.0,
                width: 60.0,
               
                radio_type: Cross,
            }
        }
    }
}