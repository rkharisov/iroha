use parity_scale_codec::{Decode, Encode};

use iroha_introspect::prelude::*;


#[derive(Introspect, Encode, Decode)]
struct Command(String, Vec<String>, #[codec(skip)] bool);

fn main() {
    let expected_meta = Metadata::Struct(StructMeta {
        ident: "Command".into(),
        declarations: vec![
            Declaration { name: None, definition: Box::new(Metadata::String)},
            Declaration { name: None, definition: Box::new(Metadata::Vec(SingleContainer { item: Box::new(Metadata::String) })) },
        ],
    });
    assert_eq!(expected_meta, Command::introspect())
}
