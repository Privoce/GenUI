use makepad_widgets::{
    ActionDefaultRef, Cx, DefaultNone, HeapLiveIdPath, WidgetActionCxExt, WidgetActionTrait,
    WidgetUid,
};

#[derive(Clone, Debug, DefaultNone)]
pub enum GWindowEvent {
    Opened,
    Closed,
    Minimize,
    Maximize,
    FullScreen,
    None,
}

fn active(cx: &mut Cx, uid: WidgetUid, path: &HeapLiveIdPath, e: impl WidgetActionTrait) -> () {
    cx.widget_action(uid, path, e);
}

pub fn active_open(uid: WidgetUid, path: &HeapLiveIdPath, cx: &mut Cx) -> () {
    active(cx, uid, path, GWindowEvent::Opened);
}

pub fn active_close(uid: WidgetUid, path: &HeapLiveIdPath, cx: &mut Cx) -> () {
    active(cx, uid, path, GWindowEvent::Closed);
}

pub fn active_fullscreen(uid: WidgetUid, path: &HeapLiveIdPath, cx: &mut Cx) -> () {
    active(cx, uid, path, GWindowEvent::FullScreen);
}

pub fn active_minimize(uid: WidgetUid, path: &HeapLiveIdPath, cx: &mut Cx) -> () {
    active(cx, uid, path, GWindowEvent::Minimize);
}

pub fn active_maximize(uid: WidgetUid, path: &HeapLiveIdPath, cx: &mut Cx) -> () {
    active(cx, uid, path, GWindowEvent::Maximize);
}
