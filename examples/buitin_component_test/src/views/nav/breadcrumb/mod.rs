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

    pub BreadCrumbPage = {{BreadCrumbPage}}{
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
                text: "BreadCrumb",
            }
        }
        <GLabel>{
            width: Fill,
            text: "Breadcrumb can help you create a breadcrumb navigation, usually used in the header of the page.",
        }
        <GLabel>{
            font_size: 12.0,
            font_family: (BOLD_FONT),
            text: "BreadCrumbItem",
        }
        <GLabel>{
            width: Fill,
            text: "BreadcrumbItem is a item of the breadcrumb, you can set the theme, text, split_type of the item.",
        }
        <CBox>{
            box_wrap = {
                spacing: 16.0,
                <GBreadCrumbItem>{
                    theme: Error,
                    text: "Spliter",
                }
                <GBreadCrumbItem>{
                    text: "Line",
                    split_type: Line
                }
                item = <GBreadCrumbItem>{
                    theme: Success,
                    text: "Arrow",
                    split_type: Arrow,
                    text_hover_color: #F69D50,
                }
                e_btn = <GButton>{
                    slot:{
                        text: "Animation",
                    }
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
                <GBreadCrumbItem>{
                    theme: Error,
                    text: "Spliter",
                }
                <GBreadCrumbItem>{
                    text: "Line",
                    split_type: Line
                }
                <GBreadCrumbItem>{
                    theme: Success,
                    text: "Arrow",
                    split_type: Arrow
                }
                            "#;
                        }
                    }
                }
            }
        }
        <GLabel>{
            font_size: 12.0,
            font_family: (BOLD_FONT),
            text: "BreadCrumb (Virtual)",
        }
        <GLabel>{
            width: Fill,
            text: "BreadCrumb is a virtual component, you can set the labels to generate the breadcrumb.You can also set the crumb_item to set the style of the item.",
        }
        <GLabel>{
            width: Fill,
            text: "If labels.len() > 3, it will use a omit item to replace the middle items.",
        }
        <CBox>{
            box_wrap = {
                spacing: 16.0,
                <GBreadCrumb>{
                    path: ["home", "components", "button"],
                }
                <GBreadCrumb>{
                    path: ["home", "components", "gen", "ui", "crumb"],
                    item: {
                        theme: Primary,
                        split_type: Arrow,
                    }
                }
                <GBreadCrumb>{
                    theme: Error,
                    background_visible: true,
                    border_radius: 2.0,
                    omit: true,
                    path: ["home", "components", "gen", "ui", "crumb"],
                    icon = {
                        src: dep("crate://self/resources/setting.svg"),
                        theme: Error,
                        stroke_hover_color: #F69D50,
                        stroke_focus_color: #FF7043,
                    },
                    item: {
                        theme: Error,
                        text_hover_color: #F69D50,
                        text_focus_color: #FF7043,
                        split_type: Arrow,
                    }
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
                <GBreadCrumb>{
                    path: ["home", "components", "button"],
                }
                <GBreadCrumb>{
                    path: ["home", "components", "gen", "ui", "crumb"],
                    item: {
                        theme: Primary,
                        split_type: Arrow,
                    }
                }
                <GBreadCrumb>{
                    theme: Error,
                    background_visible: true,
                    border_radius: 2.0,
                    omit: true,
                    path: ["home", "components", "gen", "ui", "crumb"],
                    icon = {
                        src: dep("crate://self/resources/setting.svg"),
                        theme: Error,
                        stroke_hover_color: #F69D50,
                        stroke_focus_color: #FF7043,
                    },
                    item: {
                        theme: Error,
                        text_hover_color: #F69D50,
                        text_focus_color: #FF7043,
                        split_type: Arrow,
                    }
                }
                            "#;
                        }
                    }
                }
            }
        }
        <CBox>{
            box_wrap = {
                spacing: 16.0,
                bc = <GBreadCrumb>{
                    theme: Error,
                    background_visible: true,
                    border_radius: 2.0,
                    path: ["home", "components", "gen", "ui", "crumb"],
                    icon = {
                        src: dep("crate://self/resources/setting.svg"),
                        theme: Error,
                        stroke_hover_color: #F69D50,
                        stroke_focus_color: #FF7043,
                    },
                    item: {
                        theme: Error,
                        text_hover_color: #F69D50,
                        text_focus_color: #FF7043,
                        split_type: Arrow,
                    }
                }
                lb = <GLabel>{
                    text: ""
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
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        let cb = self.gbread_crumb(id!(bc));
        let lb = self.glabel(id!(lb));
        if let Some(e) = cb.changed(&actions) {
            lb.set_text(cx, &format!("You clicked: {}, text: {}", e.index, e.text));
        }
        if cb.home(&actions).is_some() {
            lb.set_text(cx, "You clicked home");
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
pub struct BreadCrumbPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for BreadCrumbPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for BreadCrumbPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

        let e_btn = self.gbutton(id!(e_btn));
        let item = self.gbread_crumb_item(id!(item));

        if e_btn.clicked(&actions).is_some() {
            item.play_hover_on(cx);
        }

        let cb = self.gbread_crumb(id!(bc));
        let lb = self.glabel(id!(lb));
        if let Some(e) = cb.changed(&actions) {
            lb.set_text(cx, format!("You clicked: {}, text: {}", e.index, e.text));
        }
        if cb.home(&actions).is_some() {
            lb.set_text(cx, "You clicked home".to_string());
        }
    }
}
