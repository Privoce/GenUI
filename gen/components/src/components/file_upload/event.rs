use std::path::PathBuf;

use makepad_widgets::DefaultNone;

#[derive(Clone, Debug, DefaultNone)]
pub enum GFileUploadEvent {
    // back is multi or not
    BeforeSelect(bool),
    AfterSelect(Vec<PathBuf>),
    Clear(Vec<PathBuf>),
    PathError(PathError),
    None,
}

#[derive(Debug, Clone)]
pub struct PathError{
    pub err_msg: String,
    /// here pathbuf is not better
    pub path: String
}
