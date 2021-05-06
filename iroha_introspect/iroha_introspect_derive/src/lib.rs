use proc_macro::TokenStream;
use std::path::Display;

use quote::quote;
use syn::{ItemStruct, parse_macro_input};

#[proc_macro_derive(Introspect)]
pub fn introspect_derive(input: TokenStream) -> TokenStream {
    let item_struct1 = parse_macro_input!(input as ItemStruct);
    impl_introspect(&item_struct1)
}

fn impl_introspect(input: &ItemStruct) -> TokenStream {
    println!("{:?}", &input);
    let name = &input.ident;

    let body = input
        .fields
        .iter()
        // .map(|field| match field.parse_named() {
        //     Ok(field) => format!("{:?}", field),
        //     Err(_) => println!("Field can not be parsed successfully"),
        // })
        .map(|field| format!("{:?}", field))
        .into_iter();
        // .join(", ");

    let body = format!("{:?}", &input.fields);
    let output = quote! {
        impl iroha_introspect::Introspect for #name {
            fn introspect(){
                println!("{}", #body);
            }
        };
    };

    TokenStream::from(output)
}
