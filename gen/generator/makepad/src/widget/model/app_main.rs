use std::collections::HashSet;

use gen_converter::model::script::{GenScriptModel, LifeTime, PropFn, ScriptModel, UseMod};
use gen_utils::common::{token_tree_ident, Source};
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_str;

use crate::{
    widget::model::{widget::Widget, ToLiveDesign},
    ToToken,
};

use super::{
    field::Field, handler::WidgetHandler, live_design::LiveDesign, match_event::MatchEventTrait,
    traits::AppMainTrait,
};

#[derive(Debug, Clone)]
pub struct AppMain {
    /// 当前实例
    pub name: String,
    /// app main的ui入口的name
    pub root_ref: String,
    /// app main的ui入口指向
    pub root_ref_ptr: String,
    /// 处理在实例中的属性
    pub props: Option<Vec<Field>>,
    pub match_event: MatchEventTrait,
    pub app_main: AppMainTrait,
    /// 有哪些组件需要被注册
    /// live design import widget
    pub live_registers: Option<HashSet<String>>,
    pub imports: Option<TokenStream>,
    pub source: Source,
    /// rust use code
    pub uses: Option<TokenStream>,
}

impl AppMain {
    pub fn new(source: &Source) -> Self {
        let name = source.source_name();
        AppMain {
            name,
            root_ref: String::from("root"),
            root_ref_ptr: String::new(),
            props: None,
            match_event: Default::default(),
            app_main: Default::default(),
            live_registers: None,
            source: source.clone(),
            uses: None,
            imports: None,
        }
    }
    pub fn set_imports(&mut self, imports: TokenStream) -> &mut Self {
        self.imports.replace(imports);
        self
    }
    pub fn set_live_registers(&mut self, live_registers: HashSet<String>) -> &mut Self {
        if !live_registers.is_empty() {
            self.live_registers.replace(live_registers);
        }
        self
    }

    pub fn set_script(&mut self, script: Option<ScriptModel>) -> &mut Self {
        if let Some(sc) = script {
            if let ScriptModel::Gen(sc) = sc {
                let GenScriptModel {
                    uses,
                    sub_prop_binds,
                    sub_event_binds,
                    lifetimes,
                    // imports,
                    // prop_ptr,
                    // event_ptr,
                    // other,
                    ..
                } = sc;

                self.set_uses(uses)
                    .set_props(sub_prop_binds.as_ref())
                    .handle_lifetime(sub_prop_binds, lifetimes)
                    .handle_actions(sub_event_binds);
            }
        }
        self
    }
    pub fn handle_actions(&mut self, actions: Option<Vec<PropFn>>) -> &mut Self {
        if let Some(actions) = actions {
            self.match_event.handle_actions(&self.root_ref, actions);
        }
        self
    }
    pub fn handle_lifetime(
        &mut self,
        binds: Option<Vec<PropFn>>,
        lifetimes: Option<LifeTime>,
    ) -> &mut Self {
        self.match_event
            .handle_lifetime(&self.root_ref, binds, lifetimes);
        self
    }

    pub fn set_props(&mut self, props: Option<&Vec<PropFn>>) -> &mut Self {
        if let Some(props) = props {
            for prop in props {
                match self.props.as_mut() {
                    Some(props) => {
                        props.push(Field::from(prop));
                    }
                    None => {
                        let _ = self.props.replace(vec![Field::from(prop)]);
                    }
                }
            }
        }
        self
    }
    pub fn set_uses(&mut self, uses: Option<UseMod>) -> &mut Self {
        if let Some(uses) = uses {
            self.uses = WidgetHandler::uses(&uses);
        }
        self
    }
    pub fn set_root_ref(&mut self, id: String) -> &mut Self {
        self.root_ref = id;
        self
    }
    pub fn set_root_ref_ptr(&mut self, widget: &str) -> &mut Self {
        self.root_ref_ptr = widget.to_string();
        self
    }
    fn build_live_register(&self) -> TokenStream {
        let live_registers = if let Some(live_registers) = self.live_registers.as_ref() {
            // HashSet -> TokenStream
            let tk = live_registers
                .iter()
                .fold(TokenStream::new(), |mut acc, item| {
                    acc.extend(parse_str::<TokenStream>(item));
                    acc
                });
            Some(tk)
        } else {
            None
        };
        quote! {
            fn live_register (cx : & mut Cx) {
                crate::makepad_widgets::live_design(cx);
                #live_registers
            }
        }
    }
}

impl ToLiveDesign for AppMain {
    fn widget_uses(&self) -> Option<TokenStream> {
        None
    }
    fn widget_tree(&self) -> Option<TokenStream> {
        let app = token_tree_ident(&self.name);
        let root = token_tree_ident(&self.root_ref);
        let root_widget = token_tree_ident(&self.root_ref_ptr);

        let tk = quote! {

            #app = {{#app}}{
                #root: <#root_widget>{}
            }
        };
        Some(tk)
    }

    fn widget_logic(&self) -> Option<TokenStream> {
        let root_struct = token_tree_ident(&self.name);
        let ui_field = Field::ui_widget_ref(&self.root_ref).to_token_stream();
        let root_fields = if self.props.is_some() {
            self.props
                .as_ref()
                .unwrap()
                .iter()
                .fold(ui_field, |mut acc, item| {
                    acc.extend(item.to_token_stream());
                    acc
                })
        } else {
            ui_field
        };
        let live_register = self.build_live_register();

        let app_main_trait = self.app_main.to_token_stream(&self.root_ref);

        let tk = quote! {
            #[derive(Live, LiveHook)]
            pub struct #root_struct{
                #root_fields
            }

            impl MatchEvent for #root_struct {

            }

            impl AppMain for App {
                #app_main_trait
            }

            impl LiveRegister for #root_struct {
                #live_register
            }

            app_main!(#root_struct);
        };

        Some(tk)
    }

    fn to_live_design(&self) -> LiveDesign {
        self.into()
    }

    fn widget_imports(&self) -> Option<TokenStream> {
        self.imports.clone()
    }
}

impl From<gen_converter::model::Model> for AppMain {
    fn from(value: gen_converter::model::Model) -> Self {
        // clone a new script, other make to widget tree
        let script = value.script.clone();
        let mut app = AppMain::new(value.get_special());
        let widget = Widget::from(value);
        let root_id = widget.id.as_ref().expect("root id is required").to_string();
        app.set_root_ref(root_id).set_script(script);
        // let app_tk = app.to_live_design().to_token_stream();
        app
    }
}
