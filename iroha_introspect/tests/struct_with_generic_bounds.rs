
use iroha_introspect::prelude::*;

#[derive(Introspect)]
struct Foo<V : Introspect + Sized> {
    value: Option<V>,
}

fn main() {
    let expected_metadata = Metadata::Struct(
        StructMeta {
            ident: "Foo".to_string(),
            declarations: vec![
                Declaration{
                    name: Some("value".into()),
                    definition: Box::new(Metadata::Option(SingleContainer {item: Box::new(Metadata::Bool) }))
                }
            ]
        }
    );
    assert_eq!(expected_metadata, Foo::<bool>::introspect());
}
