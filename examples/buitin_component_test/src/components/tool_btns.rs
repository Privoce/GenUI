use makepad_widgets::*;

live_design! {
    use link::widgets::*;
     
    use link::gen_components::*;
    
    GToolButtonExample = <ScrollYView>{
        height: 60.0,
        width: Fill,
        flow: Down,
        spacing: 10.0,
        <Label>{
            text: "GToolButton"
        }
        <GVLayout>{
            height: Fit,
            width: Fill,
            spacing: 6.0,
            <GHLayout>{
                spacing: 6.0,
                height: Fit,
                width: Fit,
                <GToolButton>{
                    os_type: Mac,
                    icon_type: Min
                }
                <GToolButton>{
                    os_type: Mac,
                    icon_type: Max
                }
                <GToolButton>{
                    os_type: Linux,
                    icon_type: FullScreen
                }
                <GToolButton>{
                    os_type: Linux,
                    icon_type: Close
                }
                // if os type is windows or other, you need wrap a card or view
                <GToolButton>{
                    icon_type: Min
                }
                <GToolButton>{
                    icon_type: Max
                }
                <GToolButton>{
                    icon_type: FullScreen
                }
                <GToolButton>{
                    icon_type: Close
                }
            }
            
        }
    }
}