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

    pub DrawerPage = {{DrawerPage}}{
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
                text: "Drawer",
            }

        }
        <GLabel>{
            width: Fill,
            text: "Drawer is a popup that can be displayed in four directions: top, bottom, left, and right.It is a good choice for displaying additional information or operations.",
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                flow: Right,
                drawer1 = <GDropDown>{
                    trigger = <GButton>{
                        slot: {
                            text:"bottom"
                        }
                    },
                    popup :<GDrawer> {
                        container: {
                            flow: Down,
                            spacing: 10.0,
                            padding: 10.0,
                            align: {x: 1.0},
                            close_icon = <GIcon>{
                                icon_type: Close,
                                height: 12.0,
                                width: 12.0,
                                animation_key: true,
                                stroke_hover_color: #FF0000,
                            }
                        }
                    }
                }
                drawer2 = <GDropDown>{
                    position: Top,
                    trigger = <GButton>{
                        slot: {
                            text:"top"
                        }
                    },
                    popup :<GDrawer> {
                        container: {
                            flow: Down,
                            spacing: 10.0,
                            padding: 10.0,
                            align: {x: 1.0},
                            close_icon = <GIcon>{
                                icon_type: Close,
                                height: 12.0,
                                width: 12.0,
                                animation_key: true,
                                stroke_hover_color: #FF0000,
                            }
                        }
                    }
                }
                drawer3 = <GDropDown>{
                    position: Left,
                    trigger = <GButton>{
                        slot: {
                            text:"left"
                        }
                    },
                    popup :<GDrawer> {
                        container: {
                            flow: Down,
                            spacing: 10.0,
                            padding: {
                                top: 36.0,
                                left: 10.0,
                                right: 10.0,
                            },
                            align: {x: 1.0},
                            close_icon = <GIcon>{
                                icon_type: Close,
                                height: 12.0,
                                width: 12.0,
                                animation_key: true,
                                stroke_hover_color: #FF0000,
                            }
                        }
                    }
                }
                drawer4 = <GDropDown>{
                    position: Right,
                    trigger = <GButton>{
                        slot: {
                            text:"right"
                        }
                    },
                    popup :<GDrawer> {
                        container: {
                            flow: Down,
                            spacing: 10.0,
                            padding: 10.0,
                            align: {x: 1.0},
                            close_icon = <GIcon>{
                                icon_type: Close,
                                height: 12.0,
                                width: 12.0,
                                animation_key: true,
                                stroke_hover_color: #FF0000,
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
                drawer4 = <GDropDown>{
                    position: Right,
                    trigger = <GButton>{
                        slot: {
                            text:"right"
                        }
                    },
                    popup :<GDrawer> {
                        container: {
                            flow: Down,
                            spacing: 10.0,
                            padding: 10.0,
                            align: {x: 1.0},
                            close_icon = <GIcon>{
                                icon_type: Close,
                                height: 12.0,
                                width: 12.0,
                                animation_key: true,
                                stroke_hover_color: #FF0000,
                            }
                        }
                    }
                }
                fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
                    let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
                    let mut drawer1 = self.gdrop_down(id!(drawer1));
                    let mut drawer2 = self.gdrop_down(id!(drawer2));
                    let mut drawer3 = self.gdrop_down(id!(drawer3));
                    let mut drawer4 = self.gdrop_down(id!(drawer4));

                    fn close_handle(drawer: &mut GDropDownRef, cx: &mut Cx, actions: &Actions) {
                        drawer.get_mut(cx, |cx, drawer, container|{
                            let close_icon = container.gicon(id!(close_icon));
                            if close_icon.clicked(actions).is_some(){
                                drawer.close(cx);
                            }
                        });
                    }

                    close_handle(&mut drawer1, cx, &actions);
                    close_handle(&mut drawer2, cx, &actions);
                    close_handle(&mut drawer3, cx, &actions);
                    close_handle(&mut drawer4, cx, &actions);
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
pub struct DrawerPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for DrawerPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for DrawerPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        let mut drawer1 = self.gdrop_down(id!(drawer1));
        let mut drawer2 = self.gdrop_down(id!(drawer2));
        let mut drawer3 = self.gdrop_down(id!(drawer3));
        let mut drawer4 = self.gdrop_down(id!(drawer4));

        fn close_handle(drawer: &mut GDropDownRef, cx: &mut Cx, actions: &Actions) {
            drawer.get_mut(cx, |cx, drawer, container|{
                let close_icon = container.gicon(id!(close_icon));
                if close_icon.clicked(actions).is_some(){
                    drawer.close(cx);
                }
            });
        }

        close_handle(&mut drawer1, cx, &actions);
        close_handle(&mut drawer2, cx, &actions);
        close_handle(&mut drawer3, cx, &actions);
        close_handle(&mut drawer4, cx, &actions);
    }
}
