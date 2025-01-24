use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    BOLD_FONT = dep("crate://self/resources/OPPOSans-Bold.ttf");
    pub InstallPage = {{InstallPage}}{
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
        <GLabel>{
            font_size: 14.0,
            font_family: (BOLD_FONT),
            text: "Install",
        }
        <GLabel>{
            font_size: 12.0,
            font_family: (BOLD_FONT),
            text: "Crates.io (Comming Soon)",
        }
        <GDivider>{
            theme: Info,
            height: 6.0,
        }
        <GVLayout>{
            height: Fit,
            spacing: 8.0,
            <GLabel>{
                font_size: 12.0,
                font_family: (BOLD_FONT),
                text: "Github",
            }
            <GLink>{
                text: "GenUI Builtin Components",
                href: "https://github.com/Privoce/GenUI/tree/components/gen/components"
            }
            <GLabel>{
                font_size: 10.0,
                font_family: (BOLD_FONT),
                text: "1. Add the following to your Cargo.toml file",
            }

            <GView>{
                height: Fit,
                width: Fill,
                flow: Right,
                align: {
                    y: 0.5
                },
                <GView>{
                    height: 52.0,
                    width: 4.0,
                    border_radius: 0.0,
                    background_color: #BF8E50,
                    margin: {
                        right: 8.0,
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 8.0,
                    <GLabel>{
                        text: r#"makepad-widgets = {git = "https://github.com/makepad/makepad.git", branch = "rik"} "#,
                    }
                    <GLabel>{
                        text: r#"gen_components = {git = "https://github.com/Privoce/GenUI.git/gen/components", branch = "components"} "#,
                    }
                }
            }
            <GLabel>{
                font_size: 10.0,
                font_family: (BOLD_FONT),
                text: "2. Import into Live Design",
            }
            <GView>{
                height: Fit,
                width: Fill,
                flow: Right,
                align: {
                    y: 0.5
                },
                <GView>{
                    height: 52.0,
                    width: 4.0,
                    border_radius: 0.0,
                    background_color: #BF8E50,
                    margin: {
                        right: 8.0,
                    }
                }
                <GLabel>{
                    text: r#"use link::gen_components::*;"#,
                }
            }
            <GLabel>{
                font_size: 10.0,
                font_family: (BOLD_FONT),
                text: "3. Register into LiveRegister",
            }
            <GView>{
                height: Fit,
                width: Fill,
                flow: Right,
                align: {
                    y: 0.5
                },
                <GView>{
                    height: 52.0,
                    width: 4.0,
                    border_radius: 0.0,
                    background_color: #BF8E50,
                    margin: {
                        right: 8.0,
                    }
                }
                <GLabel>{
                    text: r#" crate::gen_components::live_design(cx);"#,
                }
            }
        }

    }
}

#[derive(Live, Widget)]
pub struct InstallPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for InstallPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for InstallPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let _ = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
    }
}
