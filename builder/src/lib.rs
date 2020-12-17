use proc_macro::TokenStream;
use quote::__private::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Field, Ident, Type};

fn is_type_option(field: &Field) -> Option<&Type> {
    let ty = &field.ty;
    if let syn::Type::Path(ref path) = ty {
        if path.path.segments.len() != 1 || path.path.segments[0].ident != "Option" {
            return None;
        }

        if let syn::PathArguments::AngleBracketed(ref arguments) = path.path.segments[0].arguments {
            if arguments.args.len() != 1 {
                return None;
            }

            let inner_ty = arguments.args.first().unwrap();
            if let syn::GenericArgument::Type(ref t) = inner_ty {
                return Some(t);
            }
        }
    }

    None
}

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    // parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    eprintln!("{:#?}", input);

    let name = input.ident;
    let builder_name = format!("{}Builder", name);
    let builder_ident = Ident::new(&builder_name, name.span());
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = input.data
    {
        named
    } else {
        unimplemented!()
    };

    let optionized_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        if is_type_option(f).is_some() {
            let inner_type = is_type_option(f).unwrap();
            let stream = quote! {
               #name: #inner_type
            };
            return stream;
        }
        quote! {
            #name: std::option::Option<#ty>
        }
    });

    let empty_fields = fields.iter().map(|f| {
        let name = &f.ident;
        if is_type_option(f).is_some() {
            let stream = quote! {
                 #name: String::default()
            };
            return stream;
        }
        quote! {
            #name: None
        }
    });

    let builder_methods = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        if is_type_option(f).is_some() {
            let inner_ty = is_type_option(f).unwrap();
            let stream = quote! {
                fn #name(&mut self, #name: #inner_ty) -> &mut Self {
                    self.#name = #name;
                    self
                }
            };
            return stream;
        }

        quote! {
            fn #name(&mut self, #name: #ty) -> &mut Self {
                self.#name = Some(#name);
                self
            }
        }
    });

    let builder_filed = fields.iter().map(|f| {
        let name = &f.ident;
        if is_type_option(f).is_some() {
            let inner_ty = is_type_option(f).unwrap();
            let stream = quote! {
                // todo Some or None
                // todo to make builder field Option
                #name: Some(self.#name.clone())
            };
            return stream;
        }
        quote! {
            #name: self.#name.clone().ok_or(concat!(stringify!(#name), " is not set"))?
        }
    });

    let expanded = quote! {
        pub struct #builder_ident {
            #(#optionized_fields,)*
        }

        impl #builder_ident {
            #(#builder_methods)*

            pub fn build(&mut self) -> Result<#name, Box<dyn std::error::Error>> {
                Ok(#name {
                    #(#builder_filed,)*
                })
            }
        }

        impl #name {
            pub fn builder() -> #builder_ident {
                #builder_ident{
                    #(#empty_fields,)*
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
