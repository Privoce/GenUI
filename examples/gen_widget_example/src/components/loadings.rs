use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    import gen_components::components::*;

    GLoadingExample = <ScrollYView>{
        height: 230.0,
        width: Fill,
        spacing: 10.0,
        flow: Down,
        <GLabel>{
            text: "GLoading"
        }
        <View>{
            height: 100.0,
            width: Fill,
            flow: Down,
            spacing: 16.0,
            align: {x: 0.5, y: 0.5},
            <GLoading>{
                height: 64.0,
                width: 64.0,
                theme: Error,
                animation_open: true,
                visible: true,
            }
            <GLabel>{
                text: "Loading ...",
            }
        }
        <View>{
            height: 180.0,
            width: Fill,
            flow: Down,
            spacing: 16.0,
            align: {x: 0.5, y: 0.5},
            <GLoading>{
                height: 64.0,
                width: 64.0,
                loading_type: CircleDot
            }
            <GLabel>{
                text: "Loading ...",
            }
        }
        <View>{
            height: 100.0,
            width: Fill,
            flow: Down,
            spacing: 16.0,
            align: {x: 0.5, y: 0.5},
            <GLoading>{
                loading_type: DotLine
            }
            <GLabel>{
                text: "Loading ...",
            }
        }
        
    }
}