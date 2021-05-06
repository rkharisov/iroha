pub mod derive {

    pub use iroha_introspect_derive::Introspect;

}

pub trait Introspect {
    fn introspect();
}
