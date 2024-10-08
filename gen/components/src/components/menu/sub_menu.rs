// use makepad_widgets::*;

// live_design!{
//     GSubMenuBase = {{GSubMenu}}{}
// }

// #[derive(Live, Widget)]
// pub struct GSubMenu{
//     #[live(true)]
//     pub visible: bool
// }

// impl Widget for GSubMenu {
//     fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
//         if !self.is_visible(){
//             return DrawStep::done();
//         }


//         DrawStep::done()
//     }
//     fn is_visible(&self) -> bool {
//         self.visible
//     }
// }