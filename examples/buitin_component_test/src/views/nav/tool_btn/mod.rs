use makepad_widgets::Cx;

pub fn register(cx: &mut Cx) {
    self::live_design(cx);
}

use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub ToolBtnPage = {{ToolBtnPage}}{
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
        <GHLayout>{
            height: Fit,
            align: {x: 0.5},
            <GLabel>{
                font_size: 14.0,
                font_family: (BOLD_FONT),
                text: "ToolBtn",
            }
        }
        <GLabel>{
            width: Fill,
            text: "ToolButton usually use in Window Component. You can set os_type and icon_type.",
        }
        <CBox>{
            box_wrap = {
                spacing: 16.0,
                flow: Right,
                min = <GToolButton>{
                    os_type: Mac,
                    icon_type: Min
                }
                max = <GToolButton>{
                    os_type: Mac,
                    icon_type: Max
                }
                full = <GToolButton>{
                    os_type: Mac,
                    icon_type: FullScreen
                }
                close = <GToolButton>{
                    os_type: Mac,
                    icon_type: Close
                }
                lb = <GLabel>{
                    text: ""
                }
            }
            code = {
                body: {
                    <GVLayout>{
                        height: 240.0,
                        scroll_bars: <GScrollBars>{}
                        <GLabel>{
                            theme: Dark,
                            width: Fill,
                            text: r#"
                            <GToolButton>{
                                os_type: Mac,
                                icon_type: Min
                            }
                            <GToolButton>{
                                os_type: Mac,
                                icon_type: Max
                            }
                            <GToolButton>{
                                os_type: Mac,
                                icon_type: FullScreen
                            }
                            <GToolButton>{
                                os_type: Mac,
                                icon_type: Close
                            }
                            fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
                                let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
                                let min = self.gtool_button(id!(min));
                                let max = self.gtool_button(id!(max));
                                let full = self.gtool_button(id!(full));
                                let close = self.gtool_button(id!(close));
                                let lb = self.glabel(id!(lb));

                                if let Some(e) = min.clicked(&actions) {
                                    lb.set_text(cx, &format!("{}", e.icon_type));
                                }

                                if let Some(e) = max.clicked(&actions) {
                                    lb.set_text(cx, &format!("{}", e.icon_type));
                                }

                                if let Some(e) = full.clicked(&actions) {
                                    lb.set_text(cx, &format!("{}", e.icon_type));
                                }
                                if let Some(e) = close.clicked(&actions) {
                                    lb.set_text(cx, &format!("{}", e.icon_type));
                                }
                            }
                            "#;
                        }
                    }
                }
            }
        }
        <CBox>{
            box_wrap = {
                spacing: 12.0,
                <GHLayout>{
                    height: Fit,
                    align: {y: 0.5},
                    <GLabel>{
                        width: 90.0,
                        text: "MacOs:",
                        margin: {right: 12.0},
                    }
                    <GHLayout>{
                        height: Fit,
                        spacing: 16.0,
                        <GToolButton>{
                            os_type: Mac,
                            icon_type: Min
                        }
                        <GToolButton>{
                            os_type: Mac,
                            icon_type: Max
                        }
                        <GToolButton>{
                            os_type: Mac,
                            icon_type: FullScreen
                        }
                        <GToolButton>{
                            os_type: Mac,
                            icon_type: Close
                        }
                    }
                }
                <GHLayout>{
                    height: Fit,
                    align: {y: 0.5},
                    <GLabel>{
                        width: 90.0,
                        text: "Linux:",
                        margin: {right: 12.0},
                    }
                    <GHLayout>{
                        height: Fit,
                        spacing: 16.0,
                        <GToolButton>{
                            os_type: Linux,
                            icon_type: Min
                        }
                        <GToolButton>{
                            os_type: Linux,
                            icon_type: Max
                        }
                        <GToolButton>{
                            os_type: Linux,
                            icon_type: FullScreen
                        }
                        <GToolButton>{
                            os_type: Linux,
                            icon_type: Close
                        }
                    }
                }
                <GHLayout>{
                    height: Fit,
                    align: {y: 0.5},
                    <GLabel>{
                        width: 90.0,
                        text: "Windows:",
                        margin: {right: 16.0},
                    }
                    <GHLayout>{
                        height: Fit,
                        spacing: 16.0,
                        <GToolButton>{
                            os_type: Windows,
                            icon_type: Min
                        }
                        <GToolButton>{
                            os_type: Windows,
                            icon_type: Max
                        }
                        <GToolButton>{
                            os_type: Windows,
                            icon_type: FullScreen
                        }
                        <GToolButton>{
                            os_type: Windows,
                            icon_type: Close
                        }
                    }
                }
            }
            code = {
                body: {
                    <GVLayout>{
                        height: 240.0,
                        scroll_bars: <GScrollBars>{}
                        <GLabel>{
                            theme: Dark,
                            width: Fill,
                            text: r#"
                <GHLayout>{
                    height: Fit,
                    align: {y: 0.5},
                    <GLabel>{
                        width: 90.0,
                        text: "MacOs:",
                        margin: {right: 12.0},
                    }
                    <GHLayout>{
                        height: Fit,
                        spacing: 16.0,
                        <GToolButton>{
                            os_type: Mac,
                            icon_type: Min
                        }
                        <GToolButton>{
                            os_type: Mac,
                            icon_type: Max
                        }
                        <GToolButton>{
                            os_type: Mac,
                            icon_type: FullScreen
                        }
                        <GToolButton>{
                            os_type: Mac,
                            icon_type: Close
                        }
                    }
                }
                <GHLayout>{
                    height: Fit,
                    align: {y: 0.5},
                    <GLabel>{
                        width: 90.0,
                        text: "Linux:",
                        margin: {right: 12.0},
                    }
                    <GHLayout>{
                        height: Fit,
                        spacing: 16.0,
                        <GToolButton>{
                            os_type: Linux,
                            icon_type: Min
                        }
                        <GToolButton>{
                            os_type: Linux,
                            icon_type: Max
                        }
                        <GToolButton>{
                            os_type: Linux,
                            icon_type: FullScreen
                        }
                        <GToolButton>{
                            os_type: Linux,
                            icon_type: Close
                        }
                    }
                }
                <GHLayout>{
                    height: Fit,
                    align: {y: 0.5},
                    <GLabel>{
                        width: 90.0,
                        text: "Windows:",
                        margin: {right: 16.0},
                    }
                    <GHLayout>{
                        height: Fit,
                        spacing: 16.0,
                        <GToolButton>{
                            os_type: Windows,
                            icon_type: Min
                        }
                        <GToolButton>{
                            os_type: Windows,
                            icon_type: Max
                        }
                        <GToolButton>{
                            os_type: Windows,
                            icon_type: FullScreen
                        }
                        <GToolButton>{
                            os_type: Windows,
                            icon_type: Close
                        }
                    }
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
pub struct ToolBtnPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for ToolBtnPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for ToolBtnPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        let min = self.gtool_button(id!(min));
        let max = self.gtool_button(id!(max));
        let full = self.gtool_button(id!(full));
        let close = self.gtool_button(id!(close));
        let lb = self.glabel(id!(lb));

        if let Some(e) = min.clicked(&actions) {
            lb.set_text(cx, format!("{}", e.icon_type));
        }

        if let Some(e) = max.clicked(&actions) {
            lb.set_text(cx, format!("{}", e.icon_type));
        }

        if let Some(e) = full.clicked(&actions) {
            lb.set_text(cx, format!("{}", e.icon_type));
        }
        if let Some(e) = close.clicked(&actions) {
            lb.set_text(cx, format!("{}", e.icon_type));
        }
    }
}
