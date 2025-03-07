use gen_utils::error::{CompilerError, Error};
use quote::ToTokens;
use syn::{parse_quote, FnArg, ImplItemFn, Signature, Type};

pub struct SpecialEventVisitor;

impl SpecialEventVisitor {
    pub fn visit<S>(item_fn: &mut ImplItemFn, special: S) -> Result<SpecialEvent, Error>
    where
        S: Into<SpecialEvent>,
    {
        let special: SpecialEvent = special.into();
        match special {
            SpecialEvent::HttpResponse => Self::http_response(item_fn),
        }?;

        Ok(special)
    }
    /// 访问http的响应
    /// ```rust
    /// #[http_response]
    /// fn http_response1(response: &HttpResponse) -> (){
    ///     // ...
    /// }
    ///
    /// // http_response2 ...
    /// ```
    /// ```rust
    /// fn  http_response1(response: &HttpResponse) -> (){//...}
    ///
    /// match request_id {
    ///     live_id!(http_response1) => http_response1(response),
    ///     // http_response2 ...
    ///     _ => {}
    /// }
    /// ```
    fn http_response(item_fn: &mut ImplItemFn) -> Result<(), Error> {
        // 检查方法的参数是否有2个参数且第二个类型为 &HttpResponse
        if !is_response_param(&item_fn.sig) {
            return Err(CompilerError::runtime(
                "Makepad Plugin - Script",
                "http_response method must have only one parameter and the type is &HttpResponse",
            )
            .into());
        }
        // 移除宏
        item_fn.attrs.clear();
        // 给方法添加cx参数
        item_fn.sig.inputs.insert(1, parse_quote! {cx: &mut Cx});

        Ok(())
    }
}

fn is_response_param(sig: &Signature) -> bool {
    if sig.inputs.len() != 2 {
        return false;
    }

    if let FnArg::Typed(arg_ty) = &sig.inputs[1] {
        if let Type::Reference(ty_ref) = &*arg_ty.ty {
            return ty_ref.mutability.is_none()
                && ty_ref.elem.to_token_stream().to_string() == "HttpResponse";
        }
    }
    false
}

pub enum SpecialEvent {
    HttpResponse,
}

impl From<&str> for SpecialEvent {
    fn from(value: &str) -> Self {
        match value {
            "http_response" => SpecialEvent::HttpResponse,
            _ => unreachable!(),
        }
    }
}

impl From<String> for SpecialEvent {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}
