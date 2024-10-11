use makepad_widgets::Cx;

pub mod usage;
pub mod animate;
pub mod event;
pub mod virt;

pub fn register(cx: &mut Cx){
    self::live_design(cx);
    self::usage::live_design(cx);
    self::animate::live_design(cx);
    self::event::live_design(cx);
}

use gen_components::components::view::GView;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;
    import crate::views::basic::label::usage::*;
    import crate::views::basic::label::animate::*;
    import crate::views::basic::label::event::*;
    BOLD_FONT = dep("crate://self/resources/OPPOSans-Bold.ttf");
    LabelPage = {{LabelPage}}{
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
                text: "Label",
            }
        }
        <LabelUsagePage>{}
        <LabelAnPage>{}
        <LabelEnPage>{}
        <GLabel>{
            font_size: 12.0,
            font_family: (BOLD_FONT),
            text: "Label API",
        }
        <GLabel>{
            font_size: 10.0,
            font_family: (BOLD_FONT),
            text: "Label Props",
        }
        <GTable>{
            height: Fit,
            width: Fill,
            header: {
                height: Fit,
                width: Fill,
                <GTRow>{
                    height: 32.0,
                    width: Fit,
                    <GTCell>{
                        height: Fill,
                        width: 140.0,
                        <GLabel>{
                            color: #667085,
                            text: "Prop Name",
                        }  
                    }
                    <GTCell>{
                        height: Fill,
                        width: 200.0,
                        <GLabel>{
                            color: #667085,
                            text: "Description",
                        }
                    }
                    <GTCell>{
                        height: Fill,
                        width: 160.0,
                        <GLabel>{
                            color: #667085,
                            text: "Type",
                        }
                    }
                    <GTCell>{
                        height: Fill,
                        width: 160.0,
                        <GLabel>{
                            color: #667085,
                            text: "Default Value",
                        }
                    }
                }
            }
            body: <GTBody>{
                height: 120.0,
                width: Fill,
                <GTRow>{
                    height: 32.0,
                    width: Fill,
                    <GTCell>{
                        height: Fill,
                        width: 140.0,
                        <GLabel>{
                            color: #667085,
                            text: "theme",
                        }  
                    }
                    <GTCell>{
                        height: Fill,
                        width: 200.0,
                        <GLabel>{
                            color: #667085,
                            text: "GenUI Themes",
                        }
                    }
                    <GTCell>{
                        height: Fill,
                        width: 160.0,
                        <GLabel>{
                            color: #667085,
                            text: "Themes",
                        }
                    }
                    <GTCell>{
                        height: Fill,
                        width: 160.0,
                        <GLabel>{
                            color: #667085,
                            text: "Dark",
                        }
                    }
                }
                
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct LabelPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for LabelPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for LabelPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let _ = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
    }
}
