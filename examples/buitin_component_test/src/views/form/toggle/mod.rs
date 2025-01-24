use makepad_widgets::Cx;

pub mod animate;
pub mod event;
pub mod usage;

pub fn register(cx: &mut Cx) {
    self::live_design(cx);
    self::usage::live_design(cx);
    self::animate::live_design(cx);
    self::event::live_design(cx);
}

use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::views::form::toggle::usage::*;
    use crate::views::form::toggle::animate::*;
    use crate::views::form::toggle::event::*;
    BOLD_FONT = dep("crate://self/resources/OPPOSans-Bold.ttf");
    pub TogglePage = {{TogglePage}}{
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
                text: "Toggle",
            }
        }
        <ToggleUsagePage>{}
        <ToggleAnPage>{}
        <ToggleEnPage>{}
    }
}

#[derive(Live, Widget)]
pub struct TogglePage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for TogglePage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for TogglePage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let _ = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
    }
}
