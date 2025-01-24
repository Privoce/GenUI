use gen_components::*;

use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub ProgressEnPage = {{ProgressEnPage}}{
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
            text: "Event Usage",
        }
        <GVLayout>{
            height: Fit,
            spacing: 8.0,
            <GLabel>{
                width: Fill,
                text: "In Progress, you should focus on 2 events: 1. Changed 2. BeforeChanged",
            }
        }
        an_box = <CBox>{
            box_wrap = {
                spacing: 8.0,
                <GVLayout>{
                    height: Fit,
                    spacing: 12.0,
                    progress = <GProgress>{
                        theme: Info,
                        value: 36.0,
                        max: 40.0,
                        step: 0.5,
                        read_only: false,
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 8.0,
                    <GHLayout>{
                        spacing: 8.0,
                        height: Fit,
                        clear = <GButton>{
                            slot: {text: "Clear"}
                        }
                        set1 = <GButton>{
                            slot: {text: "Set 50%"}
                        }
                        full = <GButton>{
                            slot: {text: "Full 100%"}
                        }
                        get = <GButton>{
                            slot: {text: "Get Value"}
                        }
                        percent = <GButton>{
                            slot: {text: "Get Value Percent"}
                        }
                    }
                    <GHLayout>{
                        spacing: 8.0,
                        height: Fit,
                        add = <GButton>{
                            slot: {text: "Add 10%"}
                        }
                        sub = <GButton>{
                            slot: {text: "Sub 10%"}
                        }
                        add_p = <GButton>{
                            slot: {text: "Add 12.5"}
                        }
                        sub_p = <GButton>{
                            slot: {text: "Sub 5.0"}
                        }
                    }
                }
                <GHLayout>{
                    height: Fit,
                    spacing: 16.0,
                    <GLabel>{
                        text: "Event: "
                    }
                    e_label = <GLabel>{
                        text: "",
                    }
                    <GLabel>{
                        text: "Value: "
                    }
                    val_label = <GLabel>{
                        text: "",
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
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        let mut progress = self.gprogress(id!(progress));
        let val_label = self.glabel(id!(val_label));
        let e_label = self.glabel(id!(e_label));
        let set1 = self.gbutton(id!(set1));
        let full = self.gbutton(id!(full));
        let clear = self.gbutton(id!(clear));
        let get = self.gbutton(id!(get));
        let add = self.gbutton(id!(add));
        let add_p = self.gbutton(id!(add_p));
        let sub = self.gbutton(id!(sub));
        let sub_p = self.gbutton(id!(sub_p));
        let percent = self.gbutton(id!(percent));
        if get.clicked(&actions).is_some() {
            progress.get().map(|v| {
                val_label.set_text(cx, &v.to_string());
            });
        }
        if clear.clicked(&actions).is_some() {
            progress.clear(cx);
        }
        if set1.clicked(&actions).is_some() {
            progress.set(20.0, cx);
        }
        if full.clicked(&actions).is_some() {
            progress.full(cx);
        }
        if add.clicked(&actions).is_some() {
            progress.add_percent(0.1, cx);
        }
        if sub.clicked(&actions).is_some() {
            progress.sub_percent(0.1, cx);
        }
        if add_p.clicked(&actions).is_some() {
            progress.add(12.5, cx);
        }
        if sub_p.clicked(&actions).is_some() {
            progress.sub(5.0, cx);
        }
        if percent.clicked(&actions).is_some() {
            progress.percent().map(|v| {
                val_label.set_text(cx, &format!("{:.2}%", v));
            });
        }
        if let Some(e) = progress.changed(&actions) {
            e_label.set_text(cx, "Changed");
            val_label.set_text(cx, &e.value.to_string());
        }
        if progress.before_changed(&actions).is_some() {
            e_label.set_text(cx, "BeforeChanged");
        }
        if progress.hover_in(&actions).is_some() {
            e_label.set_text(cx, "Hover In");
        }
        if progress.hover_out(&actions).is_some() {
            e_label.set_text(cx, "Hover Out");
        }
        if progress.focus_lost(&actions).is_some() {
            e_label.set_text(cx, "Focus Lost");
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
pub struct ProgressEnPage {
    #[deref]
    #[redraw]
    pub deref_widget: GView,
}

impl LiveHook for ProgressEnPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for ProgressEnPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        let mut progress = self.gprogress(id!(progress));
        let val_label = self.glabel(id!(val_label));
        let e_label = self.glabel(id!(e_label));
        let set1 = self.gbutton(id!(set1));
        let full = self.gbutton(id!(full));
        let clear = self.gbutton(id!(clear));
        let get = self.gbutton(id!(get));
        let add = self.gbutton(id!(add));
        let add_p = self.gbutton(id!(add_p));
        let sub = self.gbutton(id!(sub));
        let sub_p = self.gbutton(id!(sub_p));
        let percent = self.gbutton(id!(percent));
        if get.clicked(&actions).is_some() {
            progress.get().map(|v| {
                val_label.set_text(cx, v.to_string());
            });
        }
        if clear.clicked(&actions).is_some() {
            progress.clear(cx);
        }
        if set1.clicked(&actions).is_some() {
            progress.set(20.0, cx);
        }
        if full.clicked(&actions).is_some() {
            progress.full(cx);
        }
        if add.clicked(&actions).is_some() {
            progress.add_percent(0.1, cx);
        }
        if sub.clicked(&actions).is_some() {
            progress.sub_percent(0.1, cx);
        }
        if add_p.clicked(&actions).is_some() {
            progress.add(12.5, cx);
        }
        if sub_p.clicked(&actions).is_some() {
            progress.sub(5.0, cx);
        }
        if percent.clicked(&actions).is_some() {
            progress.percent().map(|v| {
                val_label.set_text(cx, format!("{:.2}%", v));
            });
        }
        if let Some(e) = progress.changed(&actions) {
            e_label.set_text(cx, "Changed".to_string());
            val_label.set_text(cx, e.value.to_string());
        }
        if progress.before_changed(&actions).is_some() {
            e_label.set_text(cx, "BeforeChanged".to_string());
        }
        if progress.hover_in(&actions).is_some() {
            e_label.set_text(cx, "Hover In".to_string());
        }
        if progress.hover_out(&actions).is_some() {
            e_label.set_text(cx, "Hover Out".to_string());
        }
        if progress.focus_lost(&actions).is_some() {
            e_label.set_text(cx, "Focus Lost".to_string());
        }
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}
