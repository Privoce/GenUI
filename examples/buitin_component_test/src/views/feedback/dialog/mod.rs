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

    pub DialogPage = {{DialogPage}}{
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
                text: "Dialog",
            }

        }
        <GLabel>{
            width: Fill,
            text: "Dialog is a popup that can be used to show some information or ask for user input. It usually has a close button to close the dialog.",
        }
        <GLabel>{
            width: Fill,
            text: "But you need to define yourself, such as following the example below.",
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                flow: Right,
                dialog = <GDropDown>{
                    trigger = <GButton>{
                        slot: {
                            text:"Open Dialog"
                        }
                    },
                    popup :<GDialog> {
                        container: {
                            height: 220.0,
                            width: 360.0,
                            flow: Down,
                            spacing: 10.0,
                            padding: 10.0,
                            <GView>{
                                height: Fill,
                                width: Fill,
                                spread_radius: 4.6,
                                blur_radius: 4.6,
                                spacing: 12.0,
                                flow: Down,
                                clip_x: false,
                                clip_y: false,
                                padding: 16.0,
                                shadow_offset: vec2(0.0, 2.0),
                                header = <GHLayout>{
                                    height: 32.0,
                                    align: { y: 0.5},
                                    <GLabel>{
                                        text: "Dialog",
                                        font_size: 14.0,
                                        font_family: (BOLD_FONT),
                                    }
                                    <GHLayout>{
                                        align: {x: 1.0, y: 0.5},
                                        close_icon = <GIcon>{
                                            height: 10.0,
                                            width: 10.0,
                                            animation_key: true,
                                            stroke_hover_color: #FF0000,
                                            icon_type: Close,
                                        }
                                    }
                                }
                                body = <GVLayout>{
                                    height: Fill,
                                    <GLabel>{
                                        text: "This is a dialog",
                                    }
                                }
                                footer = <GHLayout>{
                                    height: 60.0,
                                    align: {x: 1.0, y: 0.5},
                                    spacing: 16.0,
                                    cancel = <GButton>{
                                        theme: Info,
                                        slot: {
                                            text: "Cancel"
                                        }
                                    }
                                    confirm = <GButton>{
                                        slot: {
                                            text: "Confirm"
                                        }
                                    }
                                }
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
                    popup :<GDialog> {
                        container: {
                            height: 220.0,
                            width: 360.0,
                            flow: Down,
                            spacing: 10.0,
                            padding: 10.0,
                            <GView>{
                                height: Fill,
                                width: Fill,
                                spread_radius: 4.6,
                                blur_radius: 4.6,
                                spacing: 12.0,
                                flow: Down,
                                clip_x: false,
                                clip_y: false,
                                padding: 16.0,
                                shadow_offset: vec2(0.0, 2.0),
                                header = <GHLayout>{
                                    height: 32.0,
                                    align: { y: 0.5},
                                    <GLabel>{
                                        text: "Dialog",
                                        font_size: 14.0,
                                        font_family: (BOLD_FONT),
                                    }
                                    <GHLayout>{
                                        align: {x: 1.0, y: 0.5},
                                        close_icon = <GIcon>{
                                            height: 12.0,
                                            width: 12.0,
                                            animation_key: true,
                                            stroke_hover_color: #FF0000,
                                            icon_type: Close,
                                        }
                                    }
                                }
                                body = <GVLayout>{
                                    height: Fill,
                                    <GLabel>{
                                        text: "This is a dialog",
                                    }
                                }
                                footer = <GHLayout>{
                                    height: 60.0,
                                    align: {x: 1.0, y: 0.5},
                                    spacing: 16.0,
                                    cancel = <GButton>{
                                        theme: Info,
                                        slot: {
                                            text: "Cancel"
                                        }
                                    }
                                    confirm = <GButton>{
                                        slot: {
                                            text: "Confirm"
                                        }
                                    }
                                }
                            }
                        }
                    }
                    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
                        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
                        let mut dialog = self.gdrop_down(id!(dialog));

                        dialog.get_mut(cx, |cx, dialog, container| {
                            let close_icon = container.gicon(id!(close_icon));
                            let cancel = container.gbutton(id!(cancel));
                            let confirm = container.gbutton(id!(confirm));

                            if close_icon.clicked(&actions).is_some()
                                || cancel.clicked(&actions).is_some()
                                || confirm.clicked(&actions).is_some()
                            {
                                dialog.close(cx);
                            }
                        });
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
pub struct DialogPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for DialogPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for DialogPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        let mut dialog = self.gdrop_down(id!(dialog));

        dialog.get_mut(cx, |cx, dialog, container| {
            let close_icon = container.gicon(id!(close_icon));
            let cancel = container.gbutton(id!(cancel));
            let confirm = container.gbutton(id!(confirm));

            if close_icon.clicked(&actions).is_some()
                || cancel.clicked(&actions).is_some()
                || confirm.clicked(&actions).is_some()
            {
                dialog.close(cx);
            }
        });
    }
}
