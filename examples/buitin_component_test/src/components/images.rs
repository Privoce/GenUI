use makepad_widgets::*;

live_design! {
    use link::widgets::*;
     
    use link::gen_components::*;
    use makepad_draw::shader::std::*;
    GImageExample = <ScrollYView>{
        height: 120.0,
        width: Fill,
        spacing: 20.0,
        flow: Down,
        <GLabel>{
            text: "GImage",
        }
        <GHLayout>{
            height: Fit,
            width: Fill,
            spacing: 12.0,
            padding: {top: 16.0},
            <GImage>{
                height: 32.0,
                width: 36.0,
                src: Live(dep("crate://self/resources/rust.png")),
                rotation:30.0,
            }
            <GImage>{
                rotation: 1.0,
                opacity: 0.6,
                src: Live(dep("crate://self/resources/robius.png")),
            }
            <GImage>{
                scale: 0.6,
                src: Live(dep("crate://self/resources/robius.png")),
            }
            <GImage>{
                scale: 2.0,
                src: Live(dep("crate://self/resources/robius.png")),
            }
        }
    }
}