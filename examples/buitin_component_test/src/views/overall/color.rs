use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;

    BOLD_FONT = dep("crate://self/resources/OPPOSans-Bold.ttf");
    pub ColorPage = {{ColorPage}}{
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
            text: "Theme Color",
        }
        <GLabel>{
            width: Fill,
            text: "GenUI Theme Color is the theme color of GenUI, which includes a series of color variables used to define the colors of components. We provide a default color scheme.",
        }
        <GLabel>{
            width: Fill,
            text: "In terms of color types, we mainly divide them into the following kinds: \n1. Dark\n2. Primary\n3. Info\n4. Success\n5. Warning\n6. Error",
        }
        <GLabel>{
            width: Fill,
            text: "Each color type has 11 color levels: [25, 50, 100, 200, 300, 400, 500, 600, 700, 800, 900]",
        }
        <GVLayout>{
            height: Fit,
            width: Fill,
            spacing: 8.0,
            margin: {bottom: 8.0},
            <GLabel> {
                font_family: (BOLD_FONT),
                text: "Theme Dark: ",
                font_size: 12.0,
            }
            <GColor>{
                theme: Dark,
            }
            <GLabel> {
                font_family: (BOLD_FONT),
                text: "Theme Primary: ",
                font_size: 12.0,
            }
            <GColor>{
                theme: Primary,
            }
            <GLabel> {
                font_family: (BOLD_FONT),
                text: "Theme Info: ",
                font_size: 12.0,
            }
            <GColor>{
                theme: Info,
            }
            <GLabel> {
                font_family: (BOLD_FONT),
                text: "Theme Success: ",
                font_size: 12.0,
            }
            <GColor>{
                theme: Success,
            }
            <GLabel> {
                font_family: (BOLD_FONT),
                text: "Theme Warning: ",
                font_size: 12.0,
            }
            <GColor>{
                theme: Warning,
            }
            <GLabel> {
                font_family: (BOLD_FONT),
                text: "Theme Error: ",
                font_size: 12.0,
            }
            <GColor>{
                theme: Error,
            }

        }
    }
}

#[derive(Live, Widget)]
pub struct ColorPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for ColorPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for ColorPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let _ = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
    }
}
