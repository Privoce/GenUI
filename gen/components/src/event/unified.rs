use makepad_widgets::{
    ActionDefaultRef, Cx, DefaultNone, FingerHoverEvent, HeapLiveIdPath, WidgetActionCxExt,
    WidgetUid,
};

/// # UnifiedEvent
/// A unified event that should be transmitted throughout the entire system,
/// currently used to supplement event bubble transmission
#[derive(DefaultNone, Clone, Debug)]
pub enum UnifiedEvent {
    HoverIn(HoverParam),
    HoverOut(HoverParam),
    None,
}

impl UnifiedEvent {
    pub fn hover_in(cx: &mut Cx, uid: WidgetUid, path: &HeapLiveIdPath, e: FingerHoverEvent) {
        Self::hover(cx, uid, path, e, true);
    }
    pub fn hover_out(cx: &mut Cx, uid: WidgetUid, path: &HeapLiveIdPath, e: FingerHoverEvent) {
        Self::hover(cx, uid, path, e, false);
    }
    fn hover(cx: &mut Cx, uid: WidgetUid, path: &HeapLiveIdPath, e: FingerHoverEvent, is_in: bool) {
        let param = HoverParam {
            e,
            path: path.clone(),
        };
        cx.widget_action(
            uid,
            path,
            if is_in {
                UnifiedEvent::HoverIn(param)
            } else {
                UnifiedEvent::HoverOut(param)
            },
        );
    }
}

/// # HoverInParam
#[derive(Debug, Clone)]
pub struct HoverParam {
    pub e: FingerHoverEvent,
    pub path: HeapLiveIdPath,
}
