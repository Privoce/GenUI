use makepad_widgets::{Actions, HeapLiveIdPath, WidgetAction, WidgetUid};

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
    fn contains(&self, child: &HeapLiveIdPath) -> bool;
}

impl HeapLiveIdPathExp for HeapLiveIdPath {
    fn contains(&self, child: &HeapLiveIdPath) -> bool {
        // do format then split by `.`
        let father = format!("{:?}", self);
        let child = format!("{:?}", child);
        
        let father = father.split('.').collect::<Vec<&str>>();
        let child = child.split('.').collect::<Vec<&str>>();
        // eat one by one till `UniqueId`

        if father.len() < child.len() {
            panic!("father LiveIdPath length smaller than child");
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
        flag
    }
}
