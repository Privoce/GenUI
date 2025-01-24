use gen_components::components::{view::GView, drop_down::PopupMenuGlobal, popup::GPopup};
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    Note = {{Note}}{
        pop: <GPopup>{
            height: 100.0,
            width: 200.0,
            container: <GPopupContainer> {
                height: Fill,
                width: Fill,
                flow: Down,
                spacing: 10.0,
                padding: 10.0,
                <GLabel>{
                    text:"This is a popup",
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct Note {
    #[deref]
    pub super_widget: GView,
    #[live]
    pop: Option<LivePtr>,
}

impl LiveHook for Note {
    fn after_apply_from(&mut self, cx: &mut Cx, _apply: &mut Apply) {
        let global = cx.global::<PopupMenuGlobal>().clone();
        let mut global_map = global.map.borrow_mut();
        global_map.retain(|k, _| cx.live_registry.borrow().generation_valid(*k));
        let popup = self.pop.unwrap();
        global_map.get_or_insert(cx, popup, |cx| GPopup::new_from_ptr(cx, Some(popup)));
    }
}

impl Widget for Note {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, _walk: Walk) -> DrawStep {
        let area = self.draw_view.area();
        cx.add_nav_stop(self.draw_view.area(), NavRole::DropDown, Margin::default());
        let global = cx.global::<PopupMenuGlobal>().clone();
        let mut map = global.map.borrow_mut();
        let popup_menu = map.get_mut(&self.pop.unwrap()).unwrap();
        popup_menu.begin(cx);
        popup_menu.draw_container(cx, scope, None);
        popup_menu.end(cx, scope, area, DVec2::default());
        self.redraw(cx);
        cx.sweep_lock(self.draw_view.area());
        DrawStep::done()
    }
}
