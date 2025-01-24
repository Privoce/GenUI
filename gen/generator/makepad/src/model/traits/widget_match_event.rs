use super::push_handle;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug, Default, Clone)]
pub struct WidgetMatchEventTrait {
    handle_next_frame: Option<TokenStream>,
    handle_actions: Option<TokenStream>,
    handle_signal: Option<TokenStream>,
    handle_audio_devices: Option<TokenStream>,
    handle_midi_ports: Option<TokenStream>,
    handle_video_inputs: Option<TokenStream>,
    handle_http_response: Option<TokenStream>,
    handle_http_request_error: Option<TokenStream>,
    handle_http_progress: Option<TokenStream>,
    handle_http_stream: Option<TokenStream>,
    handle_http_stream_complete: Option<TokenStream>,
    // handle_network_responses: Option<TokenStream>,
    // widget_match_event: Option<TokenStream>,
}

pub enum WidgetMatchEventType {
    NextFrame,
    Actions,
    Signal,
    AudioDevices,
    MidiPorts,
    VideoInputs,
    HttpResponse,
    HttpRequestError,
    HttpProgress,
    HttpStream,
    HttpStreamComplete,
}

impl WidgetMatchEventTrait {
    pub fn push(&mut self, tk: TokenStream, ty: WidgetMatchEventType) -> () {
        match ty {
            WidgetMatchEventType::NextFrame => push_handle(&mut self.handle_next_frame, tk),
            WidgetMatchEventType::Actions => push_handle(&mut self.handle_actions, tk),
            WidgetMatchEventType::Signal => push_handle(&mut self.handle_signal, tk),
            WidgetMatchEventType::AudioDevices => push_handle(&mut self.handle_audio_devices, tk),
            WidgetMatchEventType::MidiPorts => push_handle(&mut self.handle_midi_ports, tk),
            WidgetMatchEventType::VideoInputs => push_handle(&mut self.handle_video_inputs, tk),
            WidgetMatchEventType::HttpRequestError => {
                push_handle(&mut self.handle_http_request_error, tk)
            }
            WidgetMatchEventType::HttpResponse => push_handle(&mut self.handle_http_response, tk),
            WidgetMatchEventType::HttpProgress => push_handle(&mut self.handle_http_progress, tk),
            WidgetMatchEventType::HttpStream => push_handle(&mut self.handle_http_stream, tk),
            WidgetMatchEventType::HttpStreamComplete => {
                push_handle(&mut self.handle_http_stream_complete, tk)
            }
        }
    }

    pub fn handle_http_response_tk(&self) -> Option<TokenStream> {
        if let Some(handle_http_response) = self.handle_http_response.as_ref() {
            Some(quote! {
                fn handle_http_response(&mut self, cx:&mut Cx, request_id: LiveId, response: &HttpResponse, scope: &mut Scope) {
                    #handle_http_response
                }
            })
        } else {
            None
        }
    }

    pub fn to_token_stream<TK>(&self, target: TK) -> TokenStream
    where
        TK: Into<TokenStream>,
    {
        let target = target.into();
        let handle_next_frame = &self.handle_next_frame;
        let handle_actions = &self.handle_actions;
        let handle_signal = &self.handle_signal;
        let handle_audio_devices = &self.handle_audio_devices;
        let handle_midi_ports = &self.handle_midi_ports;
        let handle_video_inputs = &self.handle_video_inputs;
        let handle_http_response = self.handle_http_response_tk();
        let handle_http_request_error = &self.handle_http_request_error;
        let handle_http_progress = &self.handle_http_progress;
        let handle_http_stream = &self.handle_http_stream;
        let handle_http_stream_complete = &self.handle_http_stream_complete;
        // let handle_network_responses = &self.handle_network_responses;
        // let widget_match_event = &self.widget_match_event;

        quote! {
            impl WidgetMatchEvent for #target{
                #handle_next_frame
                #handle_actions
                #handle_signal
                #handle_audio_devices
                #handle_midi_ports
                #handle_video_inputs
                #handle_http_response
                #handle_http_request_error
                #handle_http_progress
                #handle_http_stream
                #handle_http_stream_complete
            }
        }
    }
}
