use makepad_widgets::{font_atlas::CxFontsAtlasRc, Cx, Cx2d, Font, LiveDependency, MouseCursor};

pub fn get_font_family(font_family: &LiveDependency, cx: &mut Cx2d) -> Font {
    let font_family = font_family.clone();

    let atlas = cx.get_global::<CxFontsAtlasRc>().clone();
    let font_id = Some(
        atlas
            .0
            .borrow_mut()
            .get_font_by_path(cx, font_family.as_str()),
    );
    let font = Font {
        font_id,
        path: font_family,
    };
    font
}

pub fn set_cursor(cx: &mut Cx, cursor: Option<&MouseCursor>) -> () {
    if let Some(cursor) = cursor {
        cx.set_cursor(*cursor);
    } else {
        cx.set_cursor(MouseCursor::default());
    }
}

/// This macro generates the following functions: `text`, `set_text`, `set_text_and_visible`, `is_visible` in Widget trait
#[macro_export]
macro_rules! set_text_and_visible_fn {
    () => {
        fn text(&self) -> String {
            self.text.as_ref().to_string()
        }
        fn set_text(&mut self, v: &str) {
            self.text.as_mut_empty().push_str(v);
        }
        fn set_text_and_redraw(&mut self, cx: &mut Cx, v: &str) {
            self.text.as_mut_empty().push_str(v);
            self.redraw(cx)
        }
        fn is_visible(&self) -> bool {
            self.visible
        }
    };
}

#[macro_export]
macro_rules! widget_origin_fn {
    ($T: ty) => {
        pub fn as_origin(&self) -> Option<std::cell::Ref<$T>> {
            self.borrow()
        }
        pub fn as_origin_mut(&mut self) -> Option<std::cell::RefMut<$T>> {
            self.borrow_mut()
        }
    };
}

/// # Generate Ref Event Function
///```rust
/// impl GBreadCrumbItemRef {
/// 
///     ref_event_option!{
///         clicked => GBreadCrumbEventItemParam,
///         hover => GBreadCrumbEventItemParam
///     }
///     // pub fn clicked(&self, actions: &Actions) -> Option<GBreadCrumbEventItemParam> {
///     //     if let Some(c_ref) = self.borrow() {
///     //         return c_ref.clicked(actions);
///     //     }
///     //     None
///     // }
///     // pub fn hover(&self, actions: &Actions) -> Option<GBreadCrumbEventItemParam> {
///     //     if let Some(c_ref) = self.borrow() {
///     //         return c_ref.hover(actions);
///     //     }
///     //     None
///     // }
/// }
/// ```
#[macro_export]
macro_rules! ref_event_option {
    ($($event_fn: ident => $return: ty),*) => {
        $(
            pub fn $event_fn(&self, actions: &Actions) -> Option<$return> {
                if let Some(c_ref) = self.borrow() {
                    return c_ref.$event_fn(actions);
                }
                None
            }
        )*
    };
}

///```rust
/// impl GBreadCrumbItemRef {
///     ref_event_bool!{
///         clicked
///     }
/// }
/// ```
#[macro_export]
macro_rules! ref_event_bool {
    ($($event_fn: ident),*) => {
        $(
            pub fn $event_fn(&self, actions: &Actions) -> bool {
                if let Some(c_ref) = self.borrow() {
                    return c_ref.$event_fn(actions);
                }
                false
            }
        )*
    };
}

/// # Generate Set Event Function
/// ```rust
/// impl GBreadCrumbItemSet {
///     set_event!{
///        clicked => FingerUpEvent,
///        hover => FingerHoverEvent
///     }
/// }
/// ```
#[macro_export]
macro_rules! set_event {
    ($($event_fn: ident => $return: ty),*) => {
        $(
            pub fn $event_fn(&self, actions: &Actions) -> Option<$return> {
                for item in self.iter() {
                    if let Some(e) = item.$event_fn(actions) {
                        return Some(e);
                    }
                }
                None
            }
        )*
    };
}

#[macro_export]
macro_rules! set_event_bool {
    ($($event_fn: ident),*) => {
        $(
            pub fn $event_fn(&self, actions: &Actions) -> bool {
                self.iter().any(|c_ref| c_ref.$event_fn(actions))
            }
        )*
    };
}

/// ```
/// impl GBreadCrumbItem {
///     event_option!{
///         clicked : GBreadCrumbItemEvent => GBreadCrumbEventItemParam,
///         hover : GBreadCrumbItemEvent => GBreadCrumbEventItemParam
///     }
///     // pub fn clicked(&self, actions: &Actions) -> Option<GBreadCrumbEventItemParam> {
///     //     if let GBreadCrumbItemEvent::Clicked(e) =
///     //         actions.find_widget_action(self.widget_uid()).cast()
///     //     {
///     //         Some(e)
///     //     } else {
///     //         None
///     //     }
///     // }
///     // pub fn hover(&self, actions: &Actions) -> Option<GBreadCrumbEventItemParam> {
///     //     if let GBreadCrumbItemEvent::Hover(e) = actions.find_widget_action(self.widget_uid()).cast()
///     //     {
///     //         Some(e)
///     //     } else {
///     //         None
///     //     }
///     // }
/// }
/// ```
#[macro_export]
macro_rules! event_option {
    ($($event_fn: ident : $event: path => $return: ty),*) => {
        $(
            pub fn $event_fn(&self, actions: &Actions) -> Option<$return> {
                if let $event(e) =
                    actions.find_widget_action(self.widget_uid()).cast()
                {
                    Some(e)
                } else {
                    None
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! event_bool {
    ($($event_fn: ident : $event: path),*) => {
        $(
            pub fn $event_fn(&self, actions: &Actions) -> bool {
                if let $event =
                    actions.find_widget_action(self.widget_uid()).cast()
                {
                    true
                } else {
                    false
                }
            }
        )*
    };
}

/// # Generate Events Option Function (if a widget has multiple events in one action called)
/// See GFileUpload in `src/components/file_upload/mod.rs`
/// ```rust
/// pub fn after_select(&self, actions: &Actions) -> Option<Vec<PathBuf>> {
///     let mut res = None;
///     filter_widget_actions(actions, self.widget_uid()).map(|actions| {
///         actions.iter().for_each(|action| {
///             if let GFileUploadEvent::AfterSelect(e) = action.cast() {
///                 res.replace(e.clone());
///             }
///         })
///     });
///
///     res
/// }
/// ```
#[macro_export]
macro_rules! events_option {
    ($($event_fn: ident : $event: path => $return: ty),*) => {
        $(
            pub fn $event_fn(&self, actions: &Actions) -> Option<$return> {
                let mut res = None;
                filter_widget_actions(actions, self.widget_uid()).map(|actions| {
                    actions.iter().for_each(|action| {
                        if let $event(e) = action.cast() {
                            res.replace(e.clone());
                        }
                    })
                });
        
                res
            }
        )*
    };
}

/// # Generate Animation Function
/// ```
/// impl GBreadCrumbItemRef {
///     animatie_fn!{
///         animate_hover_on,
///         animate_hover_off,
///         animate_pressed
///     }
///     // pub fn animate_hover_on(&self, cx: &mut Cx) -> () {
///     //     self.borrow_mut().unwrap().animate_hover_on(cx);
///     // }
///     // pub fn animate_hover_off(&self, cx: &mut Cx) -> () {
///     //     self.borrow_mut().unwrap().animate_hover_off(cx);
///     // }
///     // pub fn animate_pressed(&self, cx: &mut Cx) -> () {
///     //     self.borrow_mut().unwrap().animate_pressed(cx);
///     // }
/// }
/// ```
#[macro_export]
macro_rules! animatie_fn{
    ($($an_fn: ident),*) => {
        $(
            pub fn $an_fn(&self, cx: &mut Cx) -> () {
                self.borrow_mut().unwrap().$an_fn(cx);
            }
        )*
    };
}

/// # Generate Area Function
/// ```
/// impl GBreadCrumbItem {
///     widget_area!{
///         area, draw_item
///     }
///     // pub fn area(&self) -> Area {
///     //     self.draw_item.area()
///     // }
/// }
/// ```
#[macro_export]
macro_rules! widget_area {
    ($($area_fn: ident, $prop: ident),*) => {
        $(
            pub fn $area_fn(&self) -> Area {
                self.$prop.area()
            }
        )*
    };
}