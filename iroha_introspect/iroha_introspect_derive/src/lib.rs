#![recursion_limit = "10000"]

use proc_macro::TokenStream;

use proc_macro2::Ident;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Fields, parse_macro_input, Field, Attribute, NestedMeta, Meta, Variant, Lit, GenericParam, Generics};
use syn::parse::Parse;
use syn::spanned::Spanned;

#[proc_macro_derive(Introspect)]
pub fn introspect_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_introspect(input)
}

fn impl_introspect(input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    let metadata = metadata(&input);
    let (params, ident_params, where_clause) = generics(&input.generics);

   let foo = name.to_string();
    let expanded = quote! {
        impl #params iroha_introspect::Introspect for #name #ident_params #where_clause {
            fn introspect() -> iroha_introspect::Metadata {
                println!("{}", #foo);
               #metadata
            }
        }
    };

    TokenStream::from(expanded)
}

fn generics(generics: &Generics) -> (TokenStream2, TokenStream2, TokenStream2){
    let Generics {
        params,
        where_clause,
        ..
    } = generics;
    let ident_params = params.iter().map(generic_ident).collect::<Vec<_>>();
    if params.is_empty() {
        (quote! {}, quote! {}, quote! {})
    } else {
        (quote! { <#params> }, quote! { <#(#ident_params,)*> }, quote! { #where_clause })
    }
}

fn generic_ident(param: &GenericParam) -> TokenStream2 {
    match param {
        GenericParam::Type(ty) => {
            let ident = &ty.ident;
            quote! { #ident }
        }
        GenericParam::Const(constgeneric) => {
            let ident = &constgeneric.ident;
            quote! { #ident }
        }
        GenericParam::Lifetime(lifetime) => {
            let lifetime = &lifetime.lifetime;
            quote! { #lifetime }
        }
    }
}

fn metadata(input: &DeriveInput) -> TokenStream2 {
    let name = &input.ident;
    return match &input.data {
        Data::Struct(data_struct) => metadata_for_structs(name, data_struct),
        Data::Enum(data_enum) => metadata_for_enums(name, data_enum),
        Data::Union(_) => unimplemented!()
    };
}

fn metadata_for_structs(name: &Ident, data_struct: &DataStruct) -> TokenStream2 {
    let name = name.to_string();
    let declarations = get_fields_declaration(&data_struct.fields);
    quote! {
                iroha_introspect::Metadata::Struct(
                    iroha_introspect::StructMeta {
                        ident: #name.into(),
                        declarations: #declarations,
                    }
                )
    }
}

fn metadata_for_enums(name: &Ident, data_enum: &DataEnum) -> TokenStream2 {
    let variants = data_enum.variants.iter()
        .enumerate()
        .map(|(discriminant, v)| {
            let variant_name = &v.ident.clone().to_string();
            let discriminant = variant_index(v, discriminant);
            let declarations = get_fields_declaration(&v.fields);
            quote! {
                iroha_introspect::EnumVariant { name: #variant_name.into(), discriminant: #discriminant, declarations: #declarations},
            }
        });
    let name = name.to_string();
    quote! {
                iroha_introspect::Metadata::Enum(
                    iroha_introspect::EnumMeta {
                        ident: #name.into(),
                        variants: vec![#(#variants)*],
                    }
                )
            }
}

fn get_fields_declaration(fields: &Fields) -> TokenStream2 {
    match fields {
        Fields::Named(named_fields) => declarations(named_fields.named.iter()),
        Fields::Unnamed(unnamed_fields) => declarations(unnamed_fields.unnamed.iter()),
        Fields::Unit => declarations(std::iter::empty())
    }
}

fn declarations<'a> (fields: impl Iterator<Item = &'a Field>) -> TokenStream2 {
    let declarations = fields
        .map(|f| {
            let name_defined = &f.ident.is_some();
            let prop_name = f.ident.clone().map(|i| i.to_string()).or_else(|| Some("".into()));
            let prop_type = &f.ty;
            let definition = if is_compact(&f) {
                quote! {
                    iroha_introspect::Metadata::Int(
                        iroha_introspect::IntMeta { mode: iroha_introspect::Mode::Compact }
                    )
                }
            } else {
                quote! {<#prop_type as iroha_introspect::Introspect>::introspect()}
            };
            //skip if #[codec(skip)] used
            if should_skip(&f.attrs) {
                quote!{}
            } else {
                quote! {
                        iroha_introspect::Declaration {
                            name: if #name_defined { Some(#prop_name.into()) } else { None },
                            definition: Box::new(#definition),
                        },
                    }
            }
        });
    quote! {vec![#(#declarations)*]}
}

/// Look for a `#[codec(compact)]` outer attribute on the given `Field`.
fn is_compact(field: &Field) -> bool {
    find_meta_item(field.attrs.iter(), |meta| {
        if let NestedMeta::Meta(Meta::Path(ref path)) = meta {
            if path.is_ident("compact") {
                return Some(());
            }
        }

        None
    }).is_some()
}

/// Look for a `#[codec(skip)]` in the given attributes.
fn should_skip(attrs: &[Attribute]) -> bool {
    find_meta_item(attrs.iter(), |meta| {
        if let NestedMeta::Meta(Meta::Path(ref path)) = meta {
            if path.is_ident("skip") {
                return Some(path.span());
            }
        }

        None
    }).is_some()
}


/// Look for a `#[scale(index = $int)]` attribute on a variant. If no attribute
/// is found, fall back to the discriminant or just the variant index.
fn variant_index(v: &Variant, i: usize) -> TokenStream2 {
    // first look for an attribute
    let index = find_meta_item(v.attrs.iter(), |meta| {
        if let NestedMeta::Meta(Meta::NameValue(ref nv)) = meta {
            if nv.path.is_ident("index") {
                if let Lit::Int(ref v) = nv.lit {
                    let byte = v.base10_parse::<u8>()
                        .expect("Internal error, index attribute must have been checked");
                    return Some(byte)
                }
            }
        }

        None
    });

    // then fallback to discriminant or just index
    index.map(|i| quote! { #i })
        .unwrap_or_else(|| v.discriminant
            .as_ref()
            .map(|&(_, ref expr)| quote! { #expr as u8 })
            .unwrap_or_else(|| quote! { #i as u8})
        )
}

fn find_meta_item<'a, F, R, I, M>(mut itr: I, mut pred: F) -> Option<R> where
    F: FnMut(M) -> Option<R> + Clone,
    I: Iterator<Item=&'a Attribute>,
    M: Parse,
{
    itr.find_map(|attr| attr.path.is_ident("codec").then(|| pred(attr.parse_args().ok()?)).flatten())
}
