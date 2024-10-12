use gen_components::components::{button::GButtonWidgetExt, view::GView};
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;
    BOLD_FONT = dep("crate://self/resources/OPPOSans-Bold.ttf");
    CBox = <GVLayout>{
        height: Fit,
        box_wrap = <GView>{
            height: Fit,
            padding: 16.0,
            width: Fill,
            flow: Down,
        }
        code = <GCollapse>{
            width: Fill,
            height: Fit,
            header: {
                padding: 12.0,
                width: Fill,
                <GIcon>{
                    height: 12.0,
                    width: 12.0,
                    icon_type: Code,
                }
                <GLabel>{
                    text: "Example Code"
                }
            },
            body: {
                height: Fit,
                width: Fill,
                scroll_bars: <GScrollBars>{},

            }
        }
    }

    ButtonAnPage = {{ButtonAnPage}}{
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
            text: "Animation Usage",
        }
        <GLabel>{
            width: Fill,
            text: "Button has two animation effects: Hover and Focus(Press).",
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                hover_shadow = <GButton>{
                    spread_radius: 0.0,
                    slot: {
                        text: "Hover with shadow"
                    }
                }
            }
            code = {
                body: {
                    <GVLayout>{
                        height: 200.0,
                        scroll_bars: <GScrollBars>{},
                        <GLabel>{
                            theme: Dark,
                            width: Fill,
                            text: r#"

                            "#;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct ButtonAnPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for ButtonAnPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for ButtonAnPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

        let hover_shadow = self.gbutton(id!(hover_shadow));
        if hover_shadow.hover_in(&actions).is_some() {
            hover_shadow.apply_over(
                cx,
                live! {
                    spread_radius: 5.2,
                },
            );
            // hover_shadow.redraw(cx);
        }
        if hover_shadow.hover_out(&actions).is_some(){
            hover_shadow.apply_over(
                cx,
                live! {
                    spread_radius: 0.0,
                },
            );
        }
    }
}
