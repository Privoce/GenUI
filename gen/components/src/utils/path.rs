use makepad_widgets::{HeapLiveIdPath, LiveId};

use super::from_str_unchecked;

pub trait HeapLiveIdPathExp {
    // body.navigation.application_pages.upload_frame.UniqueId 3.s3_list.UniqueId 3.UniqueId 1.share_wrap
    fn contains(&self, child: &HeapLiveIdPath) -> Result<bool, String>;
    fn contains_id(&self, id: &LiveId) -> bool;
    fn to_live_id(&self) -> Vec<LiveId>;
    fn trim_matches(&self, target: &HeapLiveIdPath) -> Vec<LiveId>;
    fn eq(&self, target: &HeapLiveIdPath) -> bool;
    fn is_empty(&self) -> bool;
    fn to_vec_str(&self) -> Vec<String>;
    fn to_string(&self) -> String;
}

impl HeapLiveIdPathExp for HeapLiveIdPath {
    fn to_vec_str(&self) -> Vec<String> {
        format!("{:?}", self)
            .split(".")
            .map(|x| x.to_string())
            .collect()
    }
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
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

    fn is_empty(&self) -> bool {
        format!("{:?}", self).is_empty()
    }
}
