use makepad_widgets::{id, Actions, HeapLiveIdPath, LiveId, WidgetAction, WidgetUid};

use super::from_str_unchecked;

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

pub trait HeapLiveIdPathExp {
    // body.navigation.application_pages.upload_frame.UniqueId 3.s3_list.UniqueId 3.UniqueId 1.share_wrap
    fn contains(&self, child: &HeapLiveIdPath) -> Result<bool, String>;
    fn contains_id(&self, id: &LiveId) -> bool;
    fn to_live_id(&self) -> Vec<LiveId>;
    fn trim_matches(&self, target: &HeapLiveIdPath) -> Vec<LiveId>;
    fn eq(&self, target: &HeapLiveIdPath) -> bool;
}

impl HeapLiveIdPathExp for HeapLiveIdPath {
    fn contains(&self, child: &HeapLiveIdPath) -> Result<bool, String> {
        // do format then split by `.`
        let father = format!("{:?}", self);
        let child = format!("{:?}", child);

        let father = father.split('.').collect::<Vec<&str>>();
        let child = child.split('.').collect::<Vec<&str>>();
        // eat one by one till `UniqueId`

        if father.len() < child.len() {
            return Err("father LiveIdPath length smaller than child".to_string());
        }

        let mut flag = true;
        for (index, c_p) in child.iter().enumerate() {
            // let f_p = if father[index].starts_with("UniqueId") {
            //     father[index].trim_start_matches("UniqueId ")
            // } else {
            //     father[index]
            // };
            // dbg!(c_p, f_p);

            if *c_p != father[index] {
                flag = false;
                break;
            }
        }
        Ok(flag)
    }

    /// not complete!!!
    fn to_live_id(&self) -> Vec<LiveId> {
        let path = format!("{:?}", self);
        path.split('.')
            .map(|x| LiveId(from_str_unchecked(x)))
            .collect()
    }

    fn trim_matches(&self, target: &HeapLiveIdPath) -> Vec<LiveId> {
        format!("{:?}", self)
            .trim_start_matches(&format!("{:?}", target))
            .split('.')
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| LiveId(from_str_unchecked(x.trim_matches('.'))))
            .collect()
    }

    fn eq(&self, target: &HeapLiveIdPath) -> bool {
        format!("{:?}", self) == format!("{:?}", target)
    }

    fn contains_id(&self, id: &LiveId) -> bool {
        
        format!("{:?}", self).contains(&id.to_string())
    }
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
