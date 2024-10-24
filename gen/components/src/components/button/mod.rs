mod event;
mod register;

pub use event::*;
pub use register::register;

use crate::utils::{set_cursor, BoolToF32, ThemeColor};
use crate::{
    active_event, animatie_fn, default_handle_animation, default_hit_finger_down, default_hit_finger_up, default_hit_hover_in, default_hit_hover_out, event_option, play_animation, ref_area, ref_event_option, ref_play_animation, ref_redraw, ref_render, set_event, set_scope_path, widget_area
};
use crate::{shader::draw_view::DrawGView, themes::Themes};
use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;
    GLOBAL_DURATION = 0.25

    GButtonBase = {{GButton}}{
        clip_x: false,
        clip_y: false,
        cursor: Hand,
        shadow_offset: vec2(0.0, 2.0),
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_button: {hover: 0.0, focus: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)},
                        focus: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_button: {hover: 1.0, focus: 0.0}
                    }
                }

                focus = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_button: {hover: 0.0, focus: 1.0}
                    }
                }
            }
        }
    }
}

/// # GButton Component
///
/// The `GButton` component is a customizable button designed for interactive UI elements. It supports hover, focus, and click animations, with various configurable properties for appearance, including background color, shadow, and border styles.
///
/// ## Animation
/// This component supports animations, particularly for hover and focus states. The default hover and focus animations are defined using the `animator` field:
/// - **hover.off**:  
///   - `draw_button.hover`: changes to `0.0`  
///   - `draw_button.focus`: changes to `0.0`  
///   - Animation transition: uses `Forward` with a duration of `0.25s`
/// - **hover.on**:  
///   - `draw_button.hover`: changes to `1.0`  
///   - `draw_button.focus`: changes to `0.0`  
///   - Animation transition: uses `Forward` with a duration of `0.25s`
/// - **hover.focus**:  
///   - `draw_button.hover`: changes to `0.0`  
///   - `draw_button.focus`: changes to `1.0`  
///   - Animation transition: uses `Forward` with a duration of `0.25s`
///
/// ## Event
///
/// The `GButton` component supports the following events:
///
/// - **HoverIn**: Triggered when the mouse hovers over the button.
/// - **HoverOut**: Triggered when the mouse leaves the button.
/// - **Clicked**: Triggered when the button is clicked.
/// - **Focus**: Triggered when the button receives focus (e.g., via a keyboard event).
/// - **FocusLost**: Triggered when the button loses focus.
///
/// These events can be customized to trigger additional behaviors based on user interactions.
///
/// ## Props
///
/// | macro  | prop               | description                               | type             | default   |
/// |--------|--------------------|-------------------------------------------|------------------|-----------|
/// | live   | theme              | Theme of the button                       | `Themes`         |           |
/// | live   | background_color   | Background color of the button            | `Option<Vec4>`   | `None`    |
/// | live   | background_visible | Visibility of the background              | `bool`           | `true`    |
/// | live   | hover_color        | Color of the button when hovered          | `Option<Vec4>`   | `None`    |
/// | live   | focus_color        | Color of the button when focused          | `Option<Vec4>`   | `None`    |
/// | live   | shadow_color       | Color of the shadow                       | `Option<Vec4>`   | `None`    |
/// | live   | spread_radius      | Spread radius of the shadow               | `f32`            | `0.0`     |
/// | live   | blur_radius        | Blur radius of the shadow                 | `f32`            | `4.8`     |
/// | live   | shadow_offset      | Offset of the shadow                      | `Vec2`           |           |
/// | live   | border_color       | Color of the border                       | `Option<Vec4>`   | `None`    |
/// | live   | border_width       | Width of the border                       | `f32`            | `0.0`     |
/// | live   | border_radius      | Radius of the border's corners            | `f32`            | `2.0`     |
/// | live   | cursor             | Mouse cursor when hovering over the button| `Option<MouseCursor>`|        |
/// | live   | visible            | Whether the button is visible             | `bool`           | `true`    |
/// | live   | grab_key_focus     | Whether the button grabs keyboard focus   | `bool`           | `true`    |
/// | animator | animator         | Animation controller for the button       | `Animator`       |           |
/// | walk   | `abs_pos`           | Absolute position for layout             | `Option<DVec2>`    | `None`   |
/// | walk   | `margin`            | Margin size around the view              | `Margin`           | `Margin::default()` |
/// | walk   | `width`             | Width of the view                        | `Size`             | `Size::default()` |
/// | walk   | `height`            | Height of the view                       | `Size`             | `Size::default()` |
/// | layout | `scroll`            | Scroll position for layout               | `DVec2`            | `(0.0, 0.0)` |
/// | layout | `clip_x`            | Clip content horizontally                | `bool`             | `true`   |
/// | layout | `clip_y`            | Clip content vertically                  | `bool`             | `true`   |
/// | layout | `padding`           | Padding within the view                  | `Padding`          | `Padding::default()` |
/// | layout | `align`             | Alignment for content                    | `Align`            | `Align::default()` |
/// | layout | `flow`              | Flow direction of the content            | `Flow`             | `Flow::default()` |
/// | layout | `spacing`           | Spacing between elements                 | `f64`              | `0.0`    |
#[derive(Live, Widget)]
pub struct GButton {
    #[live]
    pub theme: Themes,
    #[live]
    pub background_color: Option<Vec4>,
    #[live(true)]
    pub background_visible: bool,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub focus_color: Option<Vec4>,
    #[live]
    pub shadow_color: Option<Vec4>,
    #[live(0.0)]
    pub spread_radius: f32,
    #[live(4.8)]
    pub blur_radius: f32,
    #[live]
    pub shadow_offset: Vec2,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(0.0)]
    pub border_width: f32,
    #[live(2.0)]
    pub border_radius: f32,
    #[live]
    pub cursor: Option<MouseCursor>,
    // visible -------------------
    #[live(true)]
    pub visible: bool,
    // ---------------------------
    #[find]
    #[redraw]
    #[live]
    pub slot: WidgetRef,
    #[live(true)]
    pub grab_key_focus: bool,
    // animator -----------------
    #[live(true)]
    pub animation_key: bool,
    #[animator]
    pub animator: Animator,
    // deref -----------------
    #[redraw]
    #[live]
    pub draw_button: DrawGView,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[live(true)]
    pub event_key: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
}

impl Widget for GButton {
    fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        _scope: &mut Scope,
        sweep_area: Area,
    ) {
        if !self.visible {
            return;
        }
        let hit = event.hits_with_options(
            cx,
            self.area(),
            HitOptions::new().with_sweep_area(sweep_area),
        );

        self.handle_widget_event(cx, event, hit, sweep_area)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if !self.visible {
            return;
        }
        let focus_area = self.area();
        let hit = event.hits(cx, self.area());
        self.handle_widget_event(cx, event, hit, focus_area)
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        let _ = self.draw_button.begin(cx, walk, self.layout);

        if self.slot.is_visible() {
            let slot_walk = self.slot.walk(cx);
            let _ = self.slot.draw_walk(cx, scope, slot_walk);
        }

        self.draw_button.end(cx);

        self.set_scope_path(&scope.path);

        DrawStep::done()
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GButton {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.visible {
            return;
        }
        self.render(cx);
    }
}

impl GButton {
    set_scope_path!();
    play_animation!();
    widget_area! {
        area, draw_button,
        area_slot, slot
    }
    event_option! {
        hover_in: GButtonEvent::HoverIn => GButtonHoverParam,
        hover_out: GButtonEvent::HoverOut => GButtonHoverParam,
        focus: GButtonEvent::Focus => GButtonFocusParam,
        focus_lost: GButtonEvent::FocusLost => GButtonFocusLostParam,
        clicked: GButtonEvent::Clicked => GButtonClickedParam
    }
    active_event! {
        active_hover_in: GButtonEvent::HoverIn |e: FingerHoverEvent| => GButtonHoverParam {e},
        active_hover_out: GButtonEvent::HoverOut |e: FingerHoverEvent| => GButtonHoverParam {e},
        active_focus: GButtonEvent::Focus |e: FingerDownEvent| => GButtonFocusParam {e},
        active_focus_lost: GButtonEvent::FocusLost |e: FingerUpEvent| => GButtonFocusLostParam {e},
        active_clicked: GButtonEvent::Clicked |e: FingerUpEvent| => GButtonClickedParam {e}
    }
    pub fn render(&mut self, cx: &mut Cx) {
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.get(self.theme, 500);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.get(self.theme, 400);
        // ------------------ focus color ---------------------------------------------
        let focus_color = self.focus_color.get(self.theme, 600);
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 600);
        let shadow_color = self.shadow_color.get(self.theme, 700);
        let background_visible = self.background_visible.to_f32();
        // apply over props to draw_button ----------------------------------------------
        self.draw_button.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                background_visible: (background_visible),
                border_color: (border_color),
                border_width: (self.border_width),
                border_radius: (self.border_radius),
                focus_color: (focus_color),
                hover_color: (hover_color),
                shadow_color: (shadow_color),
                shadow_offset: (self.shadow_offset),
                spread_radius: (self.spread_radius),
                blur_radius: (self.blur_radius)
            },
        );
    }
    pub fn clear_animation(&mut self, cx: &mut Cx) {
        self.draw_button.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
    }
    pub fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, focus_area: Area) {
        default_handle_animation!(self, cx, event);
        match hit {
            Hit::FingerDown(e) => {
                default_hit_finger_down!(self, cx, focus_area, e);
            }
            Hit::FingerHoverIn(e) => {
                default_hit_hover_in!(self, cx, e);
            }
            Hit::FingerHoverOut(e) => {
                default_hit_hover_out!(self, cx, e);
            }
            Hit::FingerUp(e) => {
                default_hit_finger_up!(self, cx, e);
            }
            _ => (),
        }
    }
    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.draw_button.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) -> () {
        self.draw_button.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
    }
    pub fn animate_focus_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.draw_button.apply_over(
            cx,
            live! {
                focus: 1.0
            },
        );
    }
    pub fn animate_focus_off(&mut self, cx: &mut Cx) -> () {
        self.draw_button.apply_over(
            cx,
            live! {
                focus: 0.0
            },
        );
    }
    pub fn redraw(&self, cx: &mut Cx) -> () {
        self.draw_button.redraw(cx);
        if self.slot.is_visible() {
            self.slot.redraw(cx);
        }
    }
}

impl GButtonRef {
    ref_event_option! {
        hover_in => GButtonHoverParam,
        hover_out => GButtonHoverParam,
        focus => GButtonFocusParam,
        focus_lost => GButtonFocusLostParam,
        clicked => GButtonClickedParam
    }
    ref_area!();
    ref_render!();
    ref_redraw!();
    animatie_fn! {
        clear_animation,
        animate_hover_on,
        animate_hover_off,
        animate_focus_on,
        animate_focus_off
    }
    ref_play_animation!{
        play_hover_on: id!(hover.on),
        play_hover_off: id!(hover.off),
        play_focus_on: id!(hover.focus),
        play_focus_off: id!(hover.off)
    }
}

impl GButtonSet {
    set_event! {
        hover_in => GButtonHoverParam,
        hover_out => GButtonHoverParam,
        focus => GButtonFocusParam,
        focus_lost => GButtonFocusLostParam,
        clicked => GButtonClickedParam
    }
}
