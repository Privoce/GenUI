pub mod event;
mod register;

use event::*;
pub use register::register;

use std::{cell::RefCell, collections::HashMap};

use makepad_widgets::*;

use crate::{
    animatie_fn, event_option, ref_event_option, set_event,
    shader::draw_view::DrawGView,
    themes::Themes,
    utils::{set_cursor, BoolToF32, ThemeColor},
    widget_origin_fn,
};

live_design! {
    import makepad_draw::shader::std::*;
    GLOBAL_DURATION = 0.25

    GViewBase = {{GView}}{
        spread_radius: 0.0,
        clip_x: false,
        clip_y: false,
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_view: {pressed: 0.0, hover: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                        pressed: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_view: {pressed: 0.0, hover: [{time: 0.0, value: 1.0}],}
                    }
                }

                pressed = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_view: {pressed: [{time: 0.0, value: 1.0}], hover: 1.0,}
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveRegisterWidget, WidgetRef, WidgetSet)]
pub struct GView {
    #[live(Themes::Dark)]
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
    #[live(true)]
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
    pub draw_view: DrawGView,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[rust]
    pub draw_state: DrawStateWrap<DrawState>,
    // #[rust]
    // pub children: ComponentMap<LiveId, WidgetRef>,
    #[rust]
    pub children: Vec<(LiveId, WidgetRef)>,
    // #[rust]
    // pub draw_order: Vec<LiveId>,
    #[live]
    pub event_order: EventOrder,
    #[rust]
    pub defer_walks: Vec<(LiveId, DeferWalk)>,
    #[animator]
    pub animator: Animator,
    #[rust]
    pub find_cache: RefCell<HashMap<u64, WidgetSet>>,
    // optimize ---------------------
    #[live]
    pub dpi_factor: Option<f64>,
    #[live]
    pub optimize: ViewOptimize,
    #[rust]
    pub draw_list: Option<DrawList2d>,
    #[rust]
    pub view_size: Option<DVec2>,
    #[rust]
    pub texture_cache: Option<ViewTextureCache>,
    #[rust]
    pub area: Area,
    #[rust]
    pub scope_path: HeapLiveIdPath,
}

pub struct ViewTextureCache {
    pass: Pass,
    _depth_texture: Texture,
    color_texture: Texture,
}

pub trait OptimizeFor {
    fn is_texture(&self) -> bool;
    fn is_draw_list(&self) -> bool;
    fn needs_draw_list(&self) -> bool;
}

impl OptimizeFor for ViewOptimize {
    fn is_texture(&self) -> bool {
        if let Self::Texture = self {
            true
        } else {
            false
        }
    }
    fn is_draw_list(&self) -> bool {
        if let Self::DrawList = self {
            true
        } else {
            false
        }
    }
    fn needs_draw_list(&self) -> bool {
        return self.is_texture() || self.is_draw_list();
    }
}

#[derive(Clone)]
pub enum DrawState {
    Drawing(usize, bool),
    DeferWalk(usize),
}

impl LiveHook for GView {
    fn before_apply(
        &mut self,
        _cx: &mut Cx,
        apply: &mut Apply,
        _index: usize,
        _nodes: &[LiveNode],
    ) {
        if let ApplyFrom::UpdateFromDoc { .. } = apply.from {
            // self.draw_order.clear();
            self.find_cache.get_mut().clear();
        }
    }
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.visible {
            return;
        }
        // dbg!("after_apply", &self.scope_path);
        if self.optimize.needs_draw_list() && self.draw_list.is_none() {
            self.draw_list = Some(DrawList2d::new(cx));
        }
        // if self.scroll_bars.is_some() {
        //     if self.scroll_bars_obj.is_none() {
        //         self.scroll_bars_obj =
        //             Some(Box::new(ScrollBars::new_from_ptr(cx, self.scroll_bars)));
        //     }
        // }
        self.render(cx);
        // // ----------------- background color -------------------------------------------
        // let bg_color = self.background_color.get(self.theme, 500);
        // let shadow_color = self.shadow_color.get(self.theme, 700);
        // // ------------------ hover color -----------------------------------------------
        // let hover_color = self.hover_color.get(self.theme, 400);
        // // ------------------ pressed color ---------------------------------------------
        // let pressed_color = self.pressed_color.get(self.theme, 600);
        // // ------------------ border color ----------------------------------------------
        // let border_color = self.border_color.get(self.theme, 600);
        // // ------------------ is background_visible --------------------------------------------
        // let background_visible = self.background_visible.to_f32();
        // // ------------------ check scroll bar -------------------------------------------
        // if self.scroll_bars.is_some() {
        //     if self.scroll_bars_obj.is_none() {
        //         self.scroll_bars_obj =
        //             Some(Box::new(ScrollBars::new_from_ptr(cx, self.scroll_bars)));
        //     }
        // }
        // // ------------------ apply draw_view --------------------------------------------
        // self.draw_view.apply_over(
        //     cx,
        //     live! {
        //         background_color: (bg_color),
        //         background_visible: (background_visible),
        //         border_color: (border_color),
        //         border_width: (self.border_width),
        //         border_radius: (self.border_radius),
        //         pressed_color: (pressed_color),
        //         hover_color: (hover_color),
        //         shadow_color: (shadow_color),
        //         shadow_offset: (self.shadow_offset),
        //         spread_radius: (self.spread_radius),
        //         blur_radius: (self.blur_radius)
        //     },
        // );
        // self.draw_view.redraw(cx);
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
                let node_id = nodes[index].id;
                if let Some((_, component)) =
                    self.children.iter_mut().find(|(id, _)| *id == node_id)
                {
                    component.apply(cx, apply, index, nodes)
                } else {
                    nodes.skip_node(index)
                }
            }
            ApplyFrom::NewFromDoc { .. } | ApplyFrom::UpdateFromDoc { .. } => {
                if nodes[index].is_instance_prop() {
                    //self.draw_order.push(id);
                    if let Some((_, node)) = self.children.iter_mut().find(|(id2, _)| *id2 == id) {
                        node.apply(cx, apply, index, nodes)
                    } else {
                        self.children.push((id, WidgetRef::new(cx)));
                        self.children
                            .last_mut()
                            .unwrap()
                            .1
                            .apply(cx, apply, index, nodes)
                    }
                } else {
                    cx.apply_error_no_matching_field(live_error_origin!(), index, nodes);
                    nodes.skip_node(index)
                }
            }
            _ => nodes.skip_node(index),
        }
    }
}

impl Widget for GView {
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
            scroll_bars.handle_main_event(cx, event, scope, &mut actions);
            if actions.len().gt(&0) {
                cx.redraw_area_and_children(self.area());
            }
        }

        match &self.event_order {
            EventOrder::Down => {
                for (id, child) in self.children.iter_mut() {
                    scope.with_id(*id, |scope| {
                        child.handle_event_with(cx, event, scope, sweep_area);
                    });
                }
            }
            EventOrder::Up => {
                // the default event order is Up
                for (id, child) in self.children.iter_mut().rev() {
                    scope.with_id(*id, |scope| {
                        child.handle_event_with(cx, event, scope, sweep_area);
                    });
                }
            }
            EventOrder::List(list) => {
                for id in list {
                    if let Some((_, child)) = self.children.iter_mut().find(|(id2, _)| id2 == id) {
                        scope.with_id(*id, |scope| {
                            child.handle_event_with(cx, event, scope, sweep_area);
                        });
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
                    cx.widget_action(
                        uid,
                        &scope.path,
                        GViewEvent::KeyDown(GViewKeyEventParam {
                            e,
                            path: scope.path.clone(),
                        }),
                    );
                }
            }
            Hit::KeyUp(e) => {
                if self.grab_key_focus {
                    cx.widget_action(
                        uid,
                        &scope.path,
                        GViewEvent::KeyUp(GViewKeyEventParam {
                            e,
                            path: scope.path.clone(),
                        }),
                    );
                }
            }
            // Hit::FingerScroll(e) => cx.widget_action(uid, &scope.path, GViewEvent::FingerScroll(e)),
            Hit::FingerDown(e) => {
                if self.grab_key_focus {
                    cx.set_key_focus(sweep_area);
                }
                cx.widget_action(
                    uid,
                    &scope.path,
                    GViewEvent::FingerDown(GViewFingerDownParam {
                        e,
                        path: scope.path.clone(),
                    }),
                );
            }
            Hit::FingerMove(e) => cx.widget_action(
                uid,
                &scope.path,
                GViewEvent::FingerMove(GViewFingerMoveParam {
                    e,
                    path: scope.path.clone(),
                }),
            ),
            Hit::FingerHoverIn(e) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                cx.widget_action(
                    uid,
                    &scope.path,
                    GViewEvent::FingerHoverIn(GViewFingerHoverParam {
                        e,
                        path: scope.path.clone(),
                    }),
                );
                if self.animator.live_ptr.is_some() && self.animation_open {
                    self.animator_play(cx, id!(hover.on))
                }
            }
            Hit::FingerHoverOver(e) => {
                cx.widget_action(
                    uid,
                    &scope.path,
                    GViewEvent::FingerHoverOver(GViewFingerHoverParam {
                        e,
                        path: scope.path.clone(),
                    }),
                );
            }
            Hit::FingerHoverOut(e) => {
                cx.widget_action(
                    uid,
                    &scope.path,
                    GViewEvent::FingerHoverOut(GViewFingerHoverParam {
                        e,
                        path: scope.path.clone(),
                    }),
                );
                if self.animator.live_ptr.is_some() && self.animation_open {
                    self.animator_play(cx, id!(hover.off));
                }
            }
            Hit::FingerUp(e) => {
                cx.widget_action(
                    uid,
                    &scope.path,
                    GViewEvent::FingerUp(GViewFingerUpParam {
                        e,
                        path: scope.path.clone(),
                    }),
                );
            }
            _ => (),
        }
        if let Some(scroll_bars) = &mut self.scroll_bars_obj {
            scroll_bars.handle_scroll_event(cx, event, scope, &mut Vec::new());
        }
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // begin the draw state
        if self.draw_state.begin(cx, DrawState::Drawing(0, false)) {
            self.scope_path = scope.path.clone();
            if !self.visible {
                // visible is false, so we are done
                self.draw_state.end();
                return DrawStep::done();
            }
            self.defer_walks.clear();

            match self.optimize {
                ViewOptimize::Texture => {
                    let walk = self.walk_from_previous_size(walk);
                    if !cx.will_redraw(self.draw_list.as_mut().unwrap(), walk) {
                        if let Some(texture_cache) = &self.texture_cache {
                            self.draw_view
                                .draw_vars
                                .set_texture(0, &texture_cache.color_texture);
                            let mut rect = cx.walk_turtle_with_area(&mut self.area, walk);
                            // NOTE(eddyb) see comment lower below for why this is
                            // disabled (it used to match `set_pass_scaled_area`).
                            if false {
                                rect.size *= 2.0 / self.dpi_factor.unwrap_or(1.0);
                            }
                            self.draw_view.draw_abs(cx, rect);
                            self.area = self.draw_view.area();

                            cx.set_pass_area(&texture_cache.pass, self.area);
                        }
                        return DrawStep::done();
                    }
                    // lets start a pass
                    if self.texture_cache.is_none() {
                        self.texture_cache = Some(ViewTextureCache {
                            pass: Pass::new(cx),
                            _depth_texture: Texture::new(cx),
                            color_texture: Texture::new(cx),
                        });
                        let texture_cache = self.texture_cache.as_mut().unwrap();
                        //cache.pass.set_depth_texture(cx, &cache.depth_texture, PassClearDepth::ClearWith(1.0));
                        texture_cache.color_texture = Texture::new_with_format(
                            cx,
                            TextureFormat::RenderBGRAu8 {
                                size: TextureSize::Auto,
                                initial: true,
                            },
                        );
                        texture_cache.pass.add_color_texture(
                            cx,
                            &texture_cache.color_texture,
                            PassClearColor::ClearWith(vec4(0.0, 0.0, 0.0, 0.0)),
                        );
                    }
                    let texture_cache = self.texture_cache.as_mut().unwrap();
                    cx.make_child_pass(&texture_cache.pass);
                    cx.begin_pass(&texture_cache.pass, self.dpi_factor);
                    self.draw_list.as_mut().unwrap().begin_always(cx)
                }
                ViewOptimize::DrawList => {
                    let walk = self.walk_from_previous_size(walk);
                    if self
                        .draw_list
                        .as_mut()
                        .unwrap()
                        .begin(cx, walk)
                        .is_not_redrawing()
                    {
                        cx.walk_turtle_with_area(&mut self.area, walk);
                        return DrawStep::done();
                    }
                }
                _ => (),
            }

            // get scroll position
            let scroll = if let Some(scroll_bars) = &mut self.scroll_bars_obj {
                scroll_bars.begin_nav_area(cx);
                scroll_bars.get_scroll_pos()
            } else {
                self.layout.scroll
            };

            // begin draw the view
            if self.visible {
                self.draw_view
                    .begin(cx, walk, self.layout.with_scroll(scroll)); //.with_scale(2.0 / self.dpi_factor.unwrap_or(2.0)));
            } else {
                cx.begin_turtle(walk, self.layout.with_scroll(scroll)); //.with_scale(2.0 / self.dpi_factor.unwrap_or(2.0)));
            }
        }

        // loop handle the inner children
        while let Some(DrawState::Drawing(step, resume)) = self.draw_state.get() {
            if step < self.children.len() {
                //let id = self.draw_order[step];
                if let Some((id, child)) = self.children.get_mut(step) {
                    if child.is_visible() {
                        let walk = child.walk(cx);
                        if resume {
                            scope.with_id(*id, |scope| child.draw_walk(cx, scope, walk))?;
                        } else if let Some(fw) = cx.defer_walk(walk) {
                            self.defer_walks.push((*id, fw));
                        } else {
                            self.draw_state.set(DrawState::Drawing(step, true));
                            scope.with_id(*id, |scope| child.draw_walk(cx, scope, walk))?;
                        }
                    }
                }
                self.draw_state.set(DrawState::Drawing(step + 1, false));
            } else {
                self.draw_state.set(DrawState::DeferWalk(0));
            }
        }

        // loop handle the defer walk
        while let Some(DrawState::DeferWalk(step)) = self.draw_state.get() {
            if step < self.defer_walks.len() {
                let (id, dw) = &mut self.defer_walks[step];
                if let Some((id, child)) = self.children.iter_mut().find(|(id2, _)| id2 == id) {
                    let walk = dw.resolve(cx);
                    scope.with_id(*id, |scope| child.draw_walk(cx, scope, walk))?;
                }
                self.draw_state.set(DrawState::DeferWalk(step + 1));
            } else {
                if let Some(scroll_bars) = &mut self.scroll_bars_obj {
                    scroll_bars.draw_scroll_bars(cx);
                }
                if self.visible {
                    if self.optimize.is_texture() {
                        panic!("dont use background_visible and texture caching at the same time");
                    }
                    self.draw_view.end(cx);
                    self.area = self.draw_view.area();
                } else {
                    cx.end_turtle_with_area(&mut self.area);
                };

                // // draw background
                // self.draw_view.end(cx);

                if let Some(scroll_bars) = &mut self.scroll_bars_obj {
                    scroll_bars.set_area(self.area);
                    scroll_bars.end_nav_area(cx);
                }

                if self.optimize.needs_draw_list() {
                    let rect = self.area.rect(cx);
                    self.view_size = Some(rect.size);
                    self.draw_list.as_mut().unwrap().end(cx);

                    if self.optimize.is_texture() {
                        let texture_cache = self.texture_cache.as_mut().unwrap();
                        cx.end_pass(&texture_cache.pass);
                        /*if cache.pass.id_equals(4){
                            self.draw_bg.draw_vars.set_uniform(cx, id!(marked),&[1.0]);
                        }
                        else{
                            self.draw_bg.draw_vars.set_uniform(cx, id!(marked),&[0.0]);
                        }*/
                        self.draw_view
                            .draw_vars
                            .set_texture(0, &texture_cache.color_texture);
                        self.draw_view.draw_abs(cx, rect);
                        let area = self.draw_view.area();
                        let texture_cache = self.texture_cache.as_mut().unwrap();
                        /* if false {
                            // FIXME(eddyb) this was the previous logic,
                            // but the only tested apps that use `CachedView`
                            // are sized correctly (regardless of `dpi_factor`)
                            // *without* extra scaling here.
                            cx.set_pass_scaled_area(
                                &texture_cache.pass,
                                area,
                                2.0 / self.dpi_factor.unwrap_or(1.0),
                            );
                        } else {*/
                        cx.set_pass_area(&texture_cache.pass, area);
                        //}
                    }
                }
                self.draw_state.end();
            }
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
            scroll_bars.handle_main_event(cx, event, scope, &mut actions);
            if actions.len().gt(&0) {
                cx.redraw_area_and_children(self.area());
            }
        }

        match &self.event_order {
            EventOrder::Up => {
                for (id, child) in self.children.iter_mut().rev() {
                    scope.with_id(*id, |scope| {
                        child.handle_event(cx, event, scope);
                    });
                }
            }
            EventOrder::Down => {
                for (id, child) in self.children.iter_mut() {
                    // scope.with_id(*id, |scope| {
                    //     child.handle_event(cx, event, scope);
                    // });
                    child.handle_event(cx, event, scope);
                }
            }
            EventOrder::List(list) => {
                for id in list {
                    if let Some((_, child)) = self.children.iter_mut().find(|(id2, _)| id2 == id) {
                        scope.with_id(*id, |scope| {
                            child.handle_event(cx, event, scope);
                        });
                    }
                }
            }
        }

        // handle event and set cursor to control
        match event.hits(cx, self.area()) {
            Hit::KeyDown(e) => {
                if self.grab_key_focus {
                    cx.widget_action(
                        uid,
                        &scope.path,
                        GViewEvent::KeyDown(GViewKeyEventParam {
                            e,
                            path: scope.path.clone(),
                        }),
                    );
                }
            }
            Hit::KeyUp(e) => {
                if self.grab_key_focus {
                    cx.widget_action(
                        uid,
                        &scope.path,
                        GViewEvent::KeyUp(GViewKeyEventParam {
                            e,
                            path: scope.path.clone(),
                        }),
                    );
                }
            }
            // Hit::FingerScroll(e) => cx.widget_action(uid, &scope.path, GViewEvent::FingerScroll(e)),
            Hit::FingerDown(e) => {
                if self.grab_key_focus {
                    cx.set_key_focus(self.area());
                }
                cx.widget_action(
                    uid,
                    &scope.path,
                    GViewEvent::FingerDown(GViewFingerDownParam {
                        e,
                        path: scope.path.clone(),
                    }),
                );
            }
            Hit::FingerMove(e) => cx.widget_action(
                uid,
                &scope.path,
                GViewEvent::FingerMove(GViewFingerMoveParam {
                    e,
                    path: scope.path.clone(),
                }),
            ),
            Hit::FingerHoverIn(e) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                cx.widget_action(
                    uid,
                    &scope.path,
                    GViewEvent::FingerHoverIn(GViewFingerHoverParam {
                        e,
                        path: scope.path.clone(),
                    }),
                );
                if self.animator.live_ptr.is_some() && self.animation_open {
                    self.animator_play(cx, id!(hover.on));
                }
            }
            Hit::FingerHoverOver(e) => {
                cx.widget_action(
                    uid,
                    &scope.path,
                    GViewEvent::FingerHoverOver(GViewFingerHoverParam {
                        e,
                        path: scope.path.clone(),
                    }),
                );
            }
            Hit::FingerHoverOut(e) => {
                cx.widget_action(
                    uid,
                    &scope.path,
                    GViewEvent::FingerHoverOut(GViewFingerHoverParam {
                        e,
                        path: scope.path.clone(),
                    }),
                );
                if self.animator.live_ptr.is_some() && self.animation_open {
                    self.animator_play(cx, id!(hover.off));
                }
            }
            Hit::FingerUp(e) => {
                cx.widget_action(
                    uid,
                    &scope.path,
                    GViewEvent::FingerUp(GViewFingerUpParam {
                        e,
                        path: scope.path.clone(),
                    }),
                );
            }
            _ => (),
        }
        if let Some(scroll_bars) = &mut self.scroll_bars_obj {
            scroll_bars.handle_scroll_event(cx, event, scope, &mut Vec::new());
        }
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl WidgetNode for GView {
    fn uid_to_widget(&self, uid: WidgetUid) -> WidgetRef {
        for (_, child) in &self.children {
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
                if let Some((_, child)) = self.children.iter().find(|(id, _)| *id == path[0]) {
                    if path.len() > 1 {
                        child.find_widgets(&path[1..], WidgetCache::No, &mut local_results);
                    } else {
                        local_results.push(child.clone());
                    }
                }
                for (_, child) in &self.children {
                    child.find_widgets(path, WidgetCache::No, &mut local_results);
                }
                if !local_results.is_empty() {
                    results.extend_from_set(&local_results);
                }
                self.find_cache.borrow_mut().insert(hash, local_results);
            }
            WidgetCache::No => {
                if let Some((_, child)) = self.children.iter().find(|(id, _)| *id == path[0]) {
                    if path.len() > 1 {
                        child.find_widgets(&path[1..], WidgetCache::No, results);
                    } else {
                        results.push(child.clone());
                    }
                }
                for (_, child) in &self.children {
                    child.find_widgets(path, WidgetCache::No, results);
                }
            }
        }
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        self.walk
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.area.redraw(cx);
        self.draw_view.redraw(cx);
        for (_, child) in &self.children {
            child.redraw(cx);
        }
    }
    fn area(&self) -> Area {
        self.area
    }
}

impl GView {
    event_option! {
        finger_down: GViewEvent::FingerDown => GViewFingerDownParam,
        finger_up: GViewEvent::FingerUp => GViewFingerUpParam,
        finger_move : GViewEvent::FingerMove => GViewFingerMoveParam,
        finger_hover_in: GViewEvent::FingerHoverIn => GViewFingerHoverParam,
        finger_hover_out: GViewEvent::FingerHoverOut => GViewFingerHoverParam,
        finger_hover_over: GViewEvent::FingerHoverOver => GViewFingerHoverParam,
        key_down: GViewEvent::KeyDown => GViewKeyEventParam,
        key_up: GViewEvent::KeyUp => GViewKeyEventParam
    }
    pub fn walk_from_previous_size(&self, walk: Walk) -> Walk {
        let view_size = self.view_size.unwrap_or(DVec2::default());
        Walk {
            abs_pos: walk.abs_pos,
            width: if walk.width.is_fill() {
                walk.width
            } else {
                Size::Fixed(view_size.x)
            },
            height: if walk.height.is_fill() {
                walk.height
            } else {
                Size::Fixed(view_size.y)
            },
            margin: walk.margin,
        }
    }
    pub fn set_scroll_pos(&mut self, cx: &mut Cx, v: DVec2) {
        if let Some(scroll_bars) = &mut self.scroll_bars_obj {
            scroll_bars.set_scroll_pos(cx, v);
        } else {
            self.layout.scroll = v;
        }
    }
    pub fn child_count(&self) -> usize {
        self.children.len()
    }
    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
        self.draw_view.apply_over(
            cx,
            live! {
                hover: 1.0,
                pressed: 0.0
            },
        );
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) -> () {
        self.draw_view.apply_over(
            cx,
            live! {
                hover: 0.0,
                pressed: 0.0
            },
        );
    }
    pub fn animate_pressed(&mut self, cx: &mut Cx) -> () {
        self.draw_view.apply_over(
            cx,
            live! {
                hover: 1.0,
                pressed: 1.0
            },
        );
    }
    pub fn render(&mut self, cx: &mut Cx) -> () {
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.get(self.theme, 500);
        let shadow_color = self.shadow_color.get(self.theme, 700);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.get(self.theme, 400);
        // ------------------ pressed color ---------------------------------------------
        let pressed_color = self.pressed_color.get(self.theme, 600);
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 600);
        // ------------------ is background_visible --------------------------------------------
        let background_visible = self.background_visible.to_f32();
        // ------------------ check scroll bar -------------------------------------------
        if self.scroll_bars.is_some() {
            if self.scroll_bars_obj.is_none() {
                self.scroll_bars_obj =
                    Some(Box::new(ScrollBars::new_from_ptr(cx, self.scroll_bars)));
            }
        }
        // ------------------ apply draw_view --------------------------------------------
        self.draw_view.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                background_visible: (background_visible),
                border_color: (border_color),
                border_width: (self.border_width),
                border_radius: (self.border_radius),
                pressed_color: (pressed_color),
                hover_color: (hover_color),
                shadow_color: (shadow_color),
                shadow_offset: (self.shadow_offset),
                spread_radius: (self.spread_radius),
                blur_radius: (self.blur_radius)
            },
        );
        // self.draw_view.redraw(cx);
    }
}

impl GViewRef {
    ref_event_option! {
        finger_down => GViewFingerDownParam,
        finger_up => GViewFingerUpParam,
        finger_move => GViewFingerMoveParam,
        finger_hover_in => GViewFingerHoverParam,
        finger_hover_out => GViewFingerHoverParam,
        finger_hover_over => GViewFingerHoverParam,
        key_down => GViewKeyEventParam,
        key_up => GViewKeyEventParam
    }
    animatie_fn! {
        animate_hover_on,
        animate_hover_off,
        animate_pressed
    }
    widget_origin_fn!(GView);
    pub fn set_abs_pos(&self, _cx: &mut Cx, abs_pos: DVec2) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.walk.abs_pos.replace(abs_pos);
        }
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
            inner.draw_view.set_texture(slot, texture);
        }
    }

    pub fn set_uniform(&self, cx: &Cx, uniform: &[LiveId], value: &[f32]) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.draw_view.set_uniform(cx, uniform, value);
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
            inner.children.len()
        } else {
            0
        }
    }
}

impl GViewSet {
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
    set_event! {
        finger_down => GViewFingerDownParam,
        finger_up => GViewFingerUpParam,
        finger_move => GViewFingerMoveParam,
        finger_hover_in => GViewFingerHoverParam,
        finger_hover_out => GViewFingerHoverParam,
        finger_hover_over => GViewFingerHoverParam,
        key_down => GViewKeyEventParam,
        key_up => GViewKeyEventParam
    }
}