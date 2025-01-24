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

    pub UploadPage = {{UploadPage}}{
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
                text: "Upload",
            }
        }
        <GLabel>{
            width: Fill,
            text: "Upload is a component that allows you to upload files or folders. You can set by mode.",
        }
        <CBox>{
            box_wrap = {
                spacing: 12.0,
                <GHLayout>{
                    height: Fit,
                    spacing: 12.0,
                    <GView>{
                        height: 180.0,
                        theme: Info,
                        width: Fill,
                        flow: Down,
                        padding: 16.0,
                        align: {x: 0.0, y: 0.5},
                        spacing: 8.0,
                        up1 = <GUpload>{
                            height: 60.0,
                            mode: File,
                            clear: true,
                        }
                        <GLabel>{
                            font_size: 12.0,
                            text: "You can click here to upload a file",
                        }
                        <GHLayout>{
                            height: Fit,
                            <GLabel>{
                                font_size: 9.0,
                                color: #FF7043,
                                text: "pg/png files with a size less than 500kb "
                            }
                        }
                    }
                    <GView>{
                        height: 180.0,
                        theme: Success,
                        width: Fill,
                        width: Fill,
                        flow: Down,
                        padding: 16.0,
                        align: {x: 0.5, y: 0.5},
                        spacing: 8.0,
                        up2 = <GUpload>{
                            height: 60.0,
                            clear: true,
                            mode: Folder,
                            icon: {
                                src: dep("crate://self/resources/upload.svg"),
                            }
                        }
                        <GLabel>{
                            font_size: 12.0,
                            text: "You can click here to upload a folder",
                        }
                    }
                }
                <GHLayout>{
                    height: Fit,
                    spacing: 8.0,
                    <GLabel>{
                        text: "Event: "
                    }
                    e_label = <GLabel>{
                        text: ""
                    }
                    <GLabel>{
                        text: "Mode: "
                    }
                    m_label = <GLabel>{
                        text: ""
                    }
                }
                <GHLayout>{
                    height: Fit,
                    spacing: 8.0,
                    <GLabel>{
                        text: "Clears: "
                    }
                    c_label = <GLabel>{
                        text: ""
                    }
                    <GLabel>{
                        text: "Selects "
                    }
                    s_label = <GLabel>{
                        text: ""
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
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        let up1 = self.gupload(id!(up1));
        let e_label = self.glabel(id!(e_label));
        let c_label = self.glabel(id!(c_label));
        let s_label = self.glabel(id!(s_label));
        let m_label = self.glabel(id!(m_label));

        if let Some(e) = up1.clear(&actions) {
            c_label.set_text(cx, &format!("{:?}", e.paths));
            e_label.set_text(cx, "Clear"); 
        }

        if let Some(e) = up1.before_selected(&actions) {
            m_label.set_text(cx, &format!("Clear: {:?}, {:?}", e.clear, e.mode));
            e_label.set_text(cx, "Before Selected");
        }

        if let Some(e) = up1.selected(&actions) {
            s_label.set_text(cx, &format!("{:?}", e.paths));
            e_label.set_text(cx, "Selected");
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
pub struct UploadPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for UploadPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for UploadPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        let up1 = self.gupload(id!(up1));
        let e_label = self.glabel(id!(e_label));
        let c_label = self.glabel(id!(c_label));
        let s_label = self.glabel(id!(s_label));
        let m_label = self.glabel(id!(m_label));

        if let Some(e) = up1.clear(&actions) {
            c_label.set_text(cx, format!("{:?}", e.paths));
            e_label.set_text(cx, "Clear".to_string()); 
        }

        if let Some(e) = up1.before_selected(&actions) {
            m_label.set_text(cx, format!("Clear: {:?}, {:?}", e.clear, e.mode));
            e_label.set_text(cx, "Before Selected".to_string());
        }

        if let Some(e) = up1.selected(&actions) {
            s_label.set_text(cx, format!("{:?}", e.paths));
            e_label.set_text(cx, "Selected".to_string());
        }
    }
}
