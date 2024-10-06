use std::error::Error;

use arboard::Clipboard;
use makepad_widgets::{Actions, WidgetAction, WidgetUid};

pub fn filter_widget_actions(
    actions: &Actions,
    widget_uid: WidgetUid,
) -> Option<Vec<&WidgetAction>> {
    let actions = actions
        .iter()
        .filter_map(|action| {
            if let Some(action) = action.downcast_ref::<WidgetAction>() {
                if action.widget_uid == widget_uid {
                    return Some(action);
                }
            }
            None
        })
        .collect::<Vec<_>>();

    if actions.is_empty() {
        None
    } else {
        Some(actions)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn open_browser(url: &str) -> Result<(), std::io::Error> {
    open::that(url)
}

pub fn copy_to_clipboard(value: &str) -> Result<(), Box<dyn Error>>{
    let mut clip_board = Clipboard::new()?;
    clip_board.set_text(value).map_err(|e|e.into())
}

#[cfg(test)]
mod e {
    use makepad_widgets::LiveId;

    use crate::utils::from_str_unchecked;

    #[test]
    fn r() {
        dbg!(LiveId(from_str_unchecked("hello")));
    }
}
