// use gen_converter::model::script::{LifeTime, PropFn};

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

// use crate::utils::apply_over_and_redraw;

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct MatchEventTrait {
    /// the bind props which can be changed in the app main
    /// this will be set on handle startup
    // global: Option<Vec<PropFn>>,
    startup: Option<TokenStream>,
    shutdown: Option<TokenStream>,
    foreground: Option<TokenStream>,
    background: Option<TokenStream>,
    pause: Option<TokenStream>,
    resume: Option<TokenStream>,
    app_got_focus: Option<TokenStream>,
    app_lost_focus: Option<TokenStream>,
    next_frame: Option<TokenStream>,
    action: Option<TokenStream>,
    actions: Option<TokenStream>,
    signal: Option<TokenStream>,
    audio_devices: Option<TokenStream>,
    midi_ports: Option<TokenStream>,
    video_inputs: Option<TokenStream>,
    http_response: Option<TokenStream>,
    http_request_error: Option<TokenStream>,
    http_progress: Option<TokenStream>,
    network_responses: Option<TokenStream>,
    draw: Option<TokenStream>,
    timer: Option<TokenStream>,
    draw_2d: Option<TokenStream>,
    key_down: Option<TokenStream>,
    key_up: Option<TokenStream>,
    back_pressed: Option<TokenStream>,
    match_event: Option<TokenStream>,
    match_event_with_draw_2d: Option<TokenStream>,
}

impl ToTokens for MatchEventTrait {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let startup = self.startup.as_ref();
        let shutdown = self.shutdown.as_ref();
        let foreground = self.foreground.as_ref();
        let background = self.background.as_ref();
        let pause = self.pause.as_ref();
        let resume = self.resume.as_ref();
        let app_got_focus = self.app_got_focus.as_ref();
        let app_lost_focus = self.app_lost_focus.as_ref();
        let next_frame = self.next_frame.as_ref();
        let action = self.action.as_ref();
        let actions = self.actions.as_ref();
        let signal = self.signal.as_ref();
        let audio_devices = self.audio_devices.as_ref();
        let midi_ports = self.midi_ports.as_ref();
        let video_inputs = self.video_inputs.as_ref();
        let http_response = self.http_response.as_ref();
        let http_request_error = self.http_request_error.as_ref();
        let http_progress = self.http_progress.as_ref();
        let network_responses = self.network_responses.as_ref();
        let draw = self.draw.as_ref();
        let timer = self.timer.as_ref();
        let draw_2d = self.draw_2d.as_ref();
        let key_down = self.key_down.as_ref();
        let key_up = self.key_up.as_ref();
        let back_pressed = self.back_pressed.as_ref();
        let match_event = self.match_event.as_ref();
        let match_event_with_draw_2d = self.match_event_with_draw_2d.as_ref();

        tokens.extend(quote!{
            impl MatchEvent for App {
                #startup
                #shutdown
                #foreground
                #background
                #pause
                #resume
                #app_got_focus
                #app_lost_focus
                #next_frame
                #action
                #actions
                #signal
                #audio_devices
                #midi_ports
                #video_inputs
                #http_response
                #http_request_error
                #http_progress
                #network_responses
                #draw
                #timer
                #draw_2d
                #key_down
                #key_up
                #back_pressed
                #match_event
                #match_event_with_draw_2d
            }            
        });
    }
}


// impl MatchEventTrait {
//     pub fn handle_actions(&mut self, root_id: &str, actions: Vec<PropFn>) -> &mut Self {
//         let mut tk = TokenStream::new();
//         for item in actions {
//             let PropFn {
//                 widget, id, code, ..
//             } = item;

//             tk.extend(apply_over_and_redraw(
//                 Some(root_id.to_string()),
//                 &widget,
//                 &id,
//                 token_stream_to_tree(code.to_token_stream()),
//             ));
//         }

//         self.actions.replace(quote! {
//             fn handle_actions(&mut self, cx: &mut Cx, actions:&Actions){

//             }
//         });
//         self
//     }

//     pub fn handle_lifetime(
//         &mut self,
//         root_id: &str,
//         binds: Option<Vec<PropFn>>,
//         lifetimes: Option<LifeTime>,
//     ) -> &mut Self {
//         if let Some(lifetimes) = lifetimes {
//             let LifeTime { startup, shutdown } = lifetimes;

//             self.handle_startup(root_id, binds, startup)
//                 .handle_shutdown(shutdown);
//         }
//         self
//     }
//     pub fn handle_startup(
//         &mut self,
//         root_id: &str,
//         binds: Option<Vec<PropFn>>,
//         startup: Option<StmtMacro>,
//     ) -> &mut Self {
//         if let Some(startup) = startup {
//             let mut tk = startup.mac.tokens;
//             if let Some(bind_tks) = &binds {
//                 for item in bind_tks {
//                     let PropFn {
//                         widget, id, code, ..
//                     } = item;

//                     tk.extend(apply_over_and_redraw(
//                         Some(root_id.to_string()),
//                         widget,
//                         id,
//                         token_stream_to_tree(code.to_token_stream()),
//                     ));
//                 }
//             }

//             self.startup.replace(quote! {
//                 fn handle_startup(&mut self, cx: &mut Cx) {
//                     #tk
//                 }
//             });
//         }
//         self.global = binds;
//         self
//     }
//     pub fn handle_shutdown(&mut self, shutdown: Option<StmtMacro>) -> &mut Self {
//         if let Some(shutdown) = shutdown {
//             self.shutdown.replace(quote! {
//                 fn handle_shutdown(&mut self, _cx: &mut Cx){
//                     #shutdown.mac.tokens
//                 }
//             });
//         }
//         self
//     }
// }
