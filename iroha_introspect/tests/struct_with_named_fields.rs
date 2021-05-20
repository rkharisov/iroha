use iroha_introspect::prelude::*;

use parity_scale_codec::{Decode, Encode};

#[derive(Introspect, Encode, Decode)]
struct Command {
    executable: String,
    args: Vec<String>,
    #[codec(skip)]
    mock: bool,
    num: i32
}

fn main() {
    let expected_meta = Metadata::Struct(StructMeta {
        ident: "Command".into(),
        declarations: vec![
            Declaration { name: Some("executable".into()), definition: Box::new(Metadata::String)},
            Declaration { name: Some("args".into()), definition: Box::new(Metadata::Vec(SingleContainer { item: Box::new(Metadata::String) })) },
            Declaration { name: Some("num".into()), definition: Box::new(Metadata::Int(IntMeta { mode: Mode::FixedWidth })) },
        ],
    });
    assert_eq!(expected_meta, Command::introspect());
}
