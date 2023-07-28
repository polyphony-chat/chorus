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
