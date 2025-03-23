use gen_analyzer::{value::For, Binds, Else, ElseIf, If};
use gen_utils::error::Error;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, parse_str, ImplItem, Stmt};

use crate::{
    builtin::BuiltinWidget, model::TemplatePtrs, script::Impls, str_to_tk,
    traits::MakepadExtComponent, visitor::sugar_for_fn_ident,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GetSet {
    Get,
    Set,
    UnMatch,
}

impl GetSet {
    #[allow(dead_code)]
    pub fn is_get(&self) -> bool {
        matches!(self, GetSet::Get)
    }
    pub fn is_set(&self) -> bool {
        matches!(self, GetSet::Set)
    }
    pub fn is_unmatch(&self) -> bool {
        matches!(self, GetSet::UnMatch)
    }

    /// 生成组件双向绑定需要的get和set方法
    /// 这个方法一般由PropLzVisitor调用(通过传入的ItemStruct生成所有的get和set方法)
    /// ⚠️ set方法需要注意：需要添加组件对数据的绑定和重绘
    pub fn create(
        field: &str,
        ty: &str,
        binds: &Binds,
        // is_for: bool,
        ptrs: &TemplatePtrs,
        impls: &mut Impls,
    ) -> Result<(), Error> {
        let mut bind_and_redraw = TokenStream::new();
        // dbg!(binds);
        if let Some(binds) = binds.get(field) {
            for widget in binds {
                // 如果是sugar_sign则跳过
                if widget.prop == For::SUGAR_SIGN {
                    continue;
                }

                let (set_prop_fn, value_prefix) = match widget.prop.as_str() {
                    For::SUGAR_SIGN => continue,
                    If::SUGAR_SIGN => ("visible", None),
                    ElseIf::SUGAR_SIGN => ("visible", None),
                    Else::SUGAR_SIGN => ("visible", Some(quote! {!})),
                    _ => (widget.prop.as_str(), None),
                };

                let set_prop_fn =
                    parse_str::<TokenStream>(&format!("set_{}", set_prop_fn)).unwrap();
                let value = quote! {#value_prefix value.clone()};

                let set_prop = if let Some(as_prop) = widget.as_prop.as_ref() {
                    let (widget_name, widget_id) = if let Some(father_ref) =
                        widget.father_ref.as_ref()
                    {
                        let father_name = BuiltinWidget::builtin_name_or_snake(&father_ref.name);
                        (str_to_tk!(&father_name)?, str_to_tk!(&father_ref.id)?)
                    } else {
                        return Err(Error::from("as_prop widget must have father_ref!"));
                    };

                    let prop_widget = BuiltinWidget::builtin_name_or_snake(&widget.name());
                    let as_prop_widget =
                        parse_str::<TokenStream>(&format!("as_{}", prop_widget)).unwrap();
                    let as_prop = parse_str::<TokenStream>(as_prop).unwrap();

                    quote! {
                        if let Some(mut c_ref) = self.#widget_name(id!(#widget_id)).borrow_mut(){
                            let slot_widget = c_ref.#as_prop.#as_prop_widget();
                            slot_widget.#set_prop_fn(cx, #value)?;
                        }
                    }
                } else {
                    let widget_name = parse_str::<TokenStream>(&widget.name()).unwrap();
                    let widget_id = parse_str::<TokenStream>(&widget.id).unwrap();
                    quote! {
                        let widget = self.#widget_name(id!(#widget_id));
                        widget.#set_prop_fn(cx, #value)?;
                    }
                };

                bind_and_redraw.extend(set_prop);
            }
        }

        // let field_tk = parse_str::<TokenStream>(field).unwrap();
        let sugar_for_ident = sugar_for_fn_ident(field);
        if let Some(_) = ptrs.iter().find(|ptr| {
            if let Some(f) = ptr.role.for_field() {
                f == field
            } else {
                false
            }
        }) {
            // sugar_fn_call
            bind_and_redraw.extend(quote! {
                self.#sugar_for_ident(cx, &value);
            });
        }

        let (self_get, self_get_ref) = Self::create_get_fn(field, ty);
        let (self_set, self_set_ref) = Self::create_set_fn(field, ty, bind_and_redraw);

        impls.self_impl.extend(vec![self_get, self_set]);
        impls.self_ref_impl.extend(vec![self_get_ref, self_set_ref]);
        Ok(())
    }

    pub fn getter_setter(ident: &TokenStream) -> Vec<Stmt> {
        vec![
            parse_quote! {
                fn setter<F>(&self, cx: &mut Cx, f: F) -> Result<(), Box<dyn std::error::Error>>
                where
                    F: FnOnce(&mut std::cell::RefMut<'_, #ident>, &mut Cx) -> Result<(), Box<dyn std::error::Error>>,
                {
                    if let Some(mut c_ref) = self.borrow_mut() {
                        return f(&mut c_ref, cx);
                    }
                    Ok(())
                }
            },
            parse_quote! {
                fn getter<T, F>(&self, f: F) -> T
                where
                    F: Fn(&std::cell::Ref<'_, #ident>) -> T,
                    T: Default,
                {
                    if let Some(c_ref) = self.borrow() {
                        f(&c_ref)
                    } else {
                        T::default()
                    }
                }
            },
        ]
    }
    /// 生成双向绑定的get方法
    fn create_get_fn(field: &str, ty: &str) -> (ImplItem, Stmt) {
        let fn_name = parse_str::<TokenStream>(&format!("get_{}", field)).unwrap();
        let field = parse_str::<TokenStream>(field).unwrap();
        let ty = parse_str::<TokenStream>(ty).unwrap();
        (
            parse_quote! {
                fn #fn_name(&self) -> #ty {
                    self.#field.clone()
                }
            },
            parse_quote! {
                pub fn #fn_name(&self) -> #ty{
                    self.getter(|c_ref| c_ref.#field.clone())
                }
            },
        )
    }
    /// 生成双向绑定的set方法
    fn create_set_fn(field: &str, ty: &str, bind_and_redraw: TokenStream) -> (ImplItem, Stmt) {
        let fn_set = parse_str::<TokenStream>(&format!("set_{}", field)).unwrap();
        let field = parse_str::<TokenStream>(field).unwrap();
        let ty = parse_str::<TokenStream>(ty).unwrap();
        (
            parse_quote! {
                fn #fn_set(&mut self, cx: &mut Cx, value: #ty) -> Result<(), Box<dyn std::error::Error>> {
                    #bind_and_redraw
                    self.#field = value.clone();
                    Ok(())
                }
            },
            parse_quote! {
                pub fn #fn_set(&self, cx: &mut Cx, value: #ty) -> Result<(), Box<dyn std::error::Error>> {
                    self.setter(cx, |c_ref, cx| {c_ref.#fn_set(cx, value)})
                }
            },
        )
    }
}

impl From<&str> for GetSet {
    fn from(s: &str) -> Self {
        return if s.starts_with("get_") {
            GetSet::Get
        } else if s.starts_with("set_") {
            GetSet::Set
        } else {
            GetSet::UnMatch
        };
    }
}

pub fn default_impl_get_set(ident: &TokenStream) -> TokenStream {
    quote! {
        setter! {
            #ident{
                set_theme(theme: Themes) {|c, cx| {c.theme = theme; c.render(cx)}},
                set_background_color(color: String) {|c, _cx| {let color = hex_to_vec4(&color)?; c.background_color.replace(color); c.draw_view.background_color = color; Ok(())}},
                set_shadow_color(color: String) {|c, _cx| {let color = hex_to_vec4(&color)?; c.shadow_color.replace(color); c.draw_view.shadow_color = color; Ok(())}},
                set_hover_color(color: String) {|c, _cx| {let color = hex_to_vec4(&color)?; c.hover_color.replace(color); c.draw_view.hover_color = color; Ok(())}},
                set_focus_color(color: String) {|c, _cx| {let color = hex_to_vec4(&color)?; c.focus_color.replace(color); c.draw_view.focus_color = color; Ok(())}},
                set_border_color(color: String) {|c, _cx| {let color = hex_to_vec4(&color)?; c.border_color.replace(color); c.draw_view.border_color = color; Ok(())}},
                set_border_width(width: f32) {|c, _cx| {c.border_width = width; c.draw_view.border_width = width; Ok(())}},
                set_border_radius(radius: f32) {|c, _cx| {c.border_radius = radius; c.draw_view.border_radius = radius; Ok(())}},
                set_shadow_offset(offset: Vec2) {|c, _cx| {c.shadow_offset = offset; c.draw_view.shadow_offset = offset; Ok(())}},
                set_spread_radius(radius: f32) {|c, _cx| {c.spread_radius = radius; c.draw_view.spread_radius = radius; Ok(())}},
                set_blur_radius(radius: f32) {|c, _cx| {c.blur_radius = radius; c.draw_view.blur_radius = radius; Ok(())}},
                set_background_visible(visible: bool) {|c, _cx| {c.background_visible = visible; c.draw_view.background_visible = visible.to_f32(); Ok(())}},
                set_visible(visible: bool) {|c, _cx| {c.visible = visible; Ok(())}},
                set_cursor(cursor: MouseCursor) {|c, _cx| {c.cursor = Some(cursor); Ok(())}},
                set_grab_key_focus(grab: bool) {|c, _cx| {c.grab_key_focus = grab; Ok(())}},
                set_block_signal_event(block: bool) {|c, _cx| {c.block_signal_event = block; Ok(())}},
                set_abs_pos(pos: Option<DVec2>) {|c, _cx| {c.walk.abs_pos = pos; Ok(())}},
                set_margin(margin: Margin) {|c, _cx| {c.walk.margin = margin; Ok(())}},
                set_height(height: Size) {|c, _cx| {c.walk.height = height; Ok(())}},
                set_width(width: Size) {|c, _cx| {c.walk.width = width; Ok(())}},
                set_scroll(scroll: DVec2) {|c, _cx| {c.layout.scroll = scroll; Ok(())}},
                set_clip_x(clip: bool) {|c, _cx| {c.layout.clip_x = clip; Ok(())}},
                set_clip_y(clip: bool) {|c, _cx| {c.layout.clip_y = clip; Ok(())}},
                set_padding(padding: Padding) {|c, _cx| {c.layout.padding = padding; Ok(())}},
                set_align(align: Align) {|c, _cx| {c.layout.align = align; Ok(())}},
                set_flow(flow: Flow) {|c, _cx| {c.layout.flow = flow; Ok(())}},
                set_spacing(spacing: f64) {|c, _cx| {c.layout.spacing = spacing; Ok(())}},
                set_dpi_factor(factor: f64) {|c, _cx| {c.dpi_factor.replace(factor); Ok(())}},
                set_optimize(optimize: ViewOptimize) {|c, _cx| {c.optimize = optimize; Ok(())}},
                set_capture_overload(overload: bool) {|c, _cx| {c.capture_overload = overload; Ok(())}},
                set_event_key(event_key: bool) {|c, _cx| {c.event_key = event_key; Ok(())}}
            }
        }
        getter! {
            #ident{
                get_theme(Themes) {|c| {c.theme}},
                get_background_color(String) {|c| {vec4_to_hex(&c.draw_view.background_color)}},
                get_shadow_color(String) {|c| {vec4_to_hex(&c.draw_view.shadow_color)}},
                get_hover_color(String) {|c| {vec4_to_hex(&c.draw_view.hover_color)}},
                get_focus_color(String) {|c| {vec4_to_hex(&c.draw_view.focus_color)}},
                get_border_color(String) {|c| {vec4_to_hex(&c.draw_view.border_color)}},
                get_border_width(f32) {|c| {c.draw_view.border_width}},
                get_border_radius(f32) {|c| {c.draw_view.border_radius}},
                get_shadow_offset(Vec2) {|c| {c.draw_view.shadow_offset}},
                get_spread_radius(f32) {|c| {c.draw_view.spread_radius}},
                get_blur_radius(f32) {|c| {c.draw_view.blur_radius}},
                get_background_visible(bool) {|c| {c.draw_view.background_visible.to_bool()}},
                get_visible(bool) {|c| {c.visible}},
                get_cursor(MouseCursor) {|c| {c.cursor.unwrap_or_default()}},
                get_grab_key_focus(bool) {|c| {c.grab_key_focus}},
                get_block_signal_event(bool) {|c| {c.block_signal_event}},
                get_abs_pos(Option<DVec2>) {|c| {c.walk.abs_pos.clone()}},
                get_margin(Margin) {|c| {c.walk.margin}},
                get_height(Size) {|c| {c.walk.height}},
                get_width(Size) {|c| {c.walk.width}},
                get_scroll(DVec2) {|c| {c.layout.scroll}},
                get_clip_x(bool) {|c| {c.layout.clip_x}},
                get_clip_y(bool) {|c| {c.layout.clip_y}},
                get_padding(Padding) {|c| {c.layout.padding}},
                get_align(Align) {|c| {c.layout.align}},
                get_flow(Flow) {|c| {c.layout.flow}},
                get_spacing(f64) {|c| {c.layout.spacing}},
                get_dpi_factor(f64) {|c| {c.dpi_factor.unwrap_or_default()}},
                get_optimize(ViewOptimize) {|c| {c.optimize}},
                get_capture_overload(bool) {|c| {c.capture_overload}},
                get_event_key(bool) {|c| {c.event_key}}
            }
        }
        pub fn redraw(&mut self, cx: &mut Cx){
            self.deref_widget.redraw(cx);
        }
    }
}

pub fn default_impl_ref_get_set() -> TokenStream {
    quote! {
        ref_getter_setter!{
            get_theme, set_theme -> Themes,
            get_background_color, set_background_color -> String,
            get_shadow_color, set_shadow_color -> String,
            get_hover_color, set_hover_color -> String,
            get_focus_color, set_focus_color -> String,
            get_border_color, set_border_color -> String,
            get_border_width, set_border_width -> f32,
            get_border_radius, set_border_radius -> f32,
            get_shadow_offset, set_shadow_offset -> Vec2,
            get_spread_radius, set_spread_radius -> f32,
            get_blur_radius, set_blur_radius -> f32,
            get_background_visible, set_background_visible -> bool,
            get_visible, set_visible -> bool,
            get_cursor, set_cursor -> MouseCursor,
            get_grab_key_focus, set_grab_key_focus -> bool,
            get_block_signal_event, set_block_signal_event -> bool,
            get_abs_pos, set_abs_pos -> Option<DVec2>,
            get_margin, set_margin -> Margin,
            get_height, set_height -> Size,
            get_width, set_width -> Size,
            get_scroll, set_scroll -> DVec2,
            get_clip_x, set_clip_x -> bool,
            get_clip_y, set_clip_y -> bool,
            get_padding, set_padding -> Padding,
            get_align, set_align -> Align,
            get_flow, set_flow -> Flow,
            get_spacing, set_spacing -> f64,
            get_dpi_factor, set_dpi_factor -> f64,
            get_optimize, set_optimize -> ViewOptimize,
            get_capture_overload, set_capture_overload -> bool,
            get_event_key, set_event_key -> bool
        }
    }
}
