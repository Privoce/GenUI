use makepad_widgets::*;

live_design! {
    use link::widgets::*;
     
    use link::gen_components::*;

    GTabsExample = <GVLayout>{
        height: 400.0,
        width: Fill,
        scroll_bars: <GScrollBars> {}
        spacing: 10.0,
        <GTab>{
            height: 300.0,
            width: Fill,
            body: <GTabPane>{
                height: Fill,
                width: Fill,
        
                <GTabBody>{
                    text: "Tab1",
                    height: 100.0,
                    width: Fill,
                    <GLabel>{
                        text: "GTabBody1",
                    }
                    <GButton>{}
                    <GStateNoMsg>{
                        height: 100.0,
                    }
                }
                <GTabBody>{
                    text: "Tab2",
                    height: 100.0,
                    width: Fill,
                    <GLabel>{
                        text: "GTabBody2",
                    }

                }
            }
        }
        <GTabBody>{
            height: 200.0,
            width: Fill,
            <GLabel>{
                text: "GTabBody",
            }
            <GButton>{}
            <GStateNoMsg>{
                height: 100.0,
            }
        }

        <GVLayout>{
            height: 200.0,
            spacing: 4.0,
            scroll_bars: <GScrollBars> {}
            <GHLayout>{
                height: Fit,
                spacing: 6.0,
                <GTabButton>{
                    plain: false,
                    selected: false,
                    show_msg_count: false,
                    text: "p: false, s: false, m: false"
                }
                <GTabButton>{
                    plain: true,
                    selected: false,
                    show_msg_count: false,
                    closeable: false,
                    text: "p: true, s: false, m: false"
                }
                <GTabButton>{
                    plain: true,
                    selected: true,
                    show_msg_count: false,
                    text: "p: true, s: true, m: false"
                }
            }
            <GHLayout>{
                height: Fit,
                spacing: 6.0,
                <GTabButton>{
                    plain: false,
                    selected: false,
                    show_msg_count: true,
                    text: "p: false, s: false, m: true"
                }
                <GTabButton>{
                    plain: false,
                    selected: true,
                    show_msg_count: true,
                    text: "p: false, s: true, m: true"
                }
                <GTabButton>{
                    plain: true,
                    selected: true,
                    show_msg_count: true,
                    text: "p: true, s: true, m: true"
                }
            }
            <GHLayout>{
                height: Fit,
                spacing: 6.0,
                <GTabButton>{
                    theme: Dark,
                    msg_count: 99,
                    plain: false,
                    selected: true,
                    show_msg_count: true,
                    text: "theme: Dark"
                }
                <GTabButton>{
                    theme: Error,
                    plain: false,
                    selected: true,
                    show_msg_count: true,
                    text: "theme: Error"
                }
                <GTabButton>{
                    theme: Warning,
                    plain: true,
                    selected: true,
                    show_msg_count: true,
                    text: "theme: Warning"
                }
            }
            <GTabHeader>{
                width: 200.0,
                items: ["Home", "Makepad", "GenUI", "Rust"],
            }
            <GTabHeader>{
                selected: 1,
                width: 200.0,
                item: <GTabButton>{
                    plain: true,
                    show_msg_count: true,
                    closeable: true,
                }
                items: ["Home", "Makepad", "GenUI", "Rust"],
            }
        }
    }
}