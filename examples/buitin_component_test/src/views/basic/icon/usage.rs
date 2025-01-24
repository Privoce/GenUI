use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;
    use crate::views::basic::icon::icon_lib::*;
    
    pub IconUsagePage = {{IconUsagePage}}{
        height: Fit,
        width: Fill,
        flow: Down,
        background_visible: false,
        border_radius: 0.0,
        spacing: 12.0,
        clip_x: true,
        clip_y: true,
        <GLabel>{
            font_size: 12.0,
            font_family: (BOLD_FONT),
            text: "Basic Icon(Icon Lib) Usage",
        }
        <GVLayout>{
            height: Fit,
            spacing: 8.0,
            <GLabel>{
                width: Fill,
                text: "The current icon library is only an experimental function. It provided a set of commonly used icon collections",
            }
            <GLabel>{
                font_size: 10.0,
                font_family: (BOLD_FONT),
                text: "Advantages: ",
            }
            <GVLayout>{
                height: Fit,
                spacing: 8.0,
                <GLabel>{
                    text: "1. No copyright, completely free for commercial use",
                }
                <GLabel>{
                    text: "2. Small size",
                }
                <GLabel>{
                    text: "3. No need to download from the internet for introduction",
                }
                <GLabel>{
                    text: "4. Directly use Shader for drawing",
                }
            }
            <GLabel>{
                font_size: 10.0,
                font_family: (BOLD_FONT),
                text: "Subsequent update plan: ",
            }
            <GVLayout>{
                height: Fit,
                spacing: 8.0,
                <GLabel>{
                    text: "1. Add icons to approximately 150 commonly used icons",
                }
                <GLabel>{
                    text: "2. Optimize icon experience",
                }
            }
        }
        <GIconLibExample>{}
    }
}

#[derive(Live, Widget)]
pub struct IconUsagePage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for IconUsagePage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for IconUsagePage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let _ = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
    }
}
