use serde::{Deserialize, Serialize};

pub use iroha_introspect_derive::Introspect;
use std::collections::{BTreeMap, BTreeSet};

pub trait Introspect {
    fn introspect() -> Metadata;
}

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub enum Metadata {
    Int(IntMeta),
    Struct(StructMeta),
    Enum(EnumMeta),
    String,
    Bool,
    Vec(SingleContainer),
    Option(SingleContainer),
    Result(ResultMeta),
}

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct StructMeta {
    pub ident: String,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct SingleContainer {
    pub item: Box<Metadata>,
}

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct EnumMeta {
    pub ident: String,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: String,
    pub discriminant: u8,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct ResultMeta {
    pub ok: Box<Metadata>,
    pub err: Box<Metadata>,
}

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct IntMeta {
    pub mode: Mode,
}

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub enum Mode {
    FixedWidth,
    Compact,
}

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Declaration {
    pub name: Option<String>,
    pub definition: Box<Metadata>,
}

impl Introspect for String {
    fn introspect() -> Metadata {
        Metadata::String
    }
}

impl<T> Introspect for Vec<T> where T: Introspect {
    fn introspect() -> Metadata {
        Metadata::Vec(SingleContainer { item: Box::new(T::introspect()) })
    }
}

impl<T> Introspect for Option<T> where T: Introspect {
    fn introspect() -> Metadata {
        Metadata::Option(SingleContainer { item: Box::new(T::introspect()) })
    }
}

impl Introspect for bool {
    fn introspect() -> Metadata {
        Metadata::Bool
    }
}

impl<T: Introspect> Introspect for Box<T> {
    fn introspect() -> Metadata {
        T::introspect()
    }
}

impl<T, E> Introspect for Result<T, E> where T: Introspect, E: Introspect {
    fn introspect() -> Metadata {
        Metadata::Result(ResultMeta {
            ok: Box::new(T::introspect()),
            err: Box::new(E::introspect()),
        })
    }
}

impl<K, V> Introspect for BTreeMap<K, V> where K : Introspect, V: Introspect {
    fn introspect() -> Metadata {
       //todo finish me
        Metadata::String
    }
}

impl<V> Introspect for BTreeSet<V> where V: Introspect {
    fn introspect() -> Metadata {
        Metadata::Vec(SingleContainer { item: Box::new(V::introspect()) })
    }
}

macro_rules! introspect_for_numerics {
        { $( $ty:ty ),* } => {
            $(
                impl Introspect for $ty {
                    fn introspect() -> Metadata {
                        Metadata::Int(IntMeta { mode: Mode::FixedWidth })
                    }
                }
            )*
    };
}

introspect_for_numerics!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

pub mod prelude {
    //! Exports common types for permissions.

    pub use super::*;
}
