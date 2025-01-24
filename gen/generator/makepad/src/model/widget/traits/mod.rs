use crate::{
    model::traits::{LiveHookTrait, WidgetMatchEventTrait, WidgetMatchEventType, WidgetTrait},
    two_way_binding::TWBPollBuilder,
    visitor::{InstanceLzVisitor, InstanceOutput},
};
use gen_utils::error::Error;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemStruct, Local};

#[derive(Debug, Clone)]
pub struct Traits {
    pub widget: WidgetTrait,
    pub live_hook: LiveHookTrait,
    pub widget_match_event: Option<WidgetMatchEventTrait>,
}

impl Default for Traits {
    fn default() -> Self {
        Self {
            widget: WidgetTrait::default(),
            live_hook: LiveHookTrait::default(),
            widget_match_event: None,
        }
    }
}

impl Traits {
    pub fn push_widget_match_event(&mut self, tk: TokenStream, ty: WidgetMatchEventType) {
        if self.widget_match_event.is_none() {
            self.widget_match_event
                .replace(WidgetMatchEventTrait::default());
        }

        if let Some(widget_match_event) = self.widget_match_event.as_mut() {
            widget_match_event.push(tk, ty);
        }

        self.widget.handle_event.widget_match_event = true;
    }

    /// 设置实例初始化到livehook中的after_apply_from_doc
    pub fn set_instance_default(
        &mut self,
        instance: Option<&Local>,
        prop: &ItemStruct,
    ) -> Result<Option<InstanceOutput>, Error> {
        if let Some(instance) = instance {
            let mut lz_visitor = InstanceLzVisitor::new(prop);
            let (output, tk) = lz_visitor.visit(instance)?;
            if let Some(tk) = tk {
                self.live_hook.after_apply_from_doc.replace(quote! {
                    #(#tk)*
                });
            }

            Ok(Some(output))
        } else {
            Ok(None)
        }
    }

    pub fn to_token_stream(
        &self,
        name: &TokenStream,
        twb_poll: Option<&TWBPollBuilder>,
    ) -> TokenStream {
        let mut tokens = TokenStream::new();
        tokens.extend(self.widget.to_token_stream(name.clone(), twb_poll));
        tokens.extend(self.live_hook.to_token_stream(name.clone()));
        if let Some(widget_match_event) = self.widget_match_event.as_ref() {
            tokens.extend(widget_match_event.to_token_stream(name.clone()));
        }
        tokens
    }
}
