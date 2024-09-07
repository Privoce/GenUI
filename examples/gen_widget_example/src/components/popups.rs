use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;

    GPopupExample = <ScrollYView>{
        height: 180.0,
        width: Fill,
        spacing: 10.0,
        flow: Down,
        <GLabel>{
            text: "GPopup"
        }
        <GDropDown>{

            height: Fit,
            width: Fit,
            trigger = <GButton>{text:"open"},
            popup :<GPopup> {
                height: 150.0,
                width: 200.0,
                container: <GPopupContainer> {
                    height: Fill,
                    width: Fill,
                    flow: Down,
                    spacing: 10.0,
                    padding: 10.0,
                    <GLabel>{
                        text:"This is a popup",
                    }
                    <GButton>{
                        theme: Dark,
                        text: "Options"
                    }
                    <View>{
                        show_bg: true,
                        draw_bg: {color: #f00},
                        height: 40.0,
                        width: 40.0,
                    }
                }
            }
        }
        <GDropDown>{
            mode: ToolTip,
            height: Fit,
            width: Fit,
            trigger = <GButton>{text:"open"},
            popup :<GToolTip> {
                height: 150.0,
                width: 200.0,
                container: {
                    height: Fill,
                    width: Fill,
                    flow: Down,
                    spacing: 10.0,
                    padding: 10.0,
                    <GLabel>{
                        text:"This is a popup",
                    }
                    <GButton>{
                        theme: Dark,
                        text: "Options"
                    }
                    <View>{
                        show_bg: true,
                        draw_bg: {color: #f00},
                        height: 40.0,
                        width: 40.0,
                    }
                }
            }
        }
        <GDropDown>{
            mode: Dialog,
            height: Fit,
            width: Fit,
            trigger = <GButton>{text:"open"},
            popup :<GDialog> {
                container: {
                    height: 200.0,
                    width: 300.0,
                    flow: Down,
                    spacing: 10.0,
                    padding: 10.0,
                    <GLabel>{
                        text:"This is a popup",
                    }
                    <GButton>{
                        theme: Dark,
                        text: "Options"
                    }
                    <View>{
                        show_bg: true,
                        draw_bg: {color: #f00},
                        height: 40.0,
                        width: 40.0,
                    }
                }
            }
        }
    }
}
