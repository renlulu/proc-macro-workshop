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
        // the generated impl
        pub struct CommandBuilder {
             executable: Option<String>,
             args: Option<Vec<String>>,
             env: Option<Vec<String>>,
             current_dir: Option<String>,
        }
        impl #name {
            fn builder() -> CommandBuilder {
                CommandBuilder {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }

    };

    proc_macro::TokenStream::from(expanded)
}
