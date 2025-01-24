use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    BOLD_FONT = dep("crate://self/resources/OPPOSans-Bold.ttf");
    pub StartPage = {{StartPage}}{
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
        align: {y: 0.5},
        <GHLayout>{
            height: Fit,
            align: {x: 0.5},
            <GLabel>{
                font_size: 18.0,
                font_family: (BOLD_FONT),
                text: "GenUI Builtin Components (v0.1.0)",
            }
        }
        <GVLayout>{
            height: Fit,
            spacing: 8.0,
            width: Fill,
            <GLabel>{
                text: "• version: v0.1.0",
            }
            <GLabel>{
                text: "• update date: 2024-10-14",
            }
            <GLabel>{
                text: "• author: Will-YiFei Sheng",
            }
            <GLabel>{
                text: "• makepad_widget version: v0.6.0",
            }
            <GLabel>{
                text: "• makepad branch: rik",
            }
            <GLink>{
                text: "Github Repo: GenUI Builtin Components",
                href: "https://github.com/Privoce/GenUI/tree/components/gen/components"
            }
        }
        <GLabel>{
            width: Fill,
            text: "GenUI Builtin Components is a Makepad component library that can be directly integrated into Makepad projects. As the core component library of GenUI, it serves as the foundational building blocks within GenUI projects, accelerating the development process for both front-end and back-end developers.",
        }
        <GLabel>{
            width: Fill,
            text: "However, this is more than just a component library. It includes numerous built-in macros and system APIs that simplify development workflows and extend Makepad's current capabilities. It represents only one part of the many foundational modules in GenUI, functioning like the default HTML elements in JavaScript but designed with a more modern approach.",
        }
    }
}

#[derive(Live, Widget)]
pub struct StartPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for StartPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for StartPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let _ = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
    }
}
