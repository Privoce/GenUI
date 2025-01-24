use makepad_widgets::*;

live_design! {
    use link::widgets::*;
     
    use link::gen_components::*;

    GToggleExample = <ScrollYView>{
        height: 60.0,
        width: Fill,
        spacing: 10.0,
        flow: Down,
        <GLabel>{
            text: "GToggle"
        }
        <GHLayout>{
            height: Fit,
            width: Fill,
            spacing: 12.0,
            <GToggle>{
           
            }
            <GToggle>{
                theme: Dark
            }
            <GToggle>{
                theme: Error,
                toggle_type: Rect,
                animation_open: false,
            }
            <GToggle>{
                theme: Dark,
                toggle_type: Rect,
            }
            <GToggle>{
                theme: Warning,
                value: true,
            }
            <GToggle>{
                theme: Warning,
                value: true,
                background_visible: false,
            }
            <GToggle>{
                theme: Success,
                value: true,
                toggle_type: Rect,
                background_visible: false,
                height: 28.0,
                width: 60.0
            }
        }
        
    }
}