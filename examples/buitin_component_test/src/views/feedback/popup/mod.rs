use makepad_widgets::Cx;

// pub mod usage;
// pub mod animate;
// pub mod event;
// pub mod virt;

pub fn register(cx: &mut Cx) {
    self::live_design(cx);
}

use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub PopupPage = {{PopupPage}}{
        height: Fill,
        width: Fill,
        flow: Down,
        background_visible: false,
        border_radius: 0.0,
        spacing: 12.0,
        padding: 12.0,
        scroll_bars: <GScrollBars>{},
        clip_x: true,
        clip_y: true,
        <GVLayout>{
            height: Fit,
            align: {x: 0.5},
            <GLabel>{
                font_size: 14.0,
                font_family: (BOLD_FONT),
                text: "Popup",
            }

        }
        <GLabel>{
            width: Fill,
            text: "Popup is a floating window that can be used to display information or ask for user input. It can be used to display information or ask for user input.",
        }
        <CBox>{
            box_wrap = {
                spacing: 24.0,
                flow: Right,
                <GDropDown>{
                    offset: 6.0,
                    height: Fit,
                    width: Fit,
                    trigger = <GButton>{
                        slot: {
                            text:"Click to open"
                        }
                    },
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
                        }
                    }
                }
                <GDropDown>{
                    offset: 6.0,
                    height: Fit,
                    width: Fit,
                    position: Right,
                    trigger = <GButton>{
                        slot: {
                            text:"Position: Right"
                        }
                    },
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
                        }
                    }
                }
                <GDropDown>{
                    offset: 6.0,
                    height: Fit,
                    width: Fit,
                    trigger_mode: Hover,
                    trigger = <GButton>{
                        slot: {
                            text:"Hover to open"
                        }
                    },
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
                        }
                    }
                }
                <GDropDown>{
                    offset: 6.0,
                    height: Fit,
                    width: Fit,
                    trigger_mode: Press,
                    trigger = <GButton>{
                        slot: {
                            text:"Press to open"
                        }
                    },
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
                        }
                    }
                }
            }
            code = {
                body: {
                    <GVLayout>{
                        height: 300.0,
                        scroll_bars: <GScrollBars>{},
                        <GLabel>{
                            theme: Dark,
                            width: Fill,
                            text: r#"
                <GDropDown>{
                    offset: 6.0,
                    height: Fit,
                    width: Fit,
                    trigger = <GButton>{
                        slot: {
                            text:"Click to open"
                        }
                    },
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
                        }
                    }
                }
                <GDropDown>{
                    offset: 6.0,
                    height: Fit,
                    width: Fit,
                    position: Right,
                    trigger = <GButton>{
                        slot: {
                            text:"Position: Right"
                        }
                    },
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
                        }
                    }
                }
                <GDropDown>{
                    offset: 6.0,
                    height: Fit,
                    width: Fit,
                    trigger_mode: Hover,
                    trigger = <GButton>{
                        slot: {
                            text:"Hover to open"
                        }
                    },
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
                        }
                    }
                }
                            "#;
                        }
                    }
                }
            }
        }
        <GLabel>{
            text: "You can set nothing as trigger(use virtual trigger)",
        }
        <GLabel>{
            text: "You can set abs_pos to set the position of the popup",
        }
        <CBox>{
            box_wrap = {
                spacing: 24.0,
                flow: Right,
                height: 100.0,

                pop = <GDropDown>{
                    offset: 6.0,
                    height: Fit,
                    width: Fit,
                    abs_pos: vec2(300.0, 300.0),
                    popup :<GPopup> {
                        height: 150.0,
                        width: 200.0,
                        container: <GPopupContainer> {
                            height: Fill,
                            width: Fill,
                            flow: Down,
                            spacing: 10.0,
                            padding: 10.0,
                            close = <GButton>{
                                theme: Dark,
                                slot: {text:"close inner"}
                            }
                        }
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 16.0,
                    open = <GButton>{
                        slot: {
                            text:"virtual open"
                        }
                    }

                }
            }
            code = {
                body: {
                    <GVLayout>{
                        height: 300.0,
                        scroll_bars: <GScrollBars>{},
                        <GLabel>{
                            theme: Dark,
                            width: Fill,
                            text: r#"
        let mut pop = self.gdrop_down(id!(pop));
        let e_label = self.glabel(id!(e_label));
        let open = self.gbutton(id!(open));

        if open.clicked(&actions).is_some() {
            pop.open(cx);
        }
        pop.get_mut(cx, |cx, pop, container| {
            let close = container.gbutton(id!(close));

            if close.clicked(&actions).is_some() {
                pop.close(cx);
            }
        });
                            "#;
                        }
                    }
                }
            }
        }
        <GLabel>{
            text: "You can use changed callback",
        }
        <CBox>{
            box_wrap = {
                spacing: 24.0,
                
                pop2 = <GDropDown>{
                    offset: 6.0,
                    height: Fit,
                    width: Fit,
                    position: Top,
                    trigger = <GButton>{
                        slot: {
                            text:"Click to open"
                        }
                    },
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
                        }
                    }
                }
                e_label = <GLabel>{
                    text: "Popup: None",
                }
            }
            code = {
                body: {
                    <GVLayout>{
                        height: 300.0,
                        scroll_bars: <GScrollBars>{},
                        <GLabel>{
                            theme: Dark,
                            width: Fill,
                            text: r#"
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        let mut pop2 = self.gdrop_down(id!(pop2));
        if let Some(e) = pop2.changed(&actions) {
            e_label.set_text(cx, &format!("Popup: {:?}", e.opened));
        }
                            "#;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct PopupPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for PopupPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for PopupPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        let mut pop = self.gdrop_down(id!(pop));
        let mut pop2 = self.gdrop_down(id!(pop2));
        let e_label = self.glabel(id!(e_label));
        let open = self.gbutton(id!(open));

        if open.clicked(&actions).is_some() {
            pop.open(cx);
        }
        pop.get_mut(cx, |cx, pop, container| {
            let close = container.gbutton(id!(close));

            if close.clicked(&actions).is_some() {
                pop.close(cx);
            }
        });

        if let Some(e) = pop2.changed(&actions) {
            e_label.set_text(cx, format!("Popup: {:?}", e.opened));
        }
    }
}
