use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    import gen_components::components::*;

    GLoadingExample = <ScrollYView>{
        height: 180.0,
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
            align: {x: 0.5, y: 0.5},
            <GLoading>{
                theme: Error,
            }
            <GLabel>{
                text: "Loading ...",
            }
        }
        <View>{
            height: 100.0,
            width: Fill,
            flow: Down,
            align: {x: 0.5, y: 0.5},
            <GLoading>{
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