use makepad_widgets :: * ; use gen_components :: * ; live_design ! { use link :: widgets :: * ; use link :: gen_components :: * ; use link :: shaders :: * ; pub Easy = { { Easy } } { spacing : 8.0 , < GTag > { text : "hello" , } < GTag > { text : "hello" , color : vec4 (1.0 , 0.0 , 0.0 , 1.0) , } < GTag > { closeable : true , theme : Info , font_size : 14.0 , text : "hello" , border_radius : 6.0 , } < GToggle > { } < Toggle > { select : true , theme : Error , } } } # [derive (Live , Widget)] pub struct Easy { # [deref] pub deref_widget : GView , } impl Easy { } # [allow (unused)] impl EasyRef { } impl Widget for Easy { # [allow (unused_variables)] fn draw_walk (& mut self , cx : & mut Cx2d , scope : & mut Scope , walk : Walk) -> DrawStep { self . deref_widget . draw_walk (cx , scope , walk) } # [allow (unused_variables)] fn handle_event (& mut self , cx : & mut Cx , event : & Event , scope : & mut Scope) { let actions = cx . capture_actions (| cx | self . deref_widget . handle_event (cx , event , scope)) ; } # [allow (unused_variables)] fn is_visible (& self) -> bool { self . visible } } impl LiveHook for Easy { # [allow (unused_variables)] fn after_apply (& mut self , cx : & mut Cx , apply : & mut Apply , index : usize , nodes : & [LiveNode]) { self . deref_widget . after_apply (cx , apply , index , nodes) ; } }