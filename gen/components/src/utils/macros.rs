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
                if !self.event_key{
                    return None;
                }

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
                if let Some(mut c_ref) = self.borrow_mut() {
                    c_ref.$an_fn(cx);
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! ref_play_animation {
    ($($an_fn: ident : $state: expr),*) => {
        $(
            pub fn $an_fn(&self, cx: &mut Cx) -> () {
                if let Some(mut c_ref) = self.borrow_mut() {
                    c_ref.play_animation(cx, $state);
                }
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

/// # Generate Area Function
#[macro_export]
macro_rules! ref_area {
    () => {
        pub fn area(&self) -> Area {
            if let Some(c_ref) = self.borrow() {
                return c_ref.area();
            }
            Area::Empty
        }
    };
}

#[macro_export]
macro_rules! ref_area_ext {
    ($($area_fn: ident),*) => {
        $(
            pub fn $area_fn(&self) -> Area {
                if let Some(c_ref) = self.borrow() {
                    return c_ref.$area_fn();
                }
                Area::Empty
            }
        )*
    };
}

#[macro_export]
macro_rules! ref_redraw {
    () => {
        pub fn redraw(&self, cx: &mut Cx) -> () {
            if let Some(c_ref) = self.borrow() {
                c_ref.redraw(cx);
            }
        }
    };
}

#[macro_export]
macro_rules! ref_redraw_mut {
    () => {
        pub fn redraw(&mut self, cx: &mut Cx) -> () {
            if let Some(mut c_ref) = self.borrow_mut() {
                c_ref.redraw(cx);
            }
        }
    };
}

#[macro_export]
macro_rules! ref_animate_state {
    () => {
        pub fn animate_state(&self) -> GLabelState {
            if let Some(c_ref) = self.borrow() {
                return c_ref.animate_state();
            }
            GLabelState::None
        }
    };
}

#[macro_export]
macro_rules! ref_render {
    () => {
        pub fn render(&self, cx: &mut Cx) -> () {
            if let Some(mut c_ref) = self.borrow_mut() {
                c_ref.render(cx);
            }
        }
    };
}

#[macro_export]
macro_rules! ref_actives {
    ($($event_fn: ident : $e_ty: ty),*) => {
       $(
            pub fn $event_fn(&self, cx: &mut Cx, e: $e_ty) -> () {
                self.borrow_mut().map(|mut c_ref| c_ref.$event_fn(cx, e));
            }
       )*
    };
}

/// ## Example
/// ```rust
/// active_event! {
///     active_hover_in: GButtonEvent::HoverIn |e: FingerHoverEvent| => GButtonHoverParam {e},
///     active_hover_out: GButtonEvent::HoverOut |e: FingerHoverEvent| => GButtonHoverParam {e},
///     active_focus: GButtonEvent::Focus |e: FingerDownEvent| => GButtonFocusParam {e},
///     active_focus_lost: GButtonEvent::FocusLost |e: FingerUpEvent| => GButtonFocusLostParam {e},
///     active_clicked: GButtonEvent::Clicked |e: FingerUpEvent| => GButtonClickedParam {e}
/// }
/// ```
#[macro_export]
macro_rules! active_event{
    ($($event_fn: ident : $event: path |$param: ident : $param_ty: ty| => $return_ty: expr),*) => {
        $(
            pub fn $event_fn (&mut self, cx: &mut Cx, $param: $param_ty){
                if self.event_key {
                    self.scope_path.as_ref().map(|path| {
                        cx.widget_action(
                            self.widget_uid(),
                            path,
                            $event($return_ty),
                        );
                    });
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! default_hit_finger_down {
    ($self:ident, $cx:ident, $focus_area:expr, $e:expr) => {
        if $self.grab_key_focus {
            $cx.set_key_focus($focus_area);
        }
        $self.play_animation($cx, id!(hover.focus));
        $self.active_focus($cx, $e);
    };
}

#[macro_export]
macro_rules! default_hit_hover_in {
    ($self:ident, $cx:ident, $e:expr) => {
        let _ = set_cursor($cx, $self.cursor.as_ref());
        $self.play_animation($cx, id!(hover.on));
        $self.active_hover_in($cx, $e);
    };
}

#[macro_export]
macro_rules! default_hit_hover_out {
    ($self:ident, $cx:ident, $e:expr) => {
        $self.play_animation($cx, id!(hover.off));
        $self.active_hover_out($cx, $e);
    };
}

#[macro_export]
macro_rules! default_hit_finger_up {
    ($self:ident, $cx:ident, $e:expr) => {
        if $e.is_over {
            if $e.device.has_hovers() {
                $self.play_animation($cx, id!(hover.on));
            } else {
                $self.play_animation($cx, id!(hover.off));
            }
            $self.active_clicked($cx, $e);
        } else {
            $self.play_animation($cx, id!(hover.off));
            $self.active_focus_lost($cx, $e);
        }
    };
}

#[macro_export]
macro_rules! default_hit_finger_up_some {
    ($self:ident, $cx:ident, $e:expr) => {
        if $e.is_over {
            if $e.device.has_hovers() {
                $self.play_animation($cx, id!(hover.on));
            } else {
                $self.play_animation($cx, id!(hover.off));
            }
            $self.active_clicked($cx, Some($e));
        } else {
            $self.play_animation($cx, id!(hover.off));
            $self.active_focus_lost($cx, Some($e));
        }
    };
}

#[macro_export]
macro_rules! default_handle_animation {
    ($self:ident, $cx:ident, $event: ident) => {
        if $self.animation_key {
            if $self.animator_handle_event($cx, $event).must_redraw() {
                $self.redraw($cx);
            }
        }
    };
}

#[macro_export]
macro_rules! set_scope_path {
    () => {
        pub fn set_scope_path(&mut self, path: &HeapLiveIdPath) {
            if self.scope_path.is_none() {
                self.scope_path.replace(path.clone());
            }
        }
    };
}

#[macro_export]
macro_rules! play_animation {
    () => {
        pub fn play_animation(&mut self, cx: &mut Cx, state: &[LiveId; 2]) {
            if self.animation_key {
                self.clear_animation(cx);
                self.animator_play(cx, state);
            }
        }
    };
}

#[macro_export]
macro_rules! check_event_scope {
    () => {
        fn check_event_scope(&self) -> Option<&HeapLiveIdPath> {
            self.event_key.then(|| self.scope_path.as_ref()).flatten()
        }
    };
}
