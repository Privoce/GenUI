use makepad_widgets::Cx;

pub fn register(cx: &mut Cx) {
    self::live_design(cx);
}

use gen_components::components::view::GView;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;
    import crate::styles::*;

    BreadCrumbPage = {{BreadCrumbPage}}{
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
                <GBreadCrumbItem>{
                    theme: Success,
                    text: "Arrow",
                    split_type: Arrow
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
        let _ = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
    }
}
