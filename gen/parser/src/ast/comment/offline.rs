use super::{position::OfflinePosition, Comments};

/// # Offline Comment
/// ## Display
/// ```gen
/// // this is offline comment
/// <template>
///  // this is inline comment
/// </template>
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct OfflineComment {
    value: Vec<Comments>,
    position: OfflinePosition,
}

impl OfflineComment {
    /// ## get the comment value
    /// ### return
    /// `&Vec<Comments>`
    pub fn value(&self) -> &Vec<Comments> {
        &self.value
    }
    /// ## get the position
    pub fn position(&self) -> OfflinePosition {
        self.position.clone()
    }
}

impl From<(Vec<Comments>, OfflinePosition)> for OfflineComment {
    fn from(value: (Vec<Comments>, OfflinePosition)) -> Self {
        OfflineComment {
            value: value.0,
            position: value.1,
        }
    }
}

impl ToString for OfflineComment {
    fn to_string(&self) -> String {
        self.value()
            .into_iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
