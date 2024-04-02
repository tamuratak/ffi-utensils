extern crate proc_macro;

#[proc_macro]
pub fn impl_protocol(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let output = input;
    proc_macro::TokenStream::from(output)
}
