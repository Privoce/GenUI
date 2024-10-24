use std::path::PathBuf;

use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerUpEvent};
use rfd::FileDialog;

use crate::shader::manual::UploadMode;

#[derive(Clone, Debug, DefaultNone)]
pub enum GUploadEvent {
    // back is multi or not
    BeforeSelected(GUploadBeforeSelectedParam),
    Selected(GUploadSelectedParam),
    Clear(GUploadClearParam),
    PathError(PathError),
    None,
}

#[derive(Debug, Clone)]
pub struct PathError {
    pub err_msg: String,
    /// here pathbuf is not better
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct GUploadBeforeSelectedParam {
    /// clear the selected or not
    pub clear: bool,
    pub mode: UploadMode,
}

#[derive(Debug, Clone)]
pub struct GUploadSelectedParam {
    /// paths which has been selected
    pub paths: Vec<PathBuf>,
    pub e: Option<FingerUpEvent>,
}

#[derive(Debug, Clone)]
pub struct GUploadClearParam {
    /// paths which has been clear
    pub paths: Vec<PathBuf>,
}

pub fn new_file_dialog() -> FileDialog {
    FileDialog::new()
}
