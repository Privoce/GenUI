use std::path::PathBuf;

use makepad_widgets::*;
use rfd::FileDialog;

use crate::shader::draw_card::DrawCard;

use super::svg::GSvg;

live_design! {
    GUploadBase =  {{GUpload}}{}
}

#[derive(Live, Widget)]
pub struct GUpload {
    #[live]
    pub multi: bool,
    #[live(true)]
    pub clear: bool,
    #[rust]
    pub selected: Vec<PathBuf>,
    #[live]
    pub icon: GSvg,
    #[live]
    pub filters: Vec<String>,
    #[redraw]
    #[live]
    pub draw_upload: DrawCard,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
}

impl Widget for GUpload {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_upload.begin(cx, walk, self.layout);
        let icon_walk = self.icon.walk(cx);
        let _ = self.icon.draw_walk(cx, scope, icon_walk);
        self.draw_upload.end(cx);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if self.clear {
            self.selected.clear();
        }

        match event.hits(cx, self.area()) {
            Hit::FingerDown(_) => {
                // call system file picker
                let f = FileDialog::new()
                    .add_filter("allow", &self.filters)
                    .set_directory("/");
                if self.multi {
                    f.pick_files().map(|p| {
                        self.selected.extend(p.into_iter());
                    });
                } else {
                    f.pick_file().map(|p| {
                        self.selected.push(p);
                    });
                }
            }
            _ => {}
        }
    }
}

impl LiveHook for GUpload {}

impl GUpload {
    pub fn area(&self) -> Area {
        self.draw_upload.area
    }
}
