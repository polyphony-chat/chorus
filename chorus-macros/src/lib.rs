// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, FieldsNamed};

#[proc_macro_derive(WebSocketEvent)]
pub fn websocket_event_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;

    quote! {
        impl WebSocketEvent for #name {}
    }
    .into()
}

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

#[proc_macro_derive(JsonField)]
pub fn jsonfield_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;
    // No need for macro hygiene, we're only using this in chorus
    quote! {
        impl JsonField for #name {
            fn get_json(&self) -> String {
                self.json.clone()
            }
            fn set_json(&mut self, json: String) {
                self.json = json;
            }
        }
    }
    .into()
}

#[proc_macro_derive(SourceUrlField)]
pub fn source_url_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;
    // No need for macro hygiene, we're only using this in chorus
    quote! {
        impl SourceUrlField for #name {
            fn get_source_url(&self) -> String {
                self.source_url.clone()
            }
            fn set_source_url(&mut self, url: String) {
                self.source_url = url;
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

        match (observe_option, observe_option_vec, observe, observe_vec) {
            (true, _, _, _) => quote! {
                #field_name: Self::option_observe_fn(self.#field_name, gateway).await
            },
            (_, true, _, _) => quote! {
                #field_name: Self::option_vec_observe_fn(self.#field_name, gateway).await
            },
            (_, _, true, _) => quote! {
                #field_name: Self::value_observe_fn(self.#field_name, gateway).await
            },
            (_, _, _, true) => quote! {
                #field_name: Self::vec_observe_fn(self.#field_name, gateway).await
            },
            _ => quote! {
                #field_name: self.#field_name
            },
        }
    };

    match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(FieldsNamed { named, .. }) => {
                let field_exprs = named.iter().map(process_field);

                let ident = &input.ident;
                let expanded = quote! {
                    #[async_trait::async_trait(?Send)]
                    impl<T: Updateable + Clone + Debug> Composite<T> for #ident {
                        async fn watch_whole(self, gateway: &GatewayHandle) -> Self {
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


#[proc_macro_derive(SqlxBitFlags)]
pub fn sqlx_bitflag_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;

    quote!{
        #[cfg(feature = "sqlx")]
        impl sqlx::Type<sqlx::MySql> for #name {
            fn type_info() -> sqlx::mysql::MySqlTypeInfo {
                u64::type_info()
            }
        }

        #[cfg(feature = "sqlx")]
        impl<'q> sqlx::Encode<'q, sqlx::MySql> for #name {
            fn encode_by_ref(&self, buf: &mut <sqlx::MySql as sqlx::database::HasArguments<'q>>::ArgumentBuffer) -> sqlx::encode::IsNull {
                u64::encode_by_ref(&self.bits(), buf)
            }
        }

        #[cfg(feature = "sqlx")]
        impl<'q> sqlx::Decode<'q, sqlx::MySql> for #name {
            fn decode(value: <sqlx::MySql as sqlx::database::HasValueRef<'q>>::ValueRef) -> Result<Self, sqlx::error::BoxDynError> {
                u64::decode(value).map(|d| #name::from_bits(d).unwrap())
            }
        }
    }
    .into()
}

#[proc_macro_derive(SerdeBitFlags)]
pub fn serde_bitflag_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;

    quote! {
        impl std::str::FromStr for #name {
            type Err = std::num::ParseIntError;

            fn from_str(s: &str) -> Result<#name, Self::Err> {
                s.parse::<u64>().map(#name::from_bits).map(|f| f.unwrap_or(#name::empty()))
            }
        }

        impl serde::Serialize for #name {
            fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(&self.bits().to_string())
            }
        }

        impl<'de> serde::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<#name, D::Error> where D: serde::de::Deserializer<'de> + Sized {
                // let s = String::deserialize(deserializer)?.parse::<u64>().map_err(serde::de::Error::custom)?;
                let s = crate::types::serde::string_or_u64(deserializer)?;

                Ok(Self::from_bits(s).unwrap())
            }
        }
    }
    .into()
}