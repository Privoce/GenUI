mod event;
mod register;
pub use event::*;
pub use register::register;

use std::{path::PathBuf, str::FromStr};

use makepad_widgets::*;
#[cfg(not(target_arch = "wasm32"))]
use rfd::FileDialog;

use crate::{
    events_option, ref_area, ref_event_option, ref_redraw, set_event, set_scope_path, shader::manual::UploadMode, utils::filter_widget_actions, widget_area
};

use super::svg::GSvg;

live_design! {
    GUploadBase =  {{GUpload}}{
        draw_upload: {
            fn pixel(self) ->vec4{
                return vec4(0.0);
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct GUpload {
    #[live]
    pub path: Option<String>,
    #[live(true)]
    pub clear: bool,
    #[live(true)]
    pub check: bool,
    #[rust]
    pub selected: Vec<PathBuf>,
    #[live]
    pub icon: GSvg,
    // filters: is the file type filter, like ["jpg", "png"]
    #[live]
    pub filters: Vec<String>,
    #[redraw]
    #[live]
    pub draw_upload: DrawQuad,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[live(true)]
    pub grab_key_focus: bool,
    // visible -------------------
    #[live(true)]
    pub visible: bool,
    #[rust]
    real_path: PathBuf,
    #[live]
    pub mode: UploadMode,
    #[live(true)]
    pub event_key: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>
}

impl Widget for GUpload {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        self.set_scope_path(&scope.path);
        self.draw_upload.begin(cx, walk, self.layout);
        if self.icon.is_visible(){
            let icon_walk = self.icon.walk(cx);
            let _ = self.icon.draw_walk(cx, scope, icon_walk);
        }
        self.draw_upload.end(cx);
        DrawStep::done()
    }
    fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
        sweep_area: Area,
    ) {
        if !self.is_visible(){return;}
        let hit = event.hits_with_options(
            cx,
            self.area(),
            HitOptions::new().with_sweep_area(sweep_area),
        );

        self.handle_widget_event(cx, event, scope, hit, sweep_area)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.is_visible(){return;}
        let focus_area = self.area();
        let hit = event.hits(cx, self.area());
        self.handle_widget_event(cx, event, scope, hit, focus_area)
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GUpload {
    fn before_apply(
        &mut self,
        cx: &mut Cx,
        _apply: &mut Apply,
        _index: usize,
        _nodes: &[LiveNode],
    ) {
        let uid = self.widget_uid();
        let _ = self.path.as_ref().map(|p| {
            match PathBuf::from_str(p) {
                Ok(p) => {
                    if self.check {
                        // check path is exist
                        // now check is exist or not
                        if !p.exists() {
                            cx.widget_action(
                                uid,
                                &Scope::empty().path,
                                GUploadEvent::PathError(PathError {
                                    err_msg: format!("{:?} is not exist!", p.to_str()),
                                    path: p.to_str().unwrap().to_string(),
                                }),
                            );
                        }
                    }
                    self.real_path = p;
                }
                Err(e) => {
                    cx.widget_action(
                        uid,
                        &Scope::empty().path,
                        GUploadEvent::PathError(PathError {
                            err_msg: e.to_string(),
                            path: p.to_string(),
                        }),
                    );
                    self.real_path = PathBuf::from_str("/").unwrap();
                }
            };
        });
    }
}

impl GUpload {
    set_scope_path!();
    widget_area! {
        area, draw_upload
    }
    events_option! {
        clear: GUploadEvent::Clear => GUploadClearParam,
        path_error: GUploadEvent::PathError => PathError,
        before_selected: GUploadEvent::BeforeSelected => GUploadBeforeSelectedParam,
        selected: GUploadEvent::Selected => GUploadSelectedParam
    }
    pub fn redraw(&self, cx:&mut Cx)->(){
        if self.icon.is_visible(){
            self.icon.redraw(cx);
        }
        self.draw_upload.redraw(cx);
    }
    pub fn handle_widget_event(
        &mut self,
        cx: &mut Cx,
        _event: &Event,
        scope: &mut Scope,
        hit: Hit,
        focus_area: Area,
    ) {
        let uid = self.widget_uid();

        match hit {
            Hit::FingerDown(_) => {
                if self.grab_key_focus {
                    cx.set_key_focus(focus_area);
                }
            }
            Hit::FingerUp(e) => {
                if self.clear && self.selected.is_empty() {
                    let paths = self.selected.clone();
                    self.selected.clear();
                    cx.widget_action(
                        uid,
                        &scope.path,
                        GUploadEvent::Clear(GUploadClearParam { paths }),
                    );
                }

                // call before selected
                cx.widget_action(
                    uid,
                    &scope.path,
                    GUploadEvent::BeforeSelected(GUploadBeforeSelectedParam {
                        clear: self.clear,
                        mode: self.mode,
                    }),
                );
                // call system file picker
                #[cfg(not(target_arch = "wasm32"))]
                let mut f_upload = || {
                    let f = FileDialog::new()
                        .add_filter("allow", &self.filters)
                        .set_directory(self.real_path.as_path());

                    match self.mode {
                        UploadMode::Folder => {
                            f.pick_folder().map(|p| {
                                self.selected.push(p);
                            });
                        }
                        UploadMode::Folders => {
                            f.pick_folders().map(|p| {
                                self.selected.extend(p.into_iter());
                            });
                        }
                        UploadMode::File => {
                            f.pick_file().map(|p| {
                                self.selected.push(p);
                            });
                        }
                        UploadMode::Files => {
                            f.pick_files().map(|p| {
                                self.selected.extend(p.into_iter());
                            });
                        }
                    }
                };
                #[cfg(not(target_arch = "wasm32"))]
                f_upload();

                // call after selected
                cx.widget_action(
                    uid,
                    &scope.path,
                    GUploadEvent::Selected(GUploadSelectedParam {
                        paths: self.selected.clone(),
                        e: Some(e),
                    }),
                );
            }
            _ => {}
        }
    }
}

impl GUploadRef {
    ref_area!();
    ref_redraw!();
    ref_event_option! {
        clear => GUploadClearParam,
        path_error => PathError,
        before_selected => GUploadBeforeSelectedParam,
        selected => GUploadSelectedParam
    }
}

impl GUploadSet {
    set_event! {
        clear => GUploadClearParam,
        path_error => PathError,
        before_selected => GUploadBeforeSelectedParam,
        selected => GUploadSelectedParam
    }
}
