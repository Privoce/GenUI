mod event;
mod register;

pub use event::*;
use makepad_widgets::*;
pub use register::register;

use crate::{
    active_event, animatie_fn, default_handle_animation, default_hit_finger_down,
    default_hit_finger_up, default_hit_hover_in, default_hit_hover_out, event_option,
    play_animation, ref_area, ref_event_option, ref_redraw, ref_render, set_event, set_scope_path,
    shader::icon_lib::{
        arrow::DrawGIconArrow,
        base::DrawGIconBase,
        code::DrawGIconCode,
        emoji::DrawGIconEmoji,
        fs::DrawGIconFs,
        person::DrawGIconPerson,
        relation::DrawGIconRelation,
        state::DrawGIconState,
        time::DrawGIconTime,
        tool::DrawGIconTool,
        types::{DrawGIconType, IconType},
        ui::DrawGIconUI,
        ApplyIconType,
    },
    themes::Themes,
    utils::{set_cursor, ThemeColor},
    widget_area,
};

live_design! {
    import makepad_draw::shader::std::*;
    GLOBAL_DURATION = 0.25
    GIconBase = {{GIcon}}{
        draw_icon: {
            instance hover: 0.0,
            instance focus: 0.0,
            fn pixel(self) -> vec4{
                return vec4(0.0);
            }
        },
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_icon: {hover: 0.0, focus: 0.0},
                        icon_base: {hover: 0.0, focus: 0.0},
                        icon_arrow: {hover: 0.0, focus: 0.0},
                        icon_code: {hover: 0.0, focus: 0.0},
                        icon_emoji: {hover: 0.0, focus: 0.0},
                        icon_fs: {hover: 0.0, focus: 0.0},
                        icon_ui: {hover: 0.0, focus: 0.0},
                        icon_person: {hover: 0.0, focus: 0.0},
                        icon_relation: {hover: 0.0, focus: 0.0},
                        icon_state: {hover: 0.0, focus: 0.0},
                        icon_time: {hover: 0.0, focus: 0.0},
                        icon_tool: {hover: 0.0, focus: 0.0},
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                        focus: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_icon: {hover: 1.0, focus: 0.0}
                        icon_base: {hover: 1.0, focus: 0.0},
                        icon_arrow: {hover: 1.0, focus: 0.0},
                        icon_code: {hover: 1.0, focus: 0.0},
                        icon_emoji: {hover: 1.0, focus: 0.0},
                        icon_fs: {hover: 1.0, focus: 0.0},
                        icon_ui: {hover: 1.0, focus: 0.0},
                        icon_person: {hover: 1.0, focus: 0.0},
                        icon_relation: {hover: 1.0, focus: 0.0},
                        icon_state: {hover: 1.0, focus: 0.0},
                        icon_time: {hover: 1.0, focus: 0.0},
                        icon_tool: {hover: 1.0, focus: 0.0},
                    }
                }

                focus = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_icon: {hover: 0.0, focus: 1.0}
                        icon_base: {hover: 0.0, focus: 1.0},
                        icon_arrow: {hover: 0.0, focus: 1.0},
                        icon_code: {hover: 0.0, focus: 1.0},
                        icon_emoji: {hover: 0.0, focus: 1.0},
                        icon_fs: {hover: 0.0, focus: 1.0},
                        icon_ui: {hover: 0.0, focus: 1.0},
                        icon_person: {hover: 0.0, focus: 1.0},
                        icon_relation: {hover: 0.0, focus: 1.0},
                        icon_state: {hover: 0.0, focus: 1.0},
                        icon_time: {hover: 0.0, focus: 1.0},
                        icon_tool: {hover: 0.0, focus: 1.0},
                    }
                }
            }
        }
    }
}

/// # GIcon component
/// The `GIcon` component is a customizable graphical icon with support for animations, events, and various properties to control its appearance and behavior.
///
/// ## Animation
/// The `GIcon` component provides built-in animations for hover and focus effects. These animations transition smoothly between states based on user interactions.
///
/// - `hover`: 
///   - **off**: Transitions to a state where the `hover` effect is disabled across all icons.
///   - **on**: Applies a `hover` effect to the icons, transitioning them to a highlighted state.
///   - **focus**: Applies a `focus` effect, transitioning the icons into focus mode.
///  
/// ## Event
/// The `GIcon` component supports various interaction events, enabling developers to listen and respond to user actions.
///
/// - `HoverIn(GIconHoverParam)`: Triggered when the icon is hovered over.
/// - `HoverOut(GIconHoverParam)`: Triggered when the hover effect is lost.
/// - `Focus(GIconFocusParam)`: Triggered when the icon gains focus.
/// - `Clicked(GIconClickedParam)`: Triggered when the icon is clicked.
/// - `FocusLost(GIconFocusLostParam)`: Triggered when the icon loses focus.
///
/// ## Props
///
/// | macro  | prop               | description                                             | type               | default |
/// |--------|--------------------|---------------------------------------------------------|--------------------|---------|
/// | live   | theme               | Themes for styling the component                        | `Themes`           | -       |
/// | live   | color               | Icon color                                              | `Option<Vec4>`      | `None`  |
/// | live   | stroke_hover_color  | Stroke color on hover                                   | `Option<Vec4>`      | `None`  |
/// | live   | stroke_focus_color  | Stroke color on focus                                   | `Option<Vec4>`      | `None`  |
/// | live   | stroke_width        | Stroke width                                            | `f32`               | `1.0`   |
/// | live   | cursor              | Icon cursor style                                       | `Option<MouseCursor>`| `None`  |
/// | live   | visible             | Controls visibility of the icon                         | `bool`              | `true`  |
/// | live   | grab_key_focus      | Determines if the icon grabs key focus                  | `bool`              | `true`  |
/// | live   | animation_key       | Animation key control                                   | `bool`              | `false` |
/// | animator | animator          | Manages icon animations                                 | `Animator`          | -       |
/// | redraw | draw_icon           | Handles redrawing the icon                              | `DrawQuad`          | -       |
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
/// | live   | icon_base           | Base icon drawable                                      | `Option<DrawGIconBase>`| `None` |
/// | live   | icon_arrow          | Arrow icon drawable                                     | `Option<DrawGIconArrow>`| `None` |
/// | live   | icon_code           | Code icon drawable                                      | `Option<DrawGIconCode>` | `None` |
/// | live   | icon_emoji          | Emoji icon drawable                                     | `Option<DrawGIconEmoji>`| `None` |
/// | live   | icon_fs             | File system icon drawable                               | `Option<DrawGIconFs>`  | `None` |
/// | live   | icon_ui             | UI icon drawable                                        | `Option<DrawGIconUI>`  | `None` |
/// | live   | icon_person         | Person icon drawable                                    | `Option<DrawGIconPerson>`| `None` |
/// | live   | icon_relation       | Relation icon drawable                                  | `Option<DrawGIconRelation>`| `None` |
/// | live   | icon_state          | State icon drawable                                     | `Option<DrawGIconState>`| `None` |
/// | live   | icon_time           | Time icon drawable                                      | `Option<DrawGIconTime>`| `None` |
/// | live   | icon_tool           | Tool icon drawable                                      | `Option<DrawGIconTool>`| `None` |
/// | rust   | draw_type           | Controls the type of icon being drawn                   | `Option<DrawGIconType>`| `None` |
/// | live   | icon_type           | Specifies the type of icon                              | `IconType`          | -       |
/// | live   | event_key           | Event key control                                       | `bool`              | `true`  |
/// | rust   | scope_path          | Path to scope the icon                                  | `Option<HeapLiveIdPath>`| `None` |
#[derive(Live, Widget)]
pub struct GIcon {
    #[live]
    pub theme: Themes,
    #[live]
    pub color: Option<Vec4>,
    #[live]
    pub stroke_hover_color: Option<Vec4>,
    #[live]
    pub stroke_focus_color: Option<Vec4>,
    #[live(1.0)]
    pub stroke_width: f32,
    #[live]
    pub cursor: Option<MouseCursor>,
    #[live(true)]
    pub visible: bool,
    #[live(true)]
    pub grab_key_focus: bool,
    #[live(false)]
    pub animation_key: bool,
    #[animator]
    pub animator: Animator,
    // redraw -------------------------------------
    #[redraw]
    #[live]
    pub draw_icon: DrawQuad,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    // icon lib draw shader ------------------------
    #[live]
    icon_base: Option<DrawGIconBase>,
    #[live]
    icon_arrow: Option<DrawGIconArrow>,
    #[live]
    icon_code: Option<DrawGIconCode>,
    #[live]
    icon_emoji: Option<DrawGIconEmoji>,
    #[live]
    icon_fs: Option<DrawGIconFs>,
    #[live]
    icon_ui: Option<DrawGIconUI>,
    #[live]
    icon_person: Option<DrawGIconPerson>,
    #[live]
    icon_relation: Option<DrawGIconRelation>,
    #[live]
    icon_state: Option<DrawGIconState>,
    #[live]
    icon_time: Option<DrawGIconTime>,
    #[live]
    icon_tool: Option<DrawGIconTool>,
    // draw icon type ------------------------------
    #[rust]
    pub draw_type: Option<DrawGIconType>,
    // icon type -----------------------------------
    #[live]
    pub icon_type: IconType,
    #[live(true)]
    pub event_key: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
}

impl Widget for GIcon {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        self.set_scope_path(&scope.path);
        self.draw_icon.begin(cx, walk, self.layout);
        match self.draw_type.as_ref().unwrap() {
            crate::shader::icon_lib::types::DrawGIconType::Base => {
                self.icon_base
                    .as_mut()
                    .unwrap()
                    .begin(cx, walk, self.layout);
                self.icon_base.as_mut().unwrap().end(cx);
            }
            crate::shader::icon_lib::types::DrawGIconType::Code => {
                self.icon_code
                    .as_mut()
                    .unwrap()
                    .begin(cx, walk, self.layout);
                self.icon_code.as_mut().unwrap().end(cx);
            }
            crate::shader::icon_lib::types::DrawGIconType::Arrow => {
                self.icon_arrow
                    .as_mut()
                    .unwrap()
                    .begin(cx, walk, self.layout);
                self.icon_arrow.as_mut().unwrap().end(cx);
            }
            crate::shader::icon_lib::types::DrawGIconType::Emoji => {
                self.icon_emoji
                    .as_mut()
                    .unwrap()
                    .begin(cx, walk, self.layout);
                self.icon_emoji.as_mut().unwrap().end(cx);
            }
            crate::shader::icon_lib::types::DrawGIconType::Fs => {
                self.icon_fs.as_mut().unwrap().begin(cx, walk, self.layout);
                self.icon_fs.as_mut().unwrap().end(cx);
            }
            crate::shader::icon_lib::types::DrawGIconType::UI => {
                self.icon_ui.as_mut().unwrap().begin(cx, walk, self.layout);
                self.icon_ui.as_mut().unwrap().end(cx);
            }
            crate::shader::icon_lib::types::DrawGIconType::Person => {
                self.icon_person
                    .as_mut()
                    .unwrap()
                    .begin(cx, walk, self.layout);
                self.icon_person.as_mut().unwrap().end(cx);
            }
            crate::shader::icon_lib::types::DrawGIconType::Relation => {
                self.icon_relation
                    .as_mut()
                    .unwrap()
                    .begin(cx, walk, self.layout);
                self.icon_relation.as_mut().unwrap().end(cx);
            }
            crate::shader::icon_lib::types::DrawGIconType::State => {
                self.icon_state
                    .as_mut()
                    .unwrap()
                    .begin(cx, walk, self.layout);
                self.icon_state.as_mut().unwrap().end(cx);
            }
            crate::shader::icon_lib::types::DrawGIconType::Time => {
                self.icon_time
                    .as_mut()
                    .unwrap()
                    .begin(cx, walk, self.layout);
                self.icon_time.as_mut().unwrap().end(cx);
            }
            crate::shader::icon_lib::types::DrawGIconType::Tool => {
                self.icon_tool
                    .as_mut()
                    .unwrap()
                    .begin(cx, walk, self.layout);
                self.icon_tool.as_mut().unwrap().end(cx);
            }
        }
        self.draw_icon.end(cx);
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
        let focus_area = self.area();
        let hit = event.hits(cx, self.area());
        self.handle_widget_event(cx, event, scope, hit, focus_area)
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GIcon {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        if !self.visible {
            return;
        }
        match self.icon_type.to_draw_type() {
            Ok(ty) => {
                self.draw_type.replace(ty);
            }
            Err(e) => {
                cx.apply_error(live_error_origin!(), index, nodes, e.to_string());
            }
        }
        self.render(cx);
    }
}

impl GIcon {
    set_scope_path!();
    play_animation!();
    widget_area! {
        area, draw_icon
    }
    event_option! {
        clicked: GIconEvent::Clicked => GIconClickedParam,
        hover_in: GIconEvent::HoverIn => GIconHoverParam,
        hover_out: GIconEvent::HoverOut => GIconHoverParam,
        focus: GIconEvent::Focus => GIconFocusParam,
        focus_lost: GIconEvent::FocusLost => GIconFocusLostParam
    }
    active_event! {
        active_hover_in: GIconEvent::HoverIn |e: FingerHoverEvent| => GIconHoverParam{ e },
        active_hover_out: GIconEvent::HoverOut |e: FingerHoverEvent| => GIconHoverParam{ e },
        active_focus: GIconEvent::Focus |e: FingerDownEvent| => GIconFocusParam{ e },
        active_focus_lost: GIconEvent::FocusLost |e: FingerUpEvent| => GIconFocusLostParam{ e },
        active_clicked: GIconEvent::Clicked |e: FingerUpEvent| => GIconClickedParam{ e }
    }
    pub fn render(&mut self, cx: &mut Cx) {
        fn handle<T>(
            target: &mut Option<T>,
            cx: &mut Cx,
            colors: [Vec4; 3],
            stroke_width: f32,
            icon_type: &IconType,
        ) -> ()
        where
            T: LiveApply + ApplyIconType,
        {
            target.as_mut().unwrap().apply_over(
                cx,
                live! {
                    color: (colors[0]),
                    stroke_width: (stroke_width),
                    stroke_hover_color: (colors[1]),
                    stroke_focus_color: (colors[2]),
                },
            );

            let _ = target.as_mut().unwrap().apply_type(icon_type);
        }

        let colors = [
            self.color.get(self.theme, 50),
            self.stroke_hover_color.get(self.theme, 25),
            self.stroke_focus_color.get(self.theme, 100),
        ];

        match self.draw_type.as_ref().unwrap() {
            crate::shader::icon_lib::types::DrawGIconType::Base => {
                handle(
                    &mut self.icon_base,
                    cx,
                    colors,
                    self.stroke_width,
                    &self.icon_type,
                );
            }
            crate::shader::icon_lib::types::DrawGIconType::Code => {
                handle(
                    &mut self.icon_code,
                    cx,
                    colors,
                    self.stroke_width,
                    &self.icon_type,
                );
            }
            crate::shader::icon_lib::types::DrawGIconType::Arrow => {
                handle(
                    &mut self.icon_arrow,
                    cx,
                    colors,
                    self.stroke_width,
                    &self.icon_type,
                );
            }
            crate::shader::icon_lib::types::DrawGIconType::Emoji => {
                handle(
                    &mut self.icon_emoji,
                    cx,
                    colors,
                    self.stroke_width,
                    &self.icon_type,
                );
            }
            crate::shader::icon_lib::types::DrawGIconType::Fs => {
                handle(
                    &mut self.icon_fs,
                    cx,
                    colors,
                    self.stroke_width,
                    &self.icon_type,
                );
            }
            crate::shader::icon_lib::types::DrawGIconType::UI => {
                handle(
                    &mut self.icon_ui,
                    cx,
                    colors,
                    self.stroke_width,
                    &self.icon_type,
                );
            }
            crate::shader::icon_lib::types::DrawGIconType::Person => {
                handle(
                    &mut self.icon_person,
                    cx,
                    colors,
                    self.stroke_width,
                    &self.icon_type,
                );
            }
            crate::shader::icon_lib::types::DrawGIconType::Relation => {
                handle(
                    &mut self.icon_relation,
                    cx,
                    colors,
                    self.stroke_width,
                    &self.icon_type,
                );
            }
            crate::shader::icon_lib::types::DrawGIconType::State => {
                handle(
                    &mut self.icon_state,
                    cx,
                    colors,
                    self.stroke_width,
                    &self.icon_type,
                );
            }
            crate::shader::icon_lib::types::DrawGIconType::Time => {
                handle(
                    &mut self.icon_time,
                    cx,
                    colors,
                    self.stroke_width,
                    &self.icon_type,
                );
            }
            crate::shader::icon_lib::types::DrawGIconType::Tool => {
                handle(
                    &mut self.icon_tool,
                    cx,
                    colors,
                    self.stroke_width,
                    &self.icon_type,
                );
            }
        }
    }
    pub fn redraw(&self, cx: &mut Cx) {
        // self.draw_icon.redraw(cx);
        match self.draw_type.as_ref().unwrap() {
            DrawGIconType::Base => {
                self.icon_base.as_ref().unwrap().redraw(cx);
            }
            DrawGIconType::Code => {
                self.icon_code.as_ref().unwrap().redraw(cx);
            }
            DrawGIconType::Arrow => {
                self.icon_arrow.as_ref().unwrap().redraw(cx);
            }
            DrawGIconType::Emoji => {
                self.icon_emoji.as_ref().unwrap().redraw(cx);
            }
            DrawGIconType::Fs => {
                self.icon_fs.as_ref().unwrap().redraw(cx);
            }
            DrawGIconType::UI => {
                self.icon_ui.as_ref().unwrap().redraw(cx);
            }
            DrawGIconType::Person => {
                self.icon_person.as_ref().unwrap().redraw(cx);
            }
            DrawGIconType::Relation => {
                self.icon_relation.as_ref().unwrap().redraw(cx);
            }
            DrawGIconType::State => {
                self.icon_state.as_ref().unwrap().redraw(cx);
            }
            DrawGIconType::Time => {
                self.icon_time.as_ref().unwrap().redraw(cx);
            }
            DrawGIconType::Tool => {
                self.icon_tool.as_ref().unwrap().redraw(cx);
            }
        }
    }
    pub fn clear_animation(&mut self, cx: &mut Cx) {
        self.draw_icon.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0,
            },
        );
    }

    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        let hover = live! {
            hover: 1.0,
        };
        match self.draw_type.as_ref().unwrap() {
            DrawGIconType::Base => {
                handle_animate(&mut self.icon_base, cx, hover);
            }
            DrawGIconType::Code => {
                handle_animate(&mut self.icon_code, cx, hover);
            }
            DrawGIconType::Arrow => {
                handle_animate(&mut self.icon_arrow, cx, hover);
            }
            DrawGIconType::Emoji => {
                handle_animate(&mut self.icon_emoji, cx, hover);
            }
            DrawGIconType::Fs => {
                handle_animate(&mut self.icon_fs, cx, hover);
            }
            DrawGIconType::UI => {
                handle_animate(&mut self.icon_ui, cx, hover);
            }
            DrawGIconType::Person => {
                handle_animate(&mut self.icon_person, cx, hover);
            }
            DrawGIconType::Relation => {
                handle_animate(&mut self.icon_relation, cx, hover);
            }
            DrawGIconType::State => {
                handle_animate(&mut self.icon_state, cx, hover);
            }
            DrawGIconType::Time => {
                handle_animate(&mut self.icon_time, cx, hover);
            }
            DrawGIconType::Tool => {
                handle_animate(&mut self.icon_tool, cx, hover);
            }
        }
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) -> () {
        let hover = live! {
            hover: 0.0,
        };
        match self.draw_type.as_ref().unwrap() {
            DrawGIconType::Base => {
                handle_animate(&mut self.icon_base, cx, hover);
            }
            DrawGIconType::Code => {
                handle_animate(&mut self.icon_code, cx, hover);
            }
            DrawGIconType::Arrow => {
                handle_animate(&mut self.icon_arrow, cx, hover);
            }
            DrawGIconType::Emoji => {
                handle_animate(&mut self.icon_emoji, cx, hover);
            }
            DrawGIconType::Fs => {
                handle_animate(&mut self.icon_fs, cx, hover);
            }
            DrawGIconType::UI => {
                handle_animate(&mut self.icon_ui, cx, hover);
            }
            DrawGIconType::Person => {
                handle_animate(&mut self.icon_person, cx, hover);
            }
            DrawGIconType::Relation => {
                handle_animate(&mut self.icon_relation, cx, hover);
            }
            DrawGIconType::State => {
                handle_animate(&mut self.icon_state, cx, hover);
            }
            DrawGIconType::Time => {
                handle_animate(&mut self.icon_time, cx, hover);
            }
            DrawGIconType::Tool => {
                handle_animate(&mut self.icon_tool, cx, hover);
            }
        }
    }
    pub fn animate_focus_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        let focus = live! {
            focus: 1.0,
        };
        match self.draw_type.as_ref().unwrap() {
            DrawGIconType::Base => {
                handle_animate(&mut self.icon_base, cx, focus);
            }
            DrawGIconType::Code => {
                handle_animate(&mut self.icon_code, cx, focus);
            }
            DrawGIconType::Arrow => {
                handle_animate(&mut self.icon_arrow, cx, focus);
            }
            DrawGIconType::Emoji => {
                handle_animate(&mut self.icon_emoji, cx, focus);
            }
            DrawGIconType::Fs => {
                handle_animate(&mut self.icon_fs, cx, focus);
            }
            DrawGIconType::UI => {
                handle_animate(&mut self.icon_ui, cx, focus);
            }
            DrawGIconType::Person => {
                handle_animate(&mut self.icon_person, cx, focus);
            }
            DrawGIconType::Relation => {
                handle_animate(&mut self.icon_relation, cx, focus);
            }
            DrawGIconType::State => {
                handle_animate(&mut self.icon_state, cx, focus);
            }
            DrawGIconType::Time => {
                handle_animate(&mut self.icon_time, cx, focus);
            }
            DrawGIconType::Tool => {
                handle_animate(&mut self.icon_tool, cx, focus);
            }
        }
    }
    pub fn animate_focus_off(&mut self, cx: &mut Cx) -> () {
        let focus = live! {
            focus: 0.0,
        };
        match self.draw_type.as_ref().unwrap() {
            DrawGIconType::Base => {
                handle_animate(&mut self.icon_base, cx, focus);
            }
            DrawGIconType::Code => {
                handle_animate(&mut self.icon_code, cx, focus);
            }
            DrawGIconType::Arrow => {
                handle_animate(&mut self.icon_arrow, cx, focus);
            }
            DrawGIconType::Emoji => {
                handle_animate(&mut self.icon_emoji, cx, focus);
            }
            DrawGIconType::Fs => {
                handle_animate(&mut self.icon_fs, cx, focus);
            }
            DrawGIconType::UI => {
                handle_animate(&mut self.icon_ui, cx, focus);
            }
            DrawGIconType::Person => {
                handle_animate(&mut self.icon_person, cx, focus);
            }
            DrawGIconType::Relation => {
                handle_animate(&mut self.icon_relation, cx, focus);
            }
            DrawGIconType::State => {
                handle_animate(&mut self.icon_state, cx, focus);
            }
            DrawGIconType::Time => {
                handle_animate(&mut self.icon_time, cx, focus);
            }
            DrawGIconType::Tool => {
                handle_animate(&mut self.icon_tool, cx, focus);
            }
        }
    }
    pub fn handle_widget_event(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        _scope: &mut Scope,
        hit: Hit,
        focus_area: Area,
    ) {
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
}

impl GIconRef {
    ref_area!();
    ref_redraw!();
    ref_render!();
    ref_event_option! {
        hover_in => GIconHoverParam,
        hover_out => GIconHoverParam,
        focus => GIconFocusParam,
        focus_lost => GIconFocusLostParam,
        clicked => GIconClickedParam
    }
    animatie_fn! {
        clear_animation,
        animate_hover_on,
        animate_hover_off,
        animate_focus_on,
        animate_focus_off
    }
}

impl GIconSet {
    set_event! {
        hover_in => GIconHoverParam,
        hover_out => GIconHoverParam,
        focus => GIconFocusParam,
        focus_lost => GIconFocusLostParam,
        clicked => GIconClickedParam
    }
}

fn handle_animate<T>(target: &mut Option<T>, cx: &mut Cx, nodes: &[LiveNode]) -> ()
where
    T: LiveApply,
{
    target.as_mut().unwrap().apply_over(cx, nodes);
}
