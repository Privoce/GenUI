use makepad_widgets::*;

live_design! {
    use link::widgets::*;
     
    use link::gen_components::*;

    GCheckboxExample = <ScrollYView>{
        height: 60.0,
        width: Fill,
        spacing: 10.0,
        flow: Down,
        <GLabel>{
            text: "GCheckbox"
        }
        <GHLayout>{
            height: Fit,
            width: Fill,
            spacing: 10.0,
            <GCheckbox>{
           
            }
            <GCheckbox>{
                theme: Warning,
                check_type: Tick,
            }
            <GCheckbox>{
                theme: Error,
                check_type: Cross,
            }
            <GCheckbox>{
                theme: Error,
                check_type: Cross,
                background_visible: false,
                value: true,
            }
            <GCheckbox>{
                theme: Success,
                height: 30.0,
                width: 30.0,
                size: 12.0,
                scale: 0.6,
                border_width: 2.0,
                check_type: Round,
                background_color: #000,
                selected_color: #42A5F5,
                hover_color: #FF7043,
                border_color: #76828F,
            }
            <GCheckbox>{
                theme: Dark,
                height: 30.0,
                width: 60.0,
                size: 12.0,
                check_type: Tick,
            }
        }
    }
}