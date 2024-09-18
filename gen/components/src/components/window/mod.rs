pub mod event;
mod register;
pub mod types;

use std::borrow::BorrowMut;

use desktop_button::DesktopButtonWidgetExt;
use event::*;
use makepad_widgets::*;
use nav_control::NavControl;
use performance_view::PerformanceView;
pub use register::register;
use types::DrawState;

use crate::shader::manual::WindowButtonMode;

use super::{
    card::{GCard, GCardWidgetExt},
    image::GImageWidgetExt,
    label::GLabelWidgetExt,
    tool_btn::{types::GOsType, GToolButtonWidgetExt},
};

live_design! {
    import makepad_draw::shader::std::*;
    GWindowBase = {{GWindow}}{
        border_radius: 0.0,
        background_visible: true,
        background_color: #22272F,
        flow: Down,
        window: {
            inner_size: vec2(1024, 768)
        },
        pass: {
            clear_color: #1F1E25
        },
        cursor: Default,
        mouse_cursor_size: vec2(20, 20),
        draw_cursor: {
            instance border_width: 1.5
            instance color: #ADBAC7,
            instance border_color: #ADBAC7

            fn get_color(self) -> vec4 {
                return self.color
            }

            fn get_border_color(self) -> vec4 {
                return self.border_color
            }

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                sdf.move_to(1.0, 1.0);
                sdf.line_to(self.rect_size.x - 1.0, self.rect_size.y * 0.5)
                sdf.line_to(self.rect_size.x * 0.5, self.rect_size.y - 1.0)
                sdf.close_path();
                sdf.fill_keep(self.get_color())
                if self.border_width > 0.0 {
                    sdf.stroke(self.get_border_color(), self.border_width)
                }
                return sdf.result
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct GWindow {
    #[live]
    pub os_type: Option<GOsType>,
    #[live]
    pub window: WindowHandle,
    #[live]
    pub pass: Pass,
    #[deref]
    pub deref_widget: GCard,
    #[live]
    pub show_title: Option<bool>,
    #[live]
    pub show_icon: Option<bool>,
    #[rust(Texture::new(cx))]
    pub depth_texture: Texture,
    #[rust(DrawList2d::new(cx))]
    pub main_draw_list: DrawList2d,
    #[rust(Overlay::new(cx))]
    pub overlay: Overlay,
    #[live]
    cursor_draw_list: DrawList2d,
    #[live]
    draw_cursor: DrawQuad,
    #[live]
    last_mouse_pos: DVec2,
    #[live]
    mouse_cursor_size: DVec2,
    #[live]
    nav_control: NavControl,
    #[rust]
    pub btns_width: f64,
    #[rust]
    pub pre_btns_width: f64,
    #[rust]
    pub offset: f64,
    #[rust(true)]
    pub redraw_flag: bool,
}

impl Widget for GWindow {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if self.begin(cx).is_not_redrawing() {
            self.draw_state.end();
            return DrawStep::done();
        }
        let _ = self.deref_widget.draw_walk(cx, scope, walk)?;

        self.end(cx);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        self.overlay.handle_event(cx, event);
        self.deref_widget.handle_event(cx, event, scope);

        if let Event::ClearAtlasses = event {
            Cx2d::reset_fonts_atlas(cx);
            Cx2d::reset_icon_atlas(cx);
        }

        if let Event::MouseMove(ev) = event {
            if let OsType::LinuxDirect = cx.os_type() {
                // ok move our mouse cursor
                self.last_mouse_pos = ev.abs;
                self.draw_cursor.update_abs(
                    cx,
                    Rect {
                        pos: ev.abs,
                        size: self.mouse_cursor_size,
                    },
                )
            }
        }
    }
}

impl LiveHook for GWindow {
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        self.window.set_pass(cx, &self.pass);
        self.depth_texture = Texture::new_with_format(
            cx,
            TextureFormat::DepthD32 {
                size: TextureSize::Auto,
            },
        );
        self.pass
            .set_depth_texture(cx, &self.depth_texture, PassClearDepth::ClearWith(1.0));
    }
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        let mut os_type = cx.os_type().clone();
        let _ = self.os_type.as_ref().map(|g_os_type|{
            os_type = g_os_type.clone().into();
        });
        
        match os_type{
            OsType::Windows => {
                // in windows: show icon and title on the left, window buttons are on the right
                self.show_icon(true);
                self.show_title(true);
                self.show_btns(id!(window_bar.win_btns_wrap), true);
                self.show_btns(id!(window_bar.mac_btns_wrap), false);
                self.show_btns(id!(window_bar.linux_btns_wrap), false);
            }
            OsType::Macos => {
                // in macos: do not show icon , show title on the center, window buttons are on the left
                self.show_icon(true);
                self.show_title(false);
                self.show_btns(id!(window_bar.win_btns_wrap), false);
                self.show_btns(id!(window_bar.mac_btns_wrap), true);
                self.show_btns(id!(window_bar.linux_btns_wrap), false);
            }
            OsType::LinuxDirect | OsType::LinuxWindow(_) => {
                // in linux: do not show icon, show  title on the center, window buttons are on the right
                self.show_icon(true);
                self.show_title(false);
                self.show_btns(id!(window_bar.win_btns_wrap), false);
                self.show_btns(id!(window_bar.mac_btns_wrap), false);
                self.show_btns(id!(window_bar.linux_btns_wrap), true);
            }
            _ => {}
        }

        self.show_icon.clone().take().map(|show| {
            self.show_icon(show);
        });

        self.show_title.clone().take().map(|show| {
            self.show_title(show);
        });
    }
}

impl GWindow {
    pub fn show_icon(&mut self, show: bool) {
        self.gimage(id!(window_bar.window_title.icon))
            .borrow_mut()
            .map(|mut img| {
                img.visible = show;
            });
    }
    pub fn show_title(&mut self, show: bool) {
        self.glabel(id!(window_bar.window_title.title))
            .borrow_mut()
            .map(|mut label| {
                label.visible = show;
            });
    }
    pub fn get_btns_width(&mut self, cx: &mut Cx) {
        let mut offset = None;
        match cx.os_type() {
            OsType::Windows => {
                self.gcard(id!(window_bar.win_btns_wrap)).borrow().map(|x| {
                    if let Size::Fixed(s) = x.walk.width {
                        self.btns_width = s;
                    } else {
                        self.btns_width = 138.0;
                    }
                });
            }
            OsType::Macos => {
                self.gcard(id!(window_bar.mac_btns_wrap)).borrow().map(|x| {
                    if let Size::Fixed(s) = x.walk.width {
                        self.btns_width = s;
                    } else {
                        self.btns_width = 72.0;
                    }
                    offset.replace(DVec2 {
                        x: 0.0,
                        y: self.btns_width / 2.0,
                    });
                });
            }
            OsType::LinuxDirect | OsType::LinuxWindow(_) => {
                self.gcard(id!(window_bar.linux_btns_wrap))
                    .borrow()
                    .map(|x| {
                        if let Size::Fixed(s) = x.walk.width {
                            self.btns_width = s;
                        } else {
                            self.btns_width = 72.0;
                        }
                        offset.replace(DVec2 {
                            x: self.btns_width / 2.0,
                            y: 0.0,
                        });
                    });
            }
            _ => {
                self.btns_width = 138.0;
            }
        }

        if self.btns_width != self.pre_btns_width {
            self.redraw_flag = true;
            // if is windows offset = 0.0
            if let Some(offset) = offset {
                let size = self.window.get_inner_size(cx);
                let align_x = if offset.x != 0.0 {
                    offset.x / size.x
                } else {
                    offset.y / size.x
                };
                self.offset = 0.5 + align_x;
            }
        } else {
            self.redraw_flag = false;
        }
    }
    pub fn show_btns(&mut self, id: &[LiveId], show: bool) {
        self.gcard(id).borrow_mut().map(|mut x| {
            x.visible = show;
        });
    }
    pub fn begin(&mut self, cx: &mut Cx2d) -> Redrawing {
        if !cx.will_redraw(&mut self.main_draw_list, Walk::default()) {
            return Redrawing::no();
        }
        cx.begin_pass(&self.pass, None);
        self.main_draw_list.begin_always(cx);
        cx.begin_pass_sized_turtle(Layout::flow_down());
        self.overlay.begin(cx);
        Redrawing::yes()
    }
    pub fn end(&mut self, cx: &mut Cx2d) {
        // only linux needs to draw the cursor here
        if let OsType::LinuxDirect = cx.os_type() {
            self.cursor_draw_list.begin_overlay_last(cx);
            self.draw_cursor.draw_abs(
                cx,
                Rect {
                    pos: self.last_mouse_pos,
                    size: self.mouse_cursor_size,
                },
            );
            self.cursor_draw_list.end(cx);
        }

        self.overlay.end(cx);
        // here template remove this -----------------------------------------------------------
        // if we are running in stdin mode, write a tracking pixel with the pass size
        // if cx.in_makepad_studio(){
        //     let df = cx.current_dpi_factor();
        //     let size = self.pass.size(cx).unwrap() * df;
        //     self.stdin_size.color = encode_size(size.x);
        //     self.stdin_size.draw_abs(cx, Rect{pos:dvec2(0.0,0.0),size:dvec2(1.0/df,1.0/df)});
        //     self.stdin_size.color = encode_size(size.y);
        //     self.stdin_size.draw_abs(cx, Rect{pos:dvec2(1.0/df,0.0),size:dvec2(1.0/df,1.0/df)});
        // }
        // here template remove this -----------------------------------------------------------

        cx.end_pass_sized_turtle();
        self.main_draw_list.end(cx);
        cx.end_pass(&self.pass);
        let _ = self.get_btns_width(cx);
        self.gcard(id!(window_bar.window_title))
            .borrow_mut()
            .map(|mut card| {
                if self.redraw_flag {
                    let size = self.window.get_inner_size(cx);
                    card.walk.width = Size::Fixed(size.x - self.btns_width);
                    card.layout.align = Align {
                        x: self.offset,
                        y: card.layout.align.y,
                    };
                }
            });
    }
}
