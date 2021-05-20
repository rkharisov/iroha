
use introspect_derive::Introspect;
use introspect::*;
use parity_scale_codec::{Decode, Encode};

#[derive(Introspect, Encode, Decode)]
enum Foo {
    Variant1 {
        id: String,
        counter: Option<u8>
    },
    Variant2(bool, String, Result<bool, String>),
    Variant3,
    Variant4(#[codec(skip)] String),
}

fn main() {
    let expected_meta = Metadata::Enum(EnumMeta {
        ident: "Foo".into(),
        variants: vec![
            EnumVariant {
                name: "Variant1".into(),
                discriminant: 0,
                declarations: vec![
                    Declaration { name: Some("id".into()), definition: Box::new(Metadata::String) },
                    Declaration {
                        name: Some("counter".into()),
                        definition: Box::new(Metadata::Option(SingleContainer {item: Box::new(Metadata::Int(IntMeta {mode: Mode::FixedWidth}))}))
                    }
                ]
            },
            EnumVariant {
                name: "Variant2".into(),
                discriminant: 1,
                declarations: vec![
                    Declaration { name: None, definition: Box::new(Metadata::Bool) },
                    Declaration { name: None, definition: Box::new(Metadata::String) },
                    Declaration {
                        name: None,
                        definition: Box::new(Metadata::Result(ResultMeta {ok: Box::new(Metadata::Bool), err: Box::new(Metadata::String)}))
                    },
                ]
            },
            EnumVariant {
                name: "Variant3".into(),
                discriminant: 2,
                declarations: vec![]
            },
            EnumVariant {
                name: "Variant4".into(),
                discriminant: 3,
                declarations: vec![]
            },
        ]
    });
    assert_eq!(expected_meta, Foo::introspect());
}
