use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input, DeriveInput};


#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    // parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // identifier name
    let name = input.ident;

    let expanded = quote! {

    };

    proc_macro::TokenStream::from(expanded)
}
