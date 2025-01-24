use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    BOLD_FONT = dep("crate://self/resources/OPPOSans-Bold.ttf");
    pub FontPage = {{FontPage}}{
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
            text: "GenUI Builtin Lib Font (2024-06-20)",
        }
        
        <GHLayout>{
            height: Fit,
            width: Fill,
            <GLabel>{
                text: "We use",
            }
            <GLink>{
                width: Fit,
                text: " OPPO Sans 3.0 ",
                href: "https://www.coloros.com/article/A00000050/"
            }
            <GLabel>{
                text: " `OPPOSans-Regular.ttf` as builtin font family.",
            }
        }
        <GLabel>{
            width: Fill,
            text: "• Commercial Use: ✓\n• Required Authorization: ✓\n• Secondary development: ✕",
        }
        <GLabel>{
            font_size: 12.0,
            font_family: (BOLD_FONT),
            text: "Follow the instructions",
        }
        <GLabel>{
            width: Fill,
            text: "1. Do not adapt or redevelop fonts.\n2. Do not sell fonts externally.\n3. Do not provide other download channels to others.\n4. Not used for illegal purposes.",
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
                margin: {
                    left: 8.0,
                    right: 8.0,
                },
                width: Fill,
                font_family: (BOLD_FONT),
                color: #BF8E50,
                text: "OPPO Sans（含中文及西文，3 款字重）允许个人或企业免费使用，含商业用途，版权归 OPPO 广东移动通信有限公司所有。",
            }
        } 
        <GVLayout>{
            height: Fit,
            spacing: 12.0,
            <GLabel>{
                font_size: 14.0,
                font_family: (BOLD_FONT),
                text: "Examples",
            }
            <GCollapse>{
                height: Fit,
                width: Fill,
                header: {
                    height: 32.0,
                    width: Fill,
                    <GLabel>{
                        text: "Define Global Font Family",
                    }
                },
                body: {
                    height: Fit,
                    theme: Dark,
                    <GInput>{
                        spread_radius: 0.0,
                        theme: Dark,
                        height: 32.0,
                        width: Fill,
                        text: r#"BOLD_FONT = dep("crate://self/resources/OPPOSans-Bold.ttf");"#,
                        read_only: true,
                    }
                }
            }
            <GCollapse>{
                height: Fit,
                width: Fill,
                header: {
                    height: 32.0,
                    width: Fill,
                    <GLabel>{
                        text: "Set Font Family",
                    }
                },
                body: {
                    height: Fit,
                    theme: Dark,
                    <GInput>{
                        spread_radius: 0.0,
                        theme: Dark,
                        height: 400.0,
                        width: Fill,
                        text: r#"
#[derive(Live, Widget)]
pub struct MyWidget {
    #[redraw]
    #[live]
    draw_text: DrawGText,
}

impl Widget for GLabel {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        // ...
        let font = get_font_family(&self.font_family, cx);
        self.draw_text.text_style.font = font;
        // ...
        DrawStep::done()
    }
}  
                        "#,
                        read_only: true,
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct FontPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for FontPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for FontPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let _ = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
    }
}