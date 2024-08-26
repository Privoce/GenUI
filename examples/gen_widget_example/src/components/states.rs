use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    import gen_components::components::*;

    GStatesExample = <GVLayout>{
        height: 200.0,
        width: Fill,
        scroll_bars: <GScrollBars> {}
        <GHLayout>{
            height: 200.0,
            width: Fill,
            <GState404>{
                height: 200.0,
                width: Fill,
            }
            <GState502>{
                height: 200.0,
                width: Fill,
            }
        }
        <GHLayout>{
            height: 200.0,
            width: Fill,
            <GStateNoMsg>{
                height: 200.0,
                width: Fill,
            }
            <GStateNoData>{
                height: 200.0,
                width: Fill,
            }
        }
        <GHLayout>{
            height: 200.0,
            width: Fill,
            <GStateNetWorkErr>{
                height: 200.0,
                width: Fill,
            }
            <GStateSearch>{
                height: 200.0,
                width: Fill,
            }
        }
    }
}