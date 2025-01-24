use makepad_widgets::*;

live_design! {
    use link::widgets::*;
     
    use link::gen_components::*;

    GTagExample = <ScrollYView>{
        height: 200.0,
        width: Fill,
        flow: Down,
        spacing: 10.0,
        <Label>{
            text: "GTag",
        }
        <GVLayout>{
            spacing: 6.0,
            height: 100.0,
            width: Fill,
            <GTag>{
                text: "badget!",
                spread_radius: 4.0,
            }
            <GTag>{
                round: true,
                theme: Success,
                text: "badge tag1",
                animation_open: true,
            }
            <GHLayout>{
                spacing: 6.0,
                <GTag>{
                    theme: Dark,
                    src: dep("crate://self/resources/config.svg"),
                    text: "other",
                }
                <GTag>{
                    theme: Error,
                    text: "closeable",
                    closeable: true
                }
                <GTag>{
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