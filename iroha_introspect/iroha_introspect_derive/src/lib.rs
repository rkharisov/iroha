use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemStruct};
use quote::quote;

#[proc_macro_derive(Introspect)]
pub fn introspect_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    impl_introspect(input)
}

fn impl_introspect(input: ItemStruct) -> TokenStream {
    let name = &input.ident;
    let output = quote! {
        impl iroha_introspect::Introspect for #name {
            pub fn introspect(){
                input
                .fields
                .iter()
                .for_each(|field| match field.parse_named() {
                    Ok(field) => println!("{:?}", field),
                    Err(_) => println!("Field can not be parsed successfully"),
                });
            }
        }
    };

    TokenStream::from(output)
}
