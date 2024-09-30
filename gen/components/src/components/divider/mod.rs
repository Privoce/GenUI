pub mod event;
mod register;
use event::*;
pub use register::register;

use crate::shader::draw_divider::DrawGDivider;
use crate::themes::Themes;
use crate::utils::{set_cursor, ThemeColor};
use crate::{
    animatie_fn, event_option, ref_event_option, set_event, widget_area, widget_origin_fn,
};

use makepad_widgets::*;
// GDivider component
live_design! {
    import makepad_draw::shader::std::*;
    GLOBAL_DURATION = 0.25

    GDividerBase = {{GDivider}}{
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_divider: { hover: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                        pressed: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_divider: { hover: [{time: 0.0, value: 1.0}],}
                    }
                }
            }

        }
    }
}

#[derive(Live, LiveRegisterWidget, WidgetRef, WidgetSet)]
pub struct GDivider {
    #[live]
    pub theme: Themes,
    #[live]
    pub stroke_color: Option<Vec4>,
    #[live]
    pub stroke_hover_color: Option<Vec4>,
    #[live(0.8)]
    pub stroke_width: f64,
    #[live(true)]
    pub visible: bool,
    #[live]
    pub cursor: Option<MouseCursor>,
    // control ---------------------
    #[live(true)]
    pub grab_key_focus: bool,
    #[live(false)]
    pub block_signal_event: bool,
    // deref ---------------------
    #[live]
    pub draw_divider: DrawGDivider,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[rust]
    draw_state: DrawStateWrap<DrawState>,
    #[rust]
    pub children: ComponentMap<LiveId, WidgetRef>,
    #[rust]
    pub draw_order: Vec<LiveId>,
    #[live]
    pub event_order: EventOrder,
    #[rust]
    pub defer_walks: Vec<(LiveId, DeferWalk)>,
    #[live(true)]
    pub animation_open: bool,
    #[animator]
    pub animator: Animator,
}

#[derive(Clone)]
enum DrawState {
    Drawing(usize, bool),
    DeferWalk(usize),
}

impl Widget for GDivider {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // begin the draw state
        if self.draw_state.begin(cx, DrawState::Drawing(0, false)) {
            if !self.visible {
                // visible is false, so we are done
                self.draw_state.end();
                return DrawStep::done();
            }
            self.defer_walks.clear();

            // begin draw the view
            let _ = self.draw_divider.begin(cx, walk, self.layout);
        }

        // loop handle the inner children
        while let Some(DrawState::Drawing(step, resumed)) = self.draw_state.get() {
            if step < self.draw_order.len() {
                // get id from draw_order list
                let id = self.draw_order[step];
                // get the child widget by id
                if let Some(child) = self.children.get_mut(&id) {
                    // is the child visible?
                    // true -> draw the child walk
                    if child.is_visible() {
                        let walk = child.walk(cx);
                        // if resumed
                        if !resumed {
                            self.draw_state.set(DrawState::Drawing(step, true));
                        }
                        scope.with_id(id, |scope| child.draw_walk(cx, scope, walk))?;
                    }
                }
                // set the next step
                self.draw_state.set(DrawState::Drawing(step + 1, false));
            } else {
                self.draw_state.set(DrawState::DeferWalk(0));
            }
        }

        // loop handle the defer walk
        while let Some(DrawState::DeferWalk(step)) = self.draw_state.get() {
            if step < self.defer_walks.len() {
                let (id, d_walk) = &mut self.defer_walks[step];
                if let Some(child) = self.children.get_mut(&id) {
                    let walk = d_walk.resolve(cx);
                    scope.with_id(*id, |scope| child.draw_walk(cx, scope, walk))?;
                }
                self.draw_state.set(DrawState::DeferWalk(step + 1));
            } else {
                // draw background
                self.draw_divider.end(cx);
            }
            self.draw_state.end();
        }

        DrawStep::done()
    }
    fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
        sweep_area: Area,
    ) {
        let hit = event.hits_with_options(
            cx,
            self.area(),
            HitOptions::new().with_sweep_area(sweep_area),
        );

        self.handle_widget_event(cx, event, scope, hit, sweep_area)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        if self.animation_open {
            if self.animator_handle_event(cx, event).must_redraw() {
                self.redraw(cx);
            }
        }

        match &self.event_order {
            EventOrder::Down => {
                for id in self.draw_order.iter() {
                    if let Some(child) = self.children.get_mut(id) {
                        if child.is_visible() || !event.requires_visibility() {
                            scope.with_id(*id, |scope| {
                                child.handle_event(cx, event, scope);
                            })
                        }
                    }
                }
            }
            EventOrder::Up => {
                // the default event order is Up
                for id in self.draw_order.iter().rev() {
                    if let Some(child) = self.children.get_mut(id) {
                        if child.is_visible() || !event.requires_visibility() {
                            scope.with_id(*id, |scope| {
                                child.handle_event(cx, event, scope);
                            });
                        }
                    }
                }
            }
            EventOrder::List(list) => {
                for id in list {
                    if let Some(child) = self.children.get_mut(id) {
                        if child.is_visible() || !event.requires_visibility() {
                            scope.with_id(*id, |scope| {
                                child.handle_event(cx, event, scope);
                            })
                        }
                    }
                }
            }
        }

        // handle event and set cursor to control
        match event.hits(cx, self.area()) {
            Hit::KeyDown(e) => {
                if self.grab_key_focus {
                    cx.widget_action(uid, &scope.path, GDividerEvent::KeyDown(e))
                }
            }
            Hit::KeyUp(e) => {
                if self.grab_key_focus {
                    cx.widget_action(uid, &scope.path, GDividerEvent::KeyUp(e))
                }
            }
            // Hit::FingerScroll(e) => cx.widget_action(uid, &scope.path, GDividerEvent::FingerScroll(e)),
            Hit::FingerDown(e) => {
                if self.grab_key_focus {
                    cx.set_key_focus(self.area());
                }
                cx.widget_action(uid, &scope.path, GDividerEvent::FingerDown(e));
            }
            Hit::FingerMove(e) => cx.widget_action(uid, &scope.path, GDividerEvent::FingerMove(e)),
            Hit::FingerHoverIn(e) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                cx.widget_action(uid, &scope.path, GDividerEvent::FingerHoverIn(e));
                self.animator_play(cx, id!(hover.on))
            }
            Hit::FingerHoverOver(e) => {
                cx.widget_action(uid, &scope.path, GDividerEvent::FingerHoverOver(e));
            }
            Hit::FingerHoverOut(e) => {
                cx.widget_action(uid, &scope.path, GDividerEvent::FingerHoverOut(e));
                self.animator_play(cx, id!(hover.off))
            }
            Hit::FingerUp(e) => {
                cx.widget_action(uid, &scope.path, GDividerEvent::FingerUp(e));
            }
            _ => (),
        }
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl WidgetNode for GDivider {
    fn find_widgets(&self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        for child in self.children.values() {
            child.find_widgets(path, cached, results);
        }
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        self.walk
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.draw_divider.redraw(cx);
        for child in self.children.values_mut() {
            child.redraw(cx);
        }
    }

    fn uid_to_widget(&self, uid: WidgetUid) -> WidgetRef {
        for child in self.children.values() {
            let x = child.uid_to_widget(uid);
            if !x.is_empty() {
                return x;
            }
        }
        WidgetRef::empty()
    }

    fn area(&self) -> Area {
        self.draw_divider.area
    }
}

impl LiveHook for GDivider {
    fn before_apply(
        &mut self,
        _cx: &mut Cx,
        apply: &mut Apply,
        _index: usize,
        _nodes: &[LiveNode],
    ) {
        if let ApplyFrom::UpdateFromDoc { .. } = apply.from {
            self.draw_order.clear();
        }
    }
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        // ----------------- background color -------------------------------------------
        let stroke_color = self.stroke_color.get(self.theme, 300);
        // ------------------ hover color -----------------------------------------------
        let stroke_hover_color = self.stroke_hover_color.get(self.theme, 200);
        // ------------------ apply draw_divider --------------------------------------------
        self.draw_divider.apply_over(
            cx,
            live! {
                stroke_color: (stroke_color),
                stroke_hover_color: (stroke_hover_color),
                stroke_width: (self.stroke_width),
            },
        );
        self.draw_divider.redraw(cx);
    }
    fn apply_value_instance(
        &mut self,
        cx: &mut Cx,
        apply: &mut Apply,
        index: usize,
        nodes: &[LiveNode],
    ) -> usize {
        let id = nodes[index].id;
        match apply.from {
            ApplyFrom::Animate | ApplyFrom::Over => {
                if let Some(child) = self.children.get_mut(&id) {
                    child.apply(cx, apply, index, nodes)
                } else {
                    nodes.skip_node(index)
                }
            }
            ApplyFrom::NewFromDoc { .. } | ApplyFrom::UpdateFromDoc { .. } => {
                if nodes[index].is_instance_prop() {
                    self.draw_order.push(id);
                    return self
                        .children
                        .get_or_insert(cx, id, |cx| WidgetRef::new(cx))
                        .apply(cx, apply, index, nodes);
                } else {
                    cx.apply_error_no_matching_field(live_error_origin!(), index, nodes);
                    nodes.skip_node(index)
                }
            }
            _ => nodes.skip_node(index),
        }
    }
}

impl GDivider {
    widget_area! {
        area, draw_divider
    }
    event_option! {
        finger_down: GDividerEvent::FingerDown => FingerDownEvent,
        finger_up: GDividerEvent::FingerUp => FingerUpEvent,
        finger_move : GDividerEvent::FingerMove => FingerMoveEvent,
        finger_hover_in: GDividerEvent::FingerHoverIn => FingerHoverEvent,
        finger_hover_out: GDividerEvent::FingerHoverOut => FingerHoverEvent,
        finger_hover_over: GDividerEvent::FingerHoverOver => FingerHoverEvent,
        key_down: GDividerEvent::KeyDown => KeyEvent,
        key_up: GDividerEvent::KeyUp => KeyEvent
    }
    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
        self.draw_divider.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) -> () {
        self.draw_divider.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
    }
    pub fn handle_widget_event(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
        hit: Hit,
        focus_area: Area,
    ) {
        let uid = self.widget_uid();
        if self.animation_open {
            if self.animator_handle_event(cx, event).must_redraw() {
                self.redraw(cx);
            }
        }

        match &self.event_order {
            EventOrder::Down => {
                for id in self.draw_order.iter() {
                    if let Some(child) = self.children.get_mut(id) {
                        if child.is_visible() || !event.requires_visibility() {
                            scope.with_id(*id, |scope| {
                                child.handle_event(cx, event, scope);
                            })
                        }
                    }
                }
            }
            EventOrder::Up => {
                // the default event order is Up
                for id in self.draw_order.iter().rev() {
                    if let Some(child) = self.children.get_mut(id) {
                        if child.is_visible() || !event.requires_visibility() {
                            scope.with_id(*id, |scope| {
                                child.handle_event(cx, event, scope);
                            });
                        }
                    }
                }
            }
            EventOrder::List(list) => {
                for id in list {
                    if let Some(child) = self.children.get_mut(id) {
                        if child.is_visible() || !event.requires_visibility() {
                            scope.with_id(*id, |scope| {
                                child.handle_event(cx, event, scope);
                            })
                        }
                    }
                }
            }
        }

        // handle event and set cursor to control
        match hit {
            Hit::KeyDown(e) => {
                if self.grab_key_focus {
                    cx.widget_action(uid, &scope.path, GDividerEvent::KeyDown(e));
                }
            }
            Hit::KeyUp(e) => {
                if self.grab_key_focus {
                    cx.widget_action(uid, &scope.path, GDividerEvent::KeyUp(e));
                }
            }
            // Hit::FingerScroll(e) => cx.widget_action(uid, &scope.path, GDividerEvent::FingerScroll(e)),
            Hit::FingerDown(e) => {
                if self.grab_key_focus {
                    cx.set_key_focus(focus_area);
                }
                cx.widget_action(uid, &scope.path, GDividerEvent::FingerDown(e));
            }
            Hit::FingerMove(e) => cx.widget_action(uid, &scope.path, GDividerEvent::FingerMove(e)),
            Hit::FingerHoverIn(e) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                cx.widget_action(uid, &scope.path, GDividerEvent::FingerHoverIn(e));
                self.animator_play(cx, id!(hover.on));
            }
            Hit::FingerHoverOver(e) => {
                cx.widget_action(uid, &scope.path, GDividerEvent::FingerHoverOver(e));
            }
            Hit::FingerHoverOut(e) => {
                cx.widget_action(uid, &scope.path, GDividerEvent::FingerHoverOut(e));
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerUp(e) => {
                cx.widget_action(uid, &scope.path, GDividerEvent::FingerUp(e));
            }
            _ => (),
        }
    }
}

impl GDividerRef {
    ref_event_option! {
        finger_down => FingerDownEvent,
        finger_up => FingerUpEvent,
        finger_move => FingerMoveEvent,
        finger_hover_in => FingerHoverEvent,
        finger_hover_out => FingerHoverEvent,
        finger_hover_over => FingerHoverEvent,
        key_down => KeyEvent,
        key_up => KeyEvent
    }
    animatie_fn! {
        animate_hover_on,
        animate_hover_off
    }
    widget_origin_fn!(GDivider);
    pub fn set_visible(&self, visible: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.visible = visible
        }
    }

    pub fn set_visible_and_redraw(&self, cx: &mut Cx, visible: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.visible = visible;
            inner.redraw(cx);
        }
    }

    pub fn visible(&self) -> bool {
        if let Some(inner) = self.borrow() {
            inner.visible
        } else {
            false
        }
    }

    pub fn area(&self) -> Area {
        if let Some(inner) = self.borrow() {
            inner.area()
        } else {
            Area::Empty
        }
    }
}

impl GDividerSet {
    pub fn redraw(&self, cx: &mut Cx) {
        for item in self.iter() {
            item.redraw(cx);
        }
    }
    set_event! {
        finger_down => FingerDownEvent,
        finger_up => FingerUpEvent,
        finger_move => FingerMoveEvent,
        finger_hover_in => FingerHoverEvent,
        finger_hover_out => FingerHoverEvent,
        finger_hover_over => FingerHoverEvent,
        key_down => KeyEvent,
        key_up => KeyEvent
    }
}
