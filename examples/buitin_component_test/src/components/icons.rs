use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    import gen_components::components::*;

    GSvgExample = <ScrollYView>{
        height: 100.0,
        width: Fill,
        spacing: 10.0,
        flow: Down,
        <GLabel>{
            text: "GSvg",
        }
        <GHLayout>{
            height: Fit,
            width: Fill,
            spacing: 10.0,
            <GSvg>{
                animation_open: true,
                cursor: Help,
                src: dep("crate://self/resources/lightning.svg"),
            }
            <GSvg>{
                theme: Dark,
                src: dep("crate://self/resources/config.svg"),
            }
            <GSvg>{
                theme: Error,
                src: dep("crate://self/resources/lightning.svg"),
            }
            <GSvg>{
                theme: Warning,
                src: dep("crate://self/resources/lightning.svg"),
            }
            <GSvg>{
                height: 60,
                width: 160,
                cursor: Help,
                color: #fff,
                src: dep("crate://self/resources/logo_makepad.svg"),
            }
        }
        
    }
}