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

    pub TabbarPage = {{TabbarPage}}{
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
                text: "Tabbar",
            }
        }
        <GLabel>{
            width: Fill,
            text: "Tabbar can help you create a tabbar navigation, usually used in the footer of the page.",
        }
        <GLabel>{
            width: Fill,
            text: "Tabbar is built into the Router component, but you can also use it separately.",
        }
        <GLabel>{
            width: Fill,
            text: "Tabbar includes multiple TabbarItem components, it has a icon_slot and a text_slot.",
        }
        <CBox>{
            box_wrap = {
                spacing: 16.0,
                flow: Right,
                a = <GTabbarItem>{
                    icon_slot: {
                        src: dep("crate://self/resources/all.svg"),
                        stroke_focus_color: #FF0000,
                    }
                    text_slot: {
                        text: "All",
                    }
                }
                <GTabbarItem>{
                    icon_slot: {
                        src: dep("crate://self/resources/lightning.svg"),
                    }
                    text_slot: {
                        text: "Lightning",
                    }
                }
                btn  = <GButton>{slot: {text: "Unselected"}}
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

                            "#;
                        }
                    }
                }
            }
        }
        <GLabel>{
            text: "Tabbar has a changed event, you can get the selected index of the tabbar.",
        }
        <CBox>{
            box_wrap = {
                spacing: 16.0,
                <GHLayout>{
                    height: Fit,
                    spacing: 16.0,
                    align: {y: 0.5},
                    <GLabel>{
                        text: "You can add also styles in Tabbar"
                    }
                    <GTabbar>{
                        theme: Info,
                        align: {x: 0.5},
                        width: 300.0,
                        <GTabbarItem>{
                            icon_slot: {
                                src: dep("crate://self/resources/all.svg"),
                            }
                            text_slot: {
                                text: "All",
                            }
                        }
                        <GTabbarItem>{
    
                        }
                        <GTabbarItem>{
                            icon_slot: {
                                src: dep("crate://self/resources/lightning.svg"),
                            }
                            text_slot: {
                                text: "Lightning",
                            }
                        }
                    }
                }


                tb = <GTabbar>{
                    <GTabbarItem>{
                        icon_slot: {
                            src: dep("crate://self/resources/all.svg"),
                        }
                        text_slot: {
                            text: "All",
                        }
                    }
                    <GTabbarItem>{

                    }
                    <GTabbarItem>{
                        icon_slot: {
                            src: dep("crate://self/resources/lightning.svg"),
                        }
                        text_slot: {
                            text: "Lightning",
                        }
                    }
                }
                lb2 = <GLabel>{
                    text: "Tabbar selected: 0",
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
                tb = <GTabbar>{
                    <GTabbarItem>{
                        icon_slot: {
                            src: dep("crate://self/resources/all.svg"),
                        }
                        text_slot: {
                            text: "All",
                        }
                    }
                    <GTabbarItem>{

                    }
                    <GTabbarItem>{
                        icon_slot: {
                            src: dep("crate://self/resources/lightning.svg"),
                        }
                        text_slot: {
                            text: "Lightning",
                        }
                    }
                }
                fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
                    let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
                    let a = self.gtabbar_item(id!(a));
                    let btn = self.gbutton(id!(btn));
                    if btn.clicked(&actions).is_some() {
                        a.borrow_mut().map(|mut x| {
                            x.unselected(cx);
                        });
                    }
                    let tb = self.gtabbar(id!(tb));
                    let lb2 = self.glabel(id!(lb2));
                    if let Some(e) = tb.changed(&actions) {
                        lb2.set_text(cx, &format!("Tabbar selected: {}", e.selected));
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
pub struct TabbarPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for TabbarPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for TabbarPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        let a = self.gtabbar_item(id!(a));
        let btn = self.gbutton(id!(btn));
        if btn.clicked(&actions).is_some() {
            a.borrow_mut().map(|mut x| {
                x.unselected(cx);
            });
        }
        let tb = self.gtabbar(id!(tb));
        let lb2 = self.glabel(id!(lb2));
        if let Some(e) = tb.changed(&actions) {
            lb2.set_text(cx, format!("Tabbar selected: {}", e.selected));
        }
    }
}
