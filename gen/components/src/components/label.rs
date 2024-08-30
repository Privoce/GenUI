use makepad_widgets::*;
use shader::draw_text::TextWrap;
use crate::utils::get_font_family;

live_design! {
    GLabelBase = {{GLabel}}{}
}

#[derive(Live, Widget)]
pub struct GLabel {
    #[live]
    pub color: Vec4,
    #[live(9.0)]
    pub font_size: f64,
    #[live(1.0)]
    pub brightness: f32,
    #[live(0.5)]
    pub curve: f32,
    #[live(1.5)]
    pub line_spacing: f64,
    #[live(0.0)]
    pub top_drop: f64,
    #[live(0.0)]
    pub height_factor: f64,
    #[live(TextWrap::Word)]
    pub wrap: TextWrap,
    #[live]
    pub font_family: LiveDependency,
    #[live(true)]
    pub visible: bool,
    // deref ---------------------
    #[redraw]
    #[live]
    draw_text: DrawText,
    #[walk]
    walk: Walk,
    #[live]
    align: Align,
    #[live]
    padding: Padding,
    #[live]
    text: RcStringMut,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum LabelEvent {
    Hover,
    HoverOut,
    Clicked,
    None,
}

impl Widget for GLabel {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let font = get_font_family(&self.font_family, cx);

        self.draw_text.text_style.font = font;
        
        let mut padding  =self.padding;
        padding.top += 2.0;
        self.draw_text.draw_walk(
            cx,
            walk.with_add_padding(padding),
            self.align,
            self.text.as_ref(),
        );

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let hit = event.hits(cx, self.area());
        self.handle_widget_event(cx, event, scope, hit)
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
        self.handle_widget_event(cx, event, scope, hit)
    }
    /// copy label text
    fn text(&self) -> String {
        self.text.as_ref().to_string()
    }
    fn set_text(&mut self, v: &str) {
        self.text.as_mut_empty().push_str(v);
    }
    fn set_text_and_redraw(&mut self, cx: &mut Cx, v: &str) {
        self.text.as_mut_empty().push_str(v);
        self.redraw(cx)
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GLabel {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        self.draw_text.apply_over(
            cx,
            live! {
                color: (self.color),
                text_style: {
                    brightness: (self.brightness),
                    curve: (self.curve),
                    line_spacing: (self.line_spacing),
                    top_drop: (self.top_drop),
                    font_size: (self.font_size),
                    height_factor: (self.height_factor),
                }
            },
        );
        self.draw_text.wrap = self.wrap.clone();
        self.draw_text.redraw(cx);

    }
}

impl GLabel {
    pub fn area(&self) -> Area {
        self.draw_text.area()
    }
    pub fn handle_widget_event(
        &mut self,
        cx: &mut Cx,
        _event: &Event,
        scope: &mut Scope,
        hit: Hit,
        
    ){
        let uid = self.widget_uid();

        match hit {
            Hit::FingerHoverIn(_) => {
                cx.widget_action(uid, &scope.path, LabelEvent::Hover);
            }
            Hit::FingerHoverOut(_) => {
                cx.widget_action(uid, &scope.path, LabelEvent::HoverOut);
            }
            Hit::FingerUp(f_up) => {
                if f_up.is_over {
                    cx.widget_action(uid, &scope.path, LabelEvent::Clicked);
                }
            }
            _ => (),
        }
    }
}