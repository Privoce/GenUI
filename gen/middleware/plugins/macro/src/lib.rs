use proc_macro::TokenStream;

/// 插件注册的宏
mod register;
/// 插件注入的宏
mod inject;

/// # 插件注册的宏
/// 这个宏会最终转为lazy_static的注册代码
/// ## Example
/// ```rust
/// plugin!{
///     http: HttpPublisher => http_init()
/// }
/// ```
/// ## After Compilation
/// ```rust
/// lazy_static::lazy_static!{
///     pub static ref HTTP: RwLock<HttpPublisher> = RwLock::new(http_init());
/// }
/// ```
#[proc_macro]
pub fn plugin(input: TokenStream) -> TokenStream {
    register::impl_plugin(input)
}

/// # 插件注入的宏(only read)
/// 这个宏会最终转为插件的引用代码
/// ## Example
/// ```rust
/// let http = inject_ref!(http);
/// ```
/// ## After Compilation
/// ```rust
/// let http = HTTP.read().unwrap();
/// ```
#[proc_macro]
pub fn inject_ref(input: TokenStream) -> TokenStream {
    inject::impl_inject_ref(input)
}

/// # 插件注入的宏(with mut)
/// ## Example
/// ```rust
/// let mut http = inject_mut!(http);
/// ```
/// ## After Compilation
/// ```rust
/// let mut http = HTTP.write().unwrap();
/// ```
#[proc_macro]
pub fn inject_mut(input: TokenStream) -> TokenStream {
    inject::impl_inject_mut(input)
}