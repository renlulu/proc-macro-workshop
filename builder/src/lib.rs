use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input, DeriveInput, Ident};


#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    // parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    eprintln!("{:#?}",input);

    let name = input.ident;
    let builder_name = format!("{}Builder",name);
    let builder_ident = Ident::new(&builder_name,name.span());
    let fields = if let syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(syn::FieldsNamed {ref named, .. }), .. }) = input.data {
        named
    } else {
        unimplemented!()
    };

    let optionized_fields = fields.iter().map(|f|
        syn::Field {
            attrs: vec![],
            vis: syn::Visibility::Inherited,
            ident: f.ident.clone(),
            colon_token: f.colon_token.clone(),
            // todo wrap the origin type with optional
            ty: f.ty.clone(),
        }
    );


    let expanded = quote! {
        pub struct #builder_ident{
            #(#optionized_fields,)*
        }
        impl #name {
            pub fn builder() -> #builder_ident{
                #builder_ident{}
            }
        }

        impl #builder_ident {

        }
    };

    proc_macro::TokenStream::from(expanded)
}
