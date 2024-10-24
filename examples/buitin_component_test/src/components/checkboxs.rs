use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    import gen_components::components::*;

    GCheckBoxExample = <ScrollYView>{
        height: 60.0,
        width: Fill,
        spacing: 10.0,
        flow: Down,
        <GLabel>{
            text: "GCheckBox"
        }
        <GHLayout>{
            height: Fit,
            width: Fill,
            spacing: 10.0,
            <GCheckBox>{
           
            }
            <GCheckBox>{
                theme: Warning,
                check_type: Tick,
            }
            <GCheckBox>{
                theme: Error,
                check_type: Cross,
            }
            <GCheckBox>{
                theme: Error,
                check_type: Cross,
                background_visible: false,
                value: true,
            }
            <GCheckBox>{
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
            <GCheckBox>{
                theme: Dark,
                height: 30.0,
                width: 60.0,
                size: 12.0,
                check_type: Tick,
            }
        }
    }
}