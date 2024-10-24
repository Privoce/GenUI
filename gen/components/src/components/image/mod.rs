mod event;
mod register;

pub use event::*;
pub use register::register;

use image_cache::{ImageCacheImpl, ImageFit};
use makepad_widgets::*;

use crate::{
    active_event, event_option, ref_area, ref_event_option, ref_redraw, ref_render, set_event, set_scope_path, shader::draw_view::DrawGView, utils::set_cursor, widget_area
};

live_design! {
    import makepad_draw::shader::std::*;
    GImageBase = {{GImage}} {
        draw_image: {
            texture image: texture2d

            fn rotation_vertex_expansion(rotation: float, w: float, h: float) -> vec2 {
                let horizontal_expansion = (abs(cos(rotation)) * w + abs(sin(rotation)) * h) / w - 1.0;
                let vertical_expansion = (abs(sin(rotation)) * w + abs(cos(rotation)) * h) / h - 1.0;

                return vec2(horizontal_expansion, vertical_expansion);
            }

            fn rotate_2d_from_center(coord: vec2, a: float, size: vec2) -> vec2 {
                let cos_a = cos(-a);
                let sin_a = sin(-a);

                let centered_coord = coord - vec2(0.5, 0.5);

                // Denormalize the coordinates to use original proportions (between height and width)
                let denorm_coord = vec2(centered_coord.x, centered_coord.y * size.y / size.x);
                let demorm_rotated = vec2(denorm_coord.x * cos_a - denorm_coord.y * sin_a, denorm_coord.x * sin_a + denorm_coord.y * cos_a);

                // Restore the coordinates to use the texture coordinates proportions (between 0 and 1 in both axis)
                let rotated = vec2(demorm_rotated.x, demorm_rotated.y * size.x / size.y);

                return rotated + vec2(0.5, 0.5);
            }

            fn get_color(self) -> vec4 {
                let rot_padding = rotation_vertex_expansion(self.rotation, self.rect_size.x, self.rect_size.y) / 2.0;

                // Current position is a traslated one, so let's get the original position
                let current_pos = self.pos.xy - rot_padding;
                let original_pos = rotate_2d_from_center(current_pos, self.rotation, self.rect_size);

                // Scale the current position by the scale factor
                let scaled_pos = original_pos / self.scale;

                // Take pixel color from the original image
                let color = sample2d(self.image, scaled_pos).xyzw;

                let faded_color = color * vec4(1.0, 1.0, 1.0, self.opacity);
                return faded_color;
            }

            fn pixel(self) -> vec4 {
                let rot_expansion = rotation_vertex_expansion(self.rotation, self.rect_size.x, self.rect_size.y);

                // Debug
                // let line_width = 0.01;
                // if self.pos.x < line_width || self.pos.x > (self.scale + rot_expansion.x - line_width) || self.pos.y < line_width || self.pos.y > (self.scale + rot_expansion.y - line_width) {
                //     return #c86;
                // }

                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                let translation_offset = vec2(self.rect_size.x * rot_expansion.x / 2.0, self.rect_size.y * self.scale * rot_expansion.y / 2.0);
                sdf.translate(translation_offset.x, translation_offset.y);

                let center = self.rect_size * 0.5;
                sdf.rotate(self.rotation, center.x, center.y);

                let scaled_size = self.rect_size * self.scale;
                sdf.box(0.0, 0.0, scaled_size.x, scaled_size.y, 1);

                sdf.fill_premul(Pal::premul(self.get_color()));
                return sdf.result
            }

            fn vertex(self) -> vec4 {
                let rot_expansion = rotation_vertex_expansion(self.rotation, self.rect_size.x, self.rect_size.y);
                let adjusted_pos = vec2(
                    self.rect_pos.x - self.rect_size.x * rot_expansion.x / 2.0,
                    self.rect_pos.y - self.rect_size.y * rot_expansion.y / 2.0
                );

                let expanded_size = vec2(self.rect_size.x * (self.scale + rot_expansion.x), self.rect_size.y * (self.scale + rot_expansion.y));
                let clipped: vec2 = clamp(
                    self.geom_pos * expanded_size + adjusted_pos,
                    self.draw_clip.xy,
                    self.draw_clip.zw
                );

                self.pos = (clipped - adjusted_pos) / self.rect_size;
                return self.camera_projection * (self.camera_view * (
                    self.view_transform * vec4(clipped.x, clipped.y, self.draw_depth + self.draw_zbias, 1.)
                ));
            }

            shape: Solid,
            fill: Image
        }
    }
}

/// # GImage Component
/// The `GImage` component handles scalable images with adjustable rotation, scaling, and visibility, supporting event triggers such as hovering and clicking.
/// 
/// ## Animation
/// No animation is available for the `GImage` component.
/// 
/// ## Event
/// The `GImage` component can trigger various events in response to user interactions:
/// - `HoverIn(GImageHoverParam)`: Triggered when the mouse pointer enters the image area.
/// - `HoverOut(GImageHoverParam)`: Triggered when the mouse pointer leaves the image area.
/// - `Clicked(GImageClickedParam)`: Triggered when the image is clicked.
///
/// ## Props
/// |macro   |prop             |description                    |type               |default |
/// |--------|-----------------|-------------------------------|-------------------|--------|
/// |live    |visible           |Visibility of the component    |bool               |true    |
/// |live    |grab_key_focus    |Enable key focus grabbing      |bool               |true    |
/// |live    |opacity           |Opacity level                  |f32                |1.0     |
/// |live    |cursor            |Mouse cursor when hovered      |Option<MouseCursor>|None    |
/// |live    |scale             |Scaling factor                 |f64                |1.0     |
/// |live    |fit               |Image fit type                 |ImageFit           |        |
/// |live    |min_width         |Minimum width of the image     |i64                |16      |
/// |live    |min_height        |Minimum height of the image    |i64                |16      |
/// |live    |rotation          |Rotation angle in radians      |f32                |0.0     |
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
/// |live    |draw_image        |The image drawing object       |DrawGView          |        |
/// |live    |src               |Image source dependency        |LiveDependency     |        |
/// |live    |texture           |Texture object                 |Option<Texture>    |None    |
/// |live    |event_key         |Trigger events when true       |bool               |true    |
#[derive(Live, Widget)]
pub struct GImage {
    #[live(true)]
    pub visible: bool,
    #[live(true)]
    pub grab_key_focus: bool,
    #[live(1.0)]
    pub opacity: f32,
    #[live]
    pub cursor: Option<MouseCursor>,
    #[live(1.0)]
    pub scale: f64,
    #[live]
    pub fit: ImageFit,
    #[live(16)]
    pub min_width: i64,
    #[live(16)]
    pub min_height: i64,
    // rotate -----------------
    #[live(0.0)]
    pub rotation: f32,
    // deref -----------------
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[redraw]
    #[live]
    pub draw_image: DrawGView,
    #[live]
    pub src: LiveDependency,
    #[rust(Texture::new(cx))]
    pub texture: Option<Texture>,
    #[live(true)]
    pub event_key: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
}

impl ImageCacheImpl for GImage {
    fn get_texture(&self, _id: usize) -> &Option<Texture> {
        &self.texture
    }

    fn set_texture(&mut self, texture: Option<Texture>, _id: usize) {
        self.texture = texture;
    }
}

impl LiveHook for GImage {
    fn after_apply(
        &mut self,
        cx: &mut Cx,
        _apply: &mut Apply,
        _index: usize,
        _nodes: &[LiveNode],
    ) {
        if !self.visible {
            return;
        }

        self.render(cx);
    }
}

impl Widget for GImage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, mut walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let rect = cx.peek_walk_turtle(walk);
        let dpi = cx.current_dpi_factor();
        let (width, height) = if let Some(image_texture) = &self.texture {
            self.draw_image.draw_vars.set_texture(0, image_texture);
            let (width, height) = image_texture
                .get_format(cx)
                .vec_width_height()
                .unwrap_or((self.min_width as usize, self.min_height as usize));
            (width as f64 * self.scale, height as f64)
        } else {
            self.draw_image.draw_vars.empty_texture(0);
            (self.min_width as f64 / dpi, self.min_height as f64 / dpi)
        };
        let aspect = width / height;
        match self.fit {
            ImageFit::Size => {
                walk.width = Size::Fixed(width);
                walk.height = Size::Fixed(height);
            }
            ImageFit::Stretch => {}
            ImageFit::Horizontal => {
                walk.height = Size::Fixed(rect.size.x / aspect);
            }
            ImageFit::Vertical => {
                walk.width = Size::Fixed(rect.size.y * aspect);
            }
            ImageFit::Smallest => {
                let walk_height = rect.size.x / aspect;
                if walk_height > rect.size.y {
                    walk.width = Size::Fixed(rect.size.y * aspect);
                } else {
                    walk.height = Size::Fixed(walk_height);
                }
            }
            ImageFit::Biggest => {
                let walk_height = rect.size.x / aspect;
                if walk_height < rect.size.y {
                    walk.width = Size::Fixed(rect.size.y * aspect);
                } else {
                    walk.height = Size::Fixed(walk_height);
                }
            }
        }
        self.draw_walk_rotated_image(cx, walk);
        self.set_scope_path(&scope.path);
        DrawStep::done()
    }
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

        self.handle_widget_event(cx, hit, sweep_area)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if !self.visible {
            return;
        }
        let focus_area = self.area();
        let hit = event.hits(cx, self.area());
        self.handle_widget_event(cx, hit, focus_area)
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl GImage {
    set_scope_path!();
    widget_area! {
        area, draw_image
    }
    active_event! {
        active_hover_in: GImageEvent::HoverIn |e: FingerHoverEvent| => GImageHoverParam{ e },
        active_hover_out: GImageEvent::HoverOut |e: FingerHoverEvent| => GImageHoverParam{ e },
        active_clicked: GImageEvent::Clicked |e: FingerUpEvent| => GImageClickedParam{ e }
    }
    event_option! {
        hover_in: GImageEvent::HoverIn => GImageHoverParam,
        hover_out: GImageEvent::HoverOut => GImageHoverParam,
        clicked: GImageEvent::Clicked => GImageClickedParam
    }
    pub fn redraw(&self, cx: &mut Cx){
        self.draw_image.redraw(cx);
    }
    pub fn render(&mut self, cx: &mut Cx) {
        self.draw_image.apply_over(
            cx,
            live! {
                rotation: (self.rotation),
                scale: (self.scale),
                opacity: (self.opacity),
            },
        );

        // self.draw_image.redraw(cx);
        self.lazy_create_image_cache(cx);
        let source = self.src.clone();
        if source.as_str().len() > 0 {
            let _ = self.load_image_dep_by_path(cx, source.as_str(), 0);
        }
    }
    pub fn draw_walk_rotated_image(&mut self, cx: &mut Cx2d, walk: Walk) -> () {
        if let Some(image_texture) = &self.texture {
            self.draw_image.draw_vars.set_texture(0, image_texture);
        }
        self.draw_image.draw_walk(cx, walk);
    }
    pub fn handle_widget_event(&mut self, cx: &mut Cx, hit: Hit, focus_area: Area) {
        match hit {
            Hit::FingerDown(_) => {
                if self.grab_key_focus {
                    cx.set_key_focus(focus_area);
                }
            }
            Hit::FingerHoverIn(e) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                self.active_hover_in(cx, e);
            }
            Hit::FingerHoverOut(e) => {
                self.active_hover_out(cx, e);
            }
            Hit::FingerUp(e) => {
                if e.is_over {
                    self.active_clicked(cx, e);
                }
            }
            _ => (),
        }
    }
}

impl GImageRef {
    ref_redraw!();
    ref_area!();
    ref_render!();
    ref_event_option! {
        hover_in => GImageHoverParam,
        hover_out => GImageHoverParam,
        clicked => GImageClickedParam
    }
}

impl GImageSet {
    set_event! {
        hover_in => GImageHoverParam,
        hover_out => GImageHoverParam,
        clicked => GImageClickedParam
    }
}
