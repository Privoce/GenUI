pub mod event;
mod register;

use event::*;
pub use register::register;

use std::{cell::RefCell, collections::HashMap};

use makepad_widgets::*;

use crate::{
    event_option, shader::draw_card::DrawGCard, themes::Themes, utils::{set_cursor, BoolToF32, ThemeColor}, widget_area
};

live_design! {
    import makepad_draw::shader::std::*;
    GLOBAL_DURATION = 0.25

    GCardBase = {{GCard}}{
        blur_radius: 100.0,
        spread_radius: 0.0,
        shadow_offset: vec2(0.0, 0.0),
        shadow_color: vec4(0.0, 0.0, 0.0, 0.0),
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_card: {pressed: 0.0, hover: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                        pressed: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_card: {pressed: 0.0, hover: [{time: 0.0, value: 1.0}],}
                    }
                }

                pressed = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_card: {pressed: [{time: 0.0, value: 1.0}], hover: 1.0,}
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveRegisterWidget, WidgetRef, WidgetSet)]
pub struct GCard {
    #[live]
    pub theme: Themes,
    #[live]
    pub background_color: Option<Vec4>,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub pressed_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(0.0)]
    pub border_width: f32,
    #[live(2.0)]
    pub border_radius: f32,
    #[live(true)]
    pub visible: bool,
    #[live(false)]
    pub background_visible: bool,
    #[live]
    pub shadow_color: Option<Vec4>,
    #[live(4.8)]
    pub spread_radius: f32,
    #[live(4.8)]
    pub blur_radius: f32,
    #[live]
    pub shadow_offset: Vec2,
    #[live]
    pub cursor: Option<MouseCursor>,
    #[live(false)]
    pub animation_open: bool,
    // scroll ---------------------
    #[live]
    pub scroll_bars: Option<LivePtr>,
    #[rust]
    pub scroll_bars_obj: Option<Box<ScrollBars>>,
    // control ---------------------
    #[live(true)]
    pub grab_key_focus: bool,
    #[live(false)]
    pub block_signal_event: bool,
    // deref ---------------------
    #[live]
    pub draw_card: DrawGCard,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[rust]
    pub draw_state: DrawStateWrap<DrawState>,
    #[rust]
    pub children: ComponentMap<LiveId, WidgetRef>,
    #[rust]
    pub draw_order: Vec<LiveId>,
    #[live]
    pub event_order: EventOrder,
    #[rust]
    pub defer_walks: Vec<(LiveId, DeferWalk)>,
    #[animator]
    pub animator: Animator,
    #[rust]
    find_cache: RefCell<HashMap<u64, WidgetSet>>,
}

#[derive(Clone)]
pub enum DrawState {
    Drawing(usize, bool),
    DeferWalk(usize),
}

impl LiveHook for GCard {
    fn before_apply(
        &mut self,
        _cx: &mut Cx,
        apply: &mut Apply,
        _index: usize,
        _nodes: &[LiveNode],
    ) {
        if let ApplyFrom::UpdateFromDoc { .. } = apply.from {
            self.draw_order.clear();
            self.find_cache.get_mut().clear();
        }
    }
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.get(self.theme, 500);
        let shadow_color = self.shadow_color.get(self.theme, 700);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.get(self.theme, 400);
        // ------------------ pressed color ---------------------------------------------
        let pressed_color = self.pressed_color.get(self.theme, 600);
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 800);
        // ------------------ is background_visible --------------------------------------------
        let background_visible = self.background_visible.to_f32();
        // ------------------ check scroll bar -------------------------------------------
        if self.scroll_bars.is_some() {
            if self.scroll_bars_obj.is_none() {
                self.scroll_bars_obj =
                    Some(Box::new(ScrollBars::new_from_ptr(cx, self.scroll_bars)));
            }
        }
        // ------------------ apply draw_card --------------------------------------------
        self.draw_card.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                border_color: (border_color),
                border_width: (self.border_width),
                border_radius: (self.border_radius),
                pressed_color: (pressed_color),
                hover_color: (hover_color),
                background_visible: (background_visible),
                shadow_color: (shadow_color),
                shadow_offset: (self.shadow_offset),
                spread_radius: (self.spread_radius),
                blur_radius: (self.blur_radius)
            },
        );
        self.draw_card.redraw(cx);
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

impl Widget for GCard {
    fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
        sweep_area: Area,
    ) {
        let uid = self.widget_uid();
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        if self.block_signal_event {
            if let Event::Signal = event {
                return;
            }
        }

        if let Some(scroll_bars) = &mut self.scroll_bars_obj {
            let mut actions = Vec::new();
            scroll_bars.handle_main_event(cx, event, &mut actions);
            if actions.len().gt(&0) {
                cx.redraw_area_and_children(self.area());
            }
        }

        match &self.event_order {
            EventOrder::Down => {
                for id in self.draw_order.iter() {
                    if let Some(child) = self.children.get_mut(id) {
                        if child.is_visible() || !event.requires_visibility() {
                            scope.with_id(*id, |scope| {
                                child.handle_event_with(cx, event, scope, sweep_area);
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
                                child.handle_event_with(cx, event, scope, sweep_area);
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
                                child.handle_event_with(cx, event, scope, sweep_area);
                            })
                        }
                    }
                }
            }
        }

        // handle event and set cursor to control
        match event.hits_with_options(
            cx,
            self.area(),
            HitOptions::new().with_sweep_area(sweep_area),
        ) {
            Hit::KeyDown(e) => {
                if self.grab_key_focus {
                    cx.widget_action(uid, &scope.path, GCardEvent::KeyDown(e))
                }
            }
            Hit::KeyUp(e) => {
                if self.grab_key_focus {
                    cx.widget_action(uid, &scope.path, GCardEvent::KeyUp(e))
                }
            }
            // Hit::FingerScroll(e) => cx.widget_action(uid, &scope.path, GCardEvent::FingerScroll(e)),
            Hit::FingerDown(e) => {
                if self.grab_key_focus {
                    cx.set_key_focus(sweep_area);
                }
                cx.widget_action(uid, &scope.path, GCardEvent::FingerDown(e));
            }
            Hit::FingerMove(e) => cx.widget_action(uid, &scope.path, GCardEvent::FingerMove(e)),
            Hit::FingerHoverIn(e) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                cx.widget_action(uid, &scope.path, GCardEvent::FingerHoverIn(e));
                if self.animator.live_ptr.is_some() && self.animation_open {
                    self.animator_play(cx, id!(hover.on))
                }
            }
            Hit::FingerHoverOver(e) => {
                cx.widget_action(uid, &scope.path, GCardEvent::FingerHoverOver(e));
            }
            Hit::FingerHoverOut(e) => {
                cx.widget_action(uid, &scope.path, GCardEvent::FingerHoverOut(e));
                if self.animator.live_ptr.is_some() && self.animation_open {
                    self.animator_play(cx, id!(hover.off))
                }
            }
            Hit::FingerUp(e) => {
                cx.widget_action(uid, &scope.path, GCardEvent::FingerUp(e));
            }
            _ => (),
        }
        if let Some(scroll_bars) = &mut self.scroll_bars_obj {
            scroll_bars.handle_scroll_event(cx, event, &mut Vec::new());
        }
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // begin the draw state
        if self.draw_state.begin(cx, DrawState::Drawing(0, false)) {
            if !self.visible {
                // visible is false, so we are done
                self.draw_state.end();
                return DrawStep::done();
            }
            self.defer_walks.clear();

            // get scroll position
            let scroll = if let Some(scroll_bars) = &mut self.scroll_bars_obj {
                scroll_bars.begin_nav_area(cx);
                scroll_bars.get_scroll_pos()
            } else {
                self.layout.scroll
            };

            // begin draw the card
            let _ = self
                .draw_card
                .begin(cx, walk, self.layout.with_scroll(scroll));
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
                let area = self.area();

                if let Some(scroll_bars) = &mut self.scroll_bars_obj {
                    scroll_bars.draw_scroll_bars(cx);
                }

                // draw background
                self.draw_card.end(cx);

                if let Some(scroll_bars) = &mut self.scroll_bars_obj {
                    scroll_bars.set_area(area);
                    scroll_bars.end_nav_area(cx);
                }
            }
            self.draw_state.end();
        }

        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        if self.block_signal_event {
            if let Event::Signal = event {
                return;
            }
        }

        if let Some(scroll_bars) = &mut self.scroll_bars_obj {
            let mut actions = Vec::new();
            scroll_bars.handle_main_event(cx, event, &mut actions);
            if actions.len().gt(&0) {
                cx.redraw_area_and_children(self.area());
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
                    cx.widget_action(uid, &scope.path, GCardEvent::KeyDown(e))
                }
            }
            Hit::KeyUp(e) => {
                if self.grab_key_focus {
                    cx.widget_action(uid, &scope.path, GCardEvent::KeyUp(e))
                }
            }
            // Hit::FingerScroll(e) => cx.widget_action(uid, &scope.path, GCardEvent::FingerScroll(e)),
            Hit::FingerDown(e) => {
                if self.grab_key_focus {
                    cx.set_key_focus(self.area());
                }
                cx.widget_action(uid, &scope.path, GCardEvent::FingerDown(e));
            }
            Hit::FingerMove(e) => cx.widget_action(uid, &scope.path, GCardEvent::FingerMove(e)),
            Hit::FingerHoverIn(e) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                cx.widget_action(uid, &scope.path, GCardEvent::FingerHoverIn(e));
                if self.animator.live_ptr.is_some() && self.animation_open {
                    self.animator_play(cx, id!(hover.on))
                }
            }
            Hit::FingerHoverOver(e) => {
                cx.widget_action(uid, &scope.path, GCardEvent::FingerHoverOver(e));
            }
            Hit::FingerHoverOut(e) => {
                cx.widget_action(uid, &scope.path, GCardEvent::FingerHoverOut(e));
                if self.animator.live_ptr.is_some() && self.animation_open {
                    self.animator_play(cx, id!(hover.off))
                }
            }
            Hit::FingerUp(e) => {
                cx.widget_action(uid, &scope.path, GCardEvent::FingerUp(e));
            }
            _ => (),
        }
        if let Some(scroll_bars) = &mut self.scroll_bars_obj {
            scroll_bars.handle_scroll_event(cx, event, &mut Vec::new());
        }
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl WidgetNode for GCard {
    fn uid_to_widget(&self, uid: WidgetUid) -> WidgetRef {
        for child in self.children.values() {
            let x = child.uid_to_widget(uid);
            if !x.is_empty() {
                return x;
            }
        }
        WidgetRef::empty()
    }

    fn find_widgets(&self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        match cached {
            WidgetCache::Yes | WidgetCache::Clear => {
                if let WidgetCache::Clear = cached {
                    self.find_cache.borrow_mut().clear();
                }
                let mut hash = 0u64;
                for i in 0..path.len() {
                    hash ^= path[i].0
                }
                if let Some(widget_set) = self.find_cache.borrow().get(&hash) {
                    results.extend_from_set(widget_set);
                    return;
                }
                let mut local_results = WidgetSet::empty();
                if let Some(child) = self.children.get(&path[0]) {
                    if path.len() > 1 {
                        child.find_widgets(&path[1..], WidgetCache::No, &mut local_results);
                    } else {
                        local_results.push(child.clone());
                    }
                }
                for child in self.children.values() {
                    child.find_widgets(path, WidgetCache::No, &mut local_results);
                }
                if !local_results.is_empty() {
                    results.extend_from_set(&local_results);
                }
                self.find_cache.borrow_mut().insert(hash, local_results);
            }
            WidgetCache::No => {
                if let Some(child) = self.children.get(&path[0]) {
                    if path.len() > 1 {
                        child.find_widgets(&path[1..], WidgetCache::No, results);
                    } else {
                        results.push(child.clone());
                    }
                }
                for child in self.children.values() {
                    child.find_widgets(path, WidgetCache::No, results);
                }
            }
        }
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        self.walk
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.draw_card.redraw(cx);
        for child in self.children.values_mut() {
            child.redraw(cx);
        }
    }
    fn area(&self)->Area {
        self.draw_card.area
    }
}

impl GCard {
    pub fn set_scroll_pos(&mut self, cx: &mut Cx, v: DVec2) {
        if let Some(scroll_bars) = &mut self.scroll_bars_obj {
            scroll_bars.set_scroll_pos(cx, v);
        } else {
            self.layout.scroll = v;
        }
    }
    pub fn child_count(&self) -> usize {
        self.draw_order.len()
    }
}

impl GCardRef {
    pub fn set_abs_pos(&self, _cx: &mut Cx, abs_pos: DVec2) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.walk.abs_pos.replace(abs_pos);
        }
    }
    event_option! {
        finger_down: ViewAction::FingerDown => FingerDownEvent,
        finger_up: ViewAction::FingerUp => FingerUpEvent
    }
    // pub fn finger_down(&self, actions: &Actions) -> Option<FingerDownEvent> {
    //     if let Some(item) = actions.find_widget_action(self.widget_uid()) {
    //         if let ViewAction::FingerDown(fd) = item.cast() {
    //             return Some(fd);
    //         }
    //     }
    //     None
    // }

    // pub fn finger_up(&self, actions: &Actions) -> Option<FingerUpEvent> {
    //     if let Some(item) = actions.find_widget_action(self.widget_uid()) {
    //         if let ViewAction::FingerUp(fd) = item.cast() {
    //             return Some(fd);
    //         }
    //     }
    //     None
    // }

    pub fn finger_move(&self, actions: &Actions) -> Option<FingerMoveEvent> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let ViewAction::FingerMove(fd) = item.cast() {
                return Some(fd);
            }
        }
        None
    }

    pub fn finger_hover_in(&self, actions: &Actions) -> Option<FingerHoverEvent> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let ViewAction::FingerHoverIn(fd) = item.cast() {
                return Some(fd);
            }
        }
        None
    }

    pub fn finger_hover_out(&self, actions: &Actions) -> Option<FingerHoverEvent> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let ViewAction::FingerHoverOut(fd) = item.cast() {
                return Some(fd);
            }
        }
        None
    }

    pub fn key_down(&self, actions: &Actions) -> Option<KeyEvent> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let ViewAction::KeyDown(fd) = item.cast() {
                return Some(fd);
            }
        }
        None
    }

    pub fn key_up(&self, actions: &Actions) -> Option<KeyEvent> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let ViewAction::KeyUp(fd) = item.cast() {
                return Some(fd);
            }
        }
        None
    }

    pub fn animator_cut(&self, cx: &mut Cx, state: &[LiveId; 2]) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.animator_cut(cx, state);
        }
    }

    pub fn animator_play(&self, cx: &mut Cx, state: &[LiveId; 2]) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.animator_play(cx, state);
        }
    }
    pub fn toggle_state(
        &self,
        cx: &mut Cx,
        is_state_1: bool,
        animate: Animate,
        state1: &[LiveId; 2],
        state2: &[LiveId; 2],
    ) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.animator_toggle(cx, is_state_1, animate, state1, state2);
        }
    }

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
    pub fn set_texture(&self, slot: usize, texture: &Texture) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.draw_card.set_texture(slot, texture);
        }
    }

    pub fn set_uniform(&self, cx: &Cx, uniform: &[LiveId], value: &[f32]) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.draw_card.set_uniform(cx, uniform, value);
        }
    }

    pub fn set_scroll_pos(&self, cx: &mut Cx, v: DVec2) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_scroll_pos(cx, v)
        }
    }
    pub fn area(&self) -> Area {
        if let Some(inner) = self.borrow_mut() {
            inner.area()
        } else {
            Area::Empty
        }
    }

    pub fn child_count(&self) -> usize {
        if let Some(inner) = self.borrow_mut() {
            inner.draw_order.len()
        } else {
            0
        }
    }
}

impl GCardSet {
    pub fn animator_cut(&mut self, cx: &mut Cx, state: &[LiveId; 2]) {
        for item in self.iter() {
            item.animator_cut(cx, state)
        }
    }

    pub fn animator_play(&mut self, cx: &mut Cx, state: &[LiveId; 2]) {
        for item in self.iter() {
            item.animator_play(cx, state);
        }
    }
    pub fn toggle_state(
        &mut self,
        cx: &mut Cx,
        is_state_1: bool,
        animate: Animate,
        state1: &[LiveId; 2],
        state2: &[LiveId; 2],
    ) {
        for item in self.iter() {
            item.toggle_state(cx, is_state_1, animate, state1, state2);
        }
    }

    pub fn set_visible(&self, visible: bool) {
        for item in self.iter() {
            item.set_visible(visible)
        }
    }

    pub fn set_texture(&self, slot: usize, texture: &Texture) {
        for item in self.iter() {
            item.set_texture(slot, texture)
        }
    }

    pub fn set_uniform(&self, cx: &Cx, uniform: &[LiveId], value: &[f32]) {
        for item in self.iter() {
            item.set_uniform(cx, uniform, value)
        }
    }

    pub fn redraw(&self, cx: &mut Cx) {
        for item in self.iter() {
            item.redraw(cx);
        }
    }

    pub fn finger_down(&self, actions: &Actions) -> Option<FingerDownEvent> {
        for item in self.iter() {
            if let Some(e) = item.finger_down(actions) {
                return Some(e);
            }
        }
        None
    }

    pub fn finger_up(&self, actions: &Actions) -> Option<FingerUpEvent> {
        for item in self.iter() {
            if let Some(e) = item.finger_up(actions) {
                return Some(e);
            }
        }
        None
    }

    pub fn finger_move(&self, actions: &Actions) -> Option<FingerMoveEvent> {
        for item in self.iter() {
            if let Some(e) = item.finger_move(actions) {
                return Some(e);
            }
        }
        None
    }

    pub fn key_down(&self, actions: &Actions) -> Option<KeyEvent> {
        for item in self.iter() {
            if let Some(e) = item.key_down(actions) {
                return Some(e);
            }
        }
        None
    }

    pub fn key_up(&self, actions: &Actions) -> Option<KeyEvent> {
        for item in self.iter() {
            if let Some(e) = item.key_up(actions) {
                return Some(e);
            }
        }
        None
    }
}
