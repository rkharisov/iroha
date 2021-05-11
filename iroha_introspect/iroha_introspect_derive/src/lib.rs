use proc_macro::TokenStream;
use std::path::Display;

use quote::quote;
use syn::{Attribute, Data, DataEnum, DataStruct, DeriveInput, Fields, FieldsNamed, ItemEnum, ItemStruct, parse_macro_input, Type, TypePath, FieldsUnnamed};
use syn::__private::TokenStream2;
use std::collections::BTreeSet;
use proc_macro2::Ident;

//todo тесты
#[proc_macro_derive(Introspect)]
pub fn introspect_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_introspect(&input)
}

//todo define attributes
fn impl_introspect(input: &DeriveInput) -> TokenStream {
    println!("{:#?}", &input);
    let name = &input.ident;
    let body = format!("{:#?}", &input);
    let docs = get_item_docs(&input.attrs);
    let field_names = get_field_names(&input.data);
    let field_types = get_field_types(&input.data);
    let field_attrs = get_field_attrs(&input.data).join(" ### ");

    let mut field = quote! {};
    for field_type in field_types {
        field.extend( quote! {
            let bar = <#field_type as iroha_introspect::Introspect>::introspect();
            result.extend(bar);
        });
    };

    let output = quote! {
        impl iroha_introspect::Introspect for #name {
            fn introspect() -> iroha_introspect::Metadata {
            // println!("body: {}", #body);
            // println!("docs: {:?}", #docs);
            // println!("field_names: {:?}", #field_names);
            // println!("field_types: {:?}", #field_types);
            // println!("field_attrs: {:?}", #field_attrs);
            return iroha_introspect::Metadata::BoolMetadata;
            }
        };
    };

    TokenStream::from(output)
}

//todo get docs of the item
fn get_item_docs(attrs: &Vec<Attribute>) -> String {
   let vec: Vec<String> = attrs.iter()
       // .map(|attr|attr.path.segments)
       .map(|attr| format!("{:#?}", &attr.path))
       .collect();
    return vec.join("\\n");
}

fn get_field_names(data: &Data) -> Vec<String> {
    match data {
        Data::Struct(value) => get_struct_field_names(&value),
        Data::Enum(_) => Vec::new(),
        Data::Union(_) => Vec::new()
    }
}

fn get_field_types(data: &Data) -> Vec<Ident> {
    match data {
        Data::Struct(value) => get_struct_fields_types(&value),
        Data::Enum(_) => Vec::new(),
        Data::Union(_) => Vec::new()
    }
}

fn get_struct_field_names(data_struct: &DataStruct) -> Vec<String> {
    println!("111111: {:#?}", data_struct);
  match &data_struct.fields {
      Fields::Named(value) => value.named.iter().filter_map(|f| f.ident.clone()).map(|i| i.to_string()).collect(),
      Fields::Unnamed(_) | Fields::Unit => Vec::new(),
  }
}

fn get_struct_fields_types(data_struct: &DataStruct) -> Vec<Ident> {
    println!("222222: {:#?}", data_struct);
    match &data_struct.fields {
        Fields::Named(value) => get_named_field_types(value),
        Fields::Unnamed(value) => get_unnamed_field_types(value),
        Fields::Unit => Vec::new(),
    }
}

fn get_named_field_types(fields_named: &FieldsNamed) -> Vec<Ident> {
    fields_named.named.iter().map(|f| f.ty.clone()).map(|f| {
        match f {
            Type::Path(type_path) => type_path.path.segments.first().unwrap().ident.clone(),
            _ => panic!()
        }
    }).collect()
}

fn get_unnamed_field_types(fields_unnamed: &FieldsUnnamed) -> Vec<Ident> {
    fields_unnamed.unnamed.iter().map(|f| f.ty.clone()).map(|f| {
        match f {
            Type::Path(type_path) => type_path.path.segments.first().unwrap().ident.clone(),
            _ => panic!()
        }
    }).collect()
}

fn get_field_attrs(data: &Data) -> Vec<String> {
    match data {
        Data::Struct(value) => get_struct_fields_attr(&value),
        Data::Enum(_) => Vec::new(),
        Data::Union(_) => Vec::new()
    }
}

fn get_struct_fields_attr(data_struct: &DataStruct) -> Vec<String> {
    println!("44444444: {:#?}", data_struct);
    match &data_struct.fields {
        Fields::Named(value) => get_named_field_attrs(value),
        Fields::Unnamed(value) => panic!(),
        Fields::Unit => Vec::new(),
    }
}

fn get_named_field_attrs(fields_named: &FieldsNamed) -> Vec<String> {
    fields_named.named.iter().flat_map(|f| {
        f.attrs.iter()
    }).map(|attr| {
            // let attr_tokens: proc_macro::TokenStream = attr.tokens.clone().into();
            // let input = parse_macro_input!(attr_tokens as DeriveInput);
        let x : proc_macro::TokenStream= attr.tokens.clone().into();
        // let input = foo(x);
        let attr_name_2 = attr.path.segments.first().unwrap().ident.to_string();
        return attr_name_2;
    }).collect()
}

// fn foo(input: proc_macro::TokenStream) -> DeriveInput {
//     return parse_macro_input!(input as DeriveInput);
// }
