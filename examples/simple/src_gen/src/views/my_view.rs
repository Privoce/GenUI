use makepad_widgets :: * ; use crate :: auto :: Label_01J5XK34JCXF6NZJE8FXVHXZB8 ::*; use crate :: auto :: IfWidget_01J5XK34JD1SSP6D2VT5J6V005 ::*; live_design ! { import makepad_widgets :: base :: * ; import makepad_widgets :: theme_desktop_dark :: * ; import makepad_draw :: shader :: std :: * ; import crate :: auto :: Label_01J5XK34JCXF6NZJE8FXVHXZB8 ::*; import crate :: auto :: IfWidget_01J5XK34JD1SSP6D2VT5J6V005 ::*; MyView = {{ MyView }}{ < Label01J5XK34JCXF6NZJE8FXVHXZB8 >{ } if_widget1 = < IfWidget01J5XK34JD1SSP6D2VT5J6V005 >{ } toggle_btn = < Button >{ text : "click here to change if signal" , } } } # [derive (Live , Widget)] pub struct MyView { # [live] pub flag1 : bool , # [deref] pub deref_widget : View } impl Widget for MyView { fn draw_walk (& mut self , cx : & mut Cx2d , scope : & mut Scope , walk : Walk) -> DrawStep { self . deref_widget . draw_walk (cx , scope , walk) } fn handle_event (& mut self , cx : & mut Cx , event : & Event , scope : & mut Scope) { let uid = self . widget_uid () ; if let Event :: Actions (actions) = event { if self . button (id ! (toggle_btn)) . clicked (actions) { let mut toggle = | | { self . flag1 = ! self . flag1 ; } ; let _ = toggle () ; self . if_widget01_j5_xk34_jd1_ssp6_d2_vt5_j6_v005 (id ! (if_widget1)) . set_if_signal (self . flag1) ; } } self . redraw (cx) ; self . deref_widget . handle_event (cx , event , scope) ; } } impl LiveHook for MyView { fn before_apply (& mut self , cx : & mut Cx , apply : & mut Apply , index : usize , nodes : & [LiveNode]) { self . flag1 = false ; } fn after_apply (& mut self , cx : & mut Cx , apply : & mut Apply , index : usize , nodes : & [LiveNode]) { self . if_widget01_j5_xk34_jd1_ssp6_d2_vt5_j6_v005 (id ! (if_widget1)) . set_if_signal (self . flag1) ; } }