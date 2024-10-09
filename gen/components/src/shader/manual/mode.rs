use makepad_widgets::*;

/// The `PopupMode` enum represents the different modes for a popup
#[derive(Live, LiveHook, PartialEq, Eq, Clone, Copy)]
#[live_ignore]
#[repr(u32)]
pub enum PopupMode {
    #[pick]
    Popup = shader_enum(1),
    ToolTip,
    Dialog,
    Drawer,
}

/// The `TriggerMode` enum represents the different modes for a trigger
#[derive(Live, LiveHook, PartialEq, Eq, Clone, Copy)]
#[live_ignore]
#[repr(u32)]
pub enum TriggerMode {
    #[pick]
    Click = shader_enum(1),
    Hover = shader_enum(2),
}

/// The `ComponentMode` enum represents the different modes for a component
#[derive(Live, LiveHook, PartialEq, Eq, Clone, Copy)]
#[live_ignore]
#[repr(u32)]
pub enum ComponentMode {
    #[pick]
    Real = shader_enum(1),
    Virtual = shader_enum(2),
}

/// The `UploadMode` enum represents the different modes for uploading
#[derive(Live, LiveHook, PartialEq, Eq, Clone, Copy)]
#[live_ignore]
#[repr(u32)]
pub enum UploadMode {
    Folder = shader_enum(1),
    Folders = shader_enum(2),
    #[pick]
    File = shader_enum(3),
    Files = shader_enum(4),
}

impl UploadMode {
    pub fn is_multi(&self) -> bool {
        match self {
            UploadMode::Folder | UploadMode::File => false,
            UploadMode::Folders | UploadMode::Files => true,
        }
    }
}

#[derive(Live, LiveHook, PartialEq, Eq, Clone, Copy)]
#[live_ignore]
#[repr(u32)]
pub enum WindowButtonMode {
    Desktop,
    #[pick]
    Tool,
}

/// Router Tabbar(Indicator|Menu) Mode
/// - VirtualMenu: virtual route use code to config GMenu (todo!)
/// - VirtualTabbar: virtual route use code to config GTabbar (AbstractGTabbar)(todo!)
/// - Bind: default mode, use dsl declare
/// - Define: define a indicator to call router nav_to
#[derive(Debug, Clone)]
pub enum RouterIndicatorMode {
    // VirtualMenu,
    // VirtualTabbar,
    Bind(LiveId),
    Define,
}

impl Default for RouterIndicatorMode {
    fn default() -> Self {
        Self::Bind(id!(tabbar)[0])
    }
}

impl RouterIndicatorMode {
    /// judge self is bind and eq the input id
    /// - if current is not bind -> false
    /// - or back `bind_id == id`
    pub fn eq_bind(&self, id: &LiveId) -> bool {
        if let RouterIndicatorMode::Bind(bind_id) = self {
            bind_id == id
        } else {
            false
        }
    }
}

#[derive(Debug, Clone)]
pub enum MenuItemMode {
    /// sub menu which has a title and items, items can be sub menu or menu item
    SubMenu(Vec<MenuItemMode>),
    /// menu item as a leaf node, `bool` is selected or not
    MenuItem(bool),
}

impl MenuItemMode {
    pub fn is_menu_item(&self) -> bool {
        matches!(self, MenuItemMode::MenuItem(_))
    }
    pub fn is_sub_menu(&self) -> bool {
        matches!(self, MenuItemMode::SubMenu(_))
    }

    /// get the selected index of the menu item
    /// try to find the item which is selected in the menu item
    pub fn selected(items: &Vec<MenuItemMode>) -> Option<Vec<usize>> {
        fn handle_nested(items: &Vec<MenuItemMode>, levels: &mut Vec<usize>) -> bool {
            let mut flag = false;
            for (index, item) in items.iter().enumerate() {
                match item {
                    MenuItemMode::SubMenu(subs) => {
                        if handle_nested(subs, levels) {
                            levels.splice(0..0, vec![index]);
                            flag = true;
                            break;
                        }
                    }
                    MenuItemMode::MenuItem(selected) => {
                        if *selected {
                            levels.push(index);
                            return true;
                        }
                    }
                }
            }
            flag
        }
        if items.is_empty() {
            return None;
        }

        let mut levels = vec![];
        if handle_nested(items, &mut levels) {
            Some(levels)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test_mode {
    #[test]
    fn menu_item() {
        let menu_items = vec![super::MenuItemMode::SubMenu(vec![
            super::MenuItemMode::MenuItem(false),
            super::MenuItemMode::MenuItem(false),
            super::MenuItemMode::SubMenu(vec![
                super::MenuItemMode::MenuItem(false),
                super::MenuItemMode::MenuItem(true),
                super::MenuItemMode::MenuItem(false),
            ]),
        ])];

        dbg!(super::MenuItemMode::selected(&menu_items));
    }
    #[test]
    fn menu_item2() {
        let menu_items = vec![
            super::MenuItemMode::SubMenu(vec![
                super::MenuItemMode::MenuItem(false),
                super::MenuItemMode::MenuItem(false),
                super::MenuItemMode::SubMenu(vec![
                    super::MenuItemMode::MenuItem(false),
                    super::MenuItemMode::MenuItem(false),
                    super::MenuItemMode::MenuItem(false),
                ]),
            ]),
            super::MenuItemMode::SubMenu(vec![
                super::MenuItemMode::MenuItem(true),
                super::MenuItemMode::MenuItem(false),
            ]),
            super::MenuItemMode::MenuItem(false),
        ];

        dbg!(super::MenuItemMode::selected(&menu_items));
    }
    #[test]
    fn menu_item3() {
        let menu_items = vec![
            super::MenuItemMode::MenuItem(false),
            super::MenuItemMode::SubMenu(vec![
                super::MenuItemMode::MenuItem(false),
                super::MenuItemMode::MenuItem(false),
                super::MenuItemMode::MenuItem(false),
            ]),
            super::MenuItemMode::MenuItem(false),
            super::MenuItemMode::MenuItem(true),
        ];

        dbg!(super::MenuItemMode::selected(&menu_items));
    }
}
