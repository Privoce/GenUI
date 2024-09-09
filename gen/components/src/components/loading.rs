use makepad_widgets::*;

use crate::{
    shader::{
        draw_card::DrawCard,
        draw_loading::{DrawGLoading, GLoadingType},
    },
    themes::Themes,
    utils::ThemeColor,
};

live_design! {
    GLoadingBase = {{GLoading}}{

    }
}

#[derive(Live, Widget)]
pub struct GLoading {
    #[live]
    pub theme: Themes,
    #[live]
    pub background_color: Option<Vec4>,
    #[live]
    pub loading_color: Option<Vec4>,
    // deref -------------------
    #[redraw]
    #[live]
    pub draw_loading_wrap: DrawCard,
    #[live]
    pub draw_loading: DrawGLoading,
    #[live]
    pub loading_type: GLoadingType,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    // frame -------------------
    #[live]
    pub time: f32,
    #[rust]
    next_frame: NextFrame,
}

impl Widget for GLoading {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_loading_wrap.begin(cx, walk, self.layout);
        self.draw_loading.draw_walk(cx, walk);
        self.draw_loading_wrap.end(cx);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if let Some(ne) = self.next_frame.is_event(event) {
            // update time to use for animation
            self.time = (ne.time * 0.001).fract() as f32;
            // force updates, so that we can animate in the absence of user-generated events
            self.redraw(cx);
            self.next_frame = cx.new_next_frame();
        }
    }
}

impl LiveHook for GLoading {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        // ------------------ hover color -----------------------------------------------
        let loading_color = self.loading_color.get(self.theme, 600);

        // ------------------ apply to draw_loading_wrap ----------------------------------------
        self.draw_loading.apply_over(
            cx,
            live! {
                background_color: (loading_color),
            },
        );
        self.draw_loading
            .apply_loading_type(self.loading_type.clone());
        self.draw_loading_wrap.redraw(cx);
    }
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        // starts the animation cycle on startup
        self.next_frame = cx.new_next_frame();
    }
}
