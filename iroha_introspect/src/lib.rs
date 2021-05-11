use std::collections::BTreeSet;

pub mod derive {

    pub use iroha_introspect_derive::Introspect;

}

pub trait Introspect {
    fn introspect() -> BTreeSet<String>;
}

impl Introspect for u128 {
    fn introspect() -> BTreeSet<String> {
       BTreeSet::new()
    }
}
