use proc_macro::TokenStream;
use quote::quote;

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
