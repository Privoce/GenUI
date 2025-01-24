use makepad_widgets::*;

live_design! {
    use link::widgets::*;
     
    use link::gen_components::*;

    GRadioExample = <ScrollYView>{
        height: 60.0,
        width: Fill,
        spacing: 10.0,
        flow: Down,
        padding: 6.0,
        <GLabel>{
            text: "GRadio"
        }
        <GHLayout>{
            height: Fit,
            width: Fill,
            spacing: 10.0,
            
            <GRadioGroup>{
                selected: 0.0,
                a = <GRadio>{
                    text: "GRadio",
                    // value: true,
                }
                b = <GRadio>{
                    theme: Warning,
                    radio_type: Tick,
                    text: "GRadio"
                }
            }
            <GRadioGroup>{
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
}