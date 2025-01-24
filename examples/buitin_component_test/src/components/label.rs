use makepad_widgets::*;

live_design! {
    use link::widgets::*;
     
    use link::gen_components::*;

    GLabelExample = <ScrollYView>{
        height: 120.0,
        width: Fill,
        spacing: 10.0,
        flow: Down,
        <Label>{
            text: "GLabel"
        }
        <GHLayout>{
            height: Fit,
            width: Fill,
            spacing: 10.0,
            <GLabel>{
                text: "Hello, world! This is a long message, but I use wrap Word to wrap it!",
                height: Fit,
                width: 120.0,
                wrap: Word,
                margin: {left: 12.0},
            }
            <GLabel>{
                text: "test another font family!!",
                font_size: 12.0,
                padding: 16.0,
                color: #FF0000,
                // font_family: dep("crate://self/resources/GoNotoKurrent-Bold.ttf"),
                font_family: dep("crate://self/resources/AlimamaFangYuanTiVF-Thin.ttf"),
            }
        }
    }
}