use parity_scale_codec::{Decode, Encode};

use introspect::*;
use introspect_derive::Introspect;

#[derive(Introspect, Encode)]
struct Foo {
    #[codec(compact)]
    u8_compact: u8,
    u8_fixed: u8,
    #[codec(compact)]
    u16_compact: u16,
    u16_fixed: u16,
    #[codec(compact)]
    u32_compact: u32,
    u32_fixed: u32,
    #[codec(compact)]
    u64_compact: u64,
    u64_fixed: u64,
    #[codec(compact)]
    u128_compact: u128,
    u128_fixed: u128,
}

fn main() {
    let expected_metadata = Metadata::Struct(StructMeta {
        ident: "Foo".to_string(),
        declarations: vec![
            Declaration { name: Some("u8_compact".into()), definition: Box::new(Metadata::Int(IntMeta{ mode: Mode::Compact})) },
            Declaration { name: Some("u8_fixed".into()), definition: Box::new(Metadata::Int(IntMeta{ mode: Mode::FixedWidth})) },
            Declaration { name: Some("u16_compact".into()), definition: Box::new(Metadata::Int(IntMeta{ mode: Mode::Compact})) },
            Declaration { name: Some("u16_fixed".into()), definition: Box::new(Metadata::Int(IntMeta{ mode: Mode::FixedWidth})) },
            Declaration { name: Some("u32_compact".into()), definition: Box::new(Metadata::Int(IntMeta{ mode: Mode::Compact})) },
            Declaration { name: Some("u32_fixed".into()), definition: Box::new(Metadata::Int(IntMeta{ mode: Mode::FixedWidth})) },
            Declaration { name: Some("u64_compact".into()), definition: Box::new(Metadata::Int(IntMeta{ mode: Mode::Compact})) },
            Declaration { name: Some("u64_fixed".into()), definition: Box::new(Metadata::Int(IntMeta{ mode: Mode::FixedWidth})) },
            Declaration { name: Some("u128_compact".into()), definition: Box::new(Metadata::Int(IntMeta{ mode: Mode::Compact})) },
            Declaration { name: Some("u128_fixed".into()), definition: Box::new(Metadata::Int(IntMeta{ mode: Mode::FixedWidth})) },
        ]
    });
    assert_eq!(expected_metadata, Foo::introspect());
}
