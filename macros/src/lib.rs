mod html;

#[proc_macro]
pub fn html(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    html::html(proc_macro2::TokenStream::from(input))
        .map(proc_macro::TokenStream::from)
        .unwrap_or_else(proc_macro::TokenStream::from)
}