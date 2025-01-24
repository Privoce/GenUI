use makepad_widgets::*;

live_design!{
    use link::widgets::*;
    use link::gen_components::*;
    use crate::views::main_page::*;
    
    pub FR = <Root>{
        main_window = <GWindow>{
            os_type: Mac,
            window_bar = {
                window_title = {
                    title = {
                        text: "GenUI Builtin Components",
                    }
                    icon = {
                        src: dep("crate://self/resources/google.png"),
                    }
                }
            }
            width: Fill,
            height: Fill,
            window: {inner_size: vec2(920, 800)},
            body = <AppMainPage>{}
            // < GView > { height : 200.0 , width : 300.0 , theme : Dark , background_color : vec4 (1.0 , 1.0 , 1.0 , 1.0) < GLabel > { text : "Hello World" , font_family : dep ("crate://self/resources/OPPOSans-Bold.ttf") , margin : { left : 16.0 , top : 16.0 , right : 16.0 , bottom : 16.0 , } , color : vec4 (0.0 , 0.0 , 0.0 , 1.0) } } < GView > { theme : Error , margin : { left : 4.0 , top : 10.0 , right : 4.0 , bottom : 10.0 , } , visible : true , padding : { left : 2.0 , top : 2.0 , right : 2.0 , bottom : 2.0 , } , cursor : Default } 
            // < GButton > { margin : { left : 16.0 , top : 16.0 , right : 16.0 , bottom : 16.0 } , theme : Error , slot : < GLabel > { text : "Hello World" , } }
        }
     
    }
}