use introspect::*;
use introspect_derive::Introspect;
use parity_scale_codec::{Decode, Encode};

#[derive(Introspect, Encode, Decode)]
enum Foo {
    #[codec(index = 1)]
    A,
    B = 77,
    C,
    #[codec(index = 99)]
    D = 88,
}

fn main() {
    let expected_meta = Metadata::Enum(EnumMeta {
        ident: "Foo".into(),
        variants: vec![
            EnumVariant {
                name: "A".into(),
                discriminant: 1,
                declarations: vec![],
            },
            EnumVariant {
                name: "B".into(),
                discriminant: 77,
                declarations: vec![],
            },
            EnumVariant {
                name: "C".into(),
                discriminant: 2,
                declarations: vec![],
            },
            EnumVariant {
                name: "D".into(),
                discriminant: 99,
                declarations: vec![],
            },
        ],
    });
    assert_eq!(expected_meta, Foo::introspect());
}
