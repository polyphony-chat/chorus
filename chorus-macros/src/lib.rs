use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, FieldsNamed};

#[proc_macro_derive(Updateable)]
pub fn updateable_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;
    // No need for macro hygiene, we're only using this in chorus
    quote! {
        impl Updateable for #name {
            fn id(&self) -> Snowflake {
                self.id
            }
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn observe_option(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn observe_option_vec(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn observe(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn observe_vec(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_derive(
    Composite,
    attributes(observe_option_vec, observe_option, observe, observe_vec)
)]
pub fn composite_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let process_field = |field: &Field| {
        let field_name = &field.ident;
        let attrs = &field.attrs;

        let observe_option = attrs
            .iter()
            .any(|attr| attr.path().is_ident("observe_option"));
        let observe_option_vec = attrs
            .iter()
            .any(|attr| attr.path().is_ident("observe_option_vec"));
        let observe = attrs.iter().any(|attr| attr.path().is_ident("observe"));
        let observe_vec = attrs.iter().any(|attr| attr.path().is_ident("observe_vec"));

        let field_is_arc_rwlock = if let syn::Type::Path(type_path) = &field.ty {
            type_path.path.segments.last().map_or(false, |segment| {
                segment.ident == "Arc" || segment.ident == "RwLock"
            })
        } else {
            false
        };

        if field_is_arc_rwlock {
            match (observe_option, observe_option_vec, observe, observe_vec) {
                (true, _, _, _) => quote! {
                    #field_name: option_observe_fn(self.#field_name)
                },
                (_, true, _, _) => quote! {
                    #field_name: option_vec_observe_fn(self.#field_name)
                },
                (_, _, true, _) => quote! {
                    #field_name: value_observe_fn(self.#field_name)
                },
                (_, _, _, true) => quote! {
                    #field_name: vec_observe_fn(self.#field_name)
                },
                _ => quote! {
                    #field_name: self.#field_name
                },
            }
        } else {
            panic!("Fields must be of type Arc<RwLock<T: Updateable>>");
        }
    };

    match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(FieldsNamed { named, .. }) => {
                let field_exprs = named.iter().map(process_field);

                let ident = &input.ident;

                let expanded = quote! {
                    impl<T: Updateable> Composite<T> for #ident {
                        fn watch_whole(self) -> Self {
                            Self {
                                #(#field_exprs,)*
                            }
                        }
                    }
                };

                TokenStream::from(expanded)
            }
            _ => panic!("Composite derive macro only supports named fields"),
        },
        _ => panic!("Composite derive macro only supports structs"),
    }
}
