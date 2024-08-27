use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    import gen_components::components::*;

    GBadgeExample = <ScrollYView>{
        height: 200.0,
        width: Fill,
        flow: Down,
        spacing: 10.0,
        <Label>{
            text: "GBadge",
        }
        <GVLayout>{
            spacing: 6.0,
            height: 100.0,
            width: Fill,
            <GBadge>{
                text: "badget!",
            }
            <GBadge>{
                round: true,
                theme: Success,
                text: "badge tag1",
            }
            <GHLayout>{
                spacing: 6.0,
                <GBadge>{
                    theme: Dark,
                    src: dep("crate://self/resources/config.svg"),
                    text: "other",
                }
                <GBadge>{
                    theme: Error,
                    text: "closeable",
                    closeable: true
                }
                <GBadge>{
                    theme: Warning,
                    round: true,
                    text: "icon-close-round",
                    src: dep("crate://self/resources/config.svg"),
                    closeable: true
                }
            }
        }
    }
}