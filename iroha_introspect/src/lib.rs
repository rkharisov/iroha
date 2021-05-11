use std::collections::{HashSet, BTreeSet};

//todo precommit hook
pub mod derive {
    pub use iroha_introspect_derive::Introspect;

    use crate::Metadata::{BoolMetadata, OptionMetadata, ResultMetadata, TupleMetadata, VecMetadata};

    use super::*;

    impl Introspect for bool {
        fn introspect() -> Metadata {
            BoolMetadata
        }
    }

    impl<T> Introspect for Option<T> where T: Introspect {
        fn introspect() -> Metadata {
            OptionMetadata(OptionMeta { item: Box::new(T::introspect()) })
        }
    }

    impl<T, E> Introspect for Result<T, E> where T: Introspect, E: Introspect {
        fn introspect() -> Metadata {
            ResultMetadata(ResultMeta {
                ok: Box::new(T::introspect()),
                err: Box::new(E::introspect())
            })
        }
    }

    impl<T> Introspect for Vec<T> where T: Introspect {
        fn introspect() -> Metadata {
            VecMetadata(VecMeta { item: Box::new(T::introspect())})
        }
    }

    impl Introspect for ()  {
        fn introspect() -> Metadata {
            TupleMetadata(TupleMeta { items: Vec::new()})
        }
    }

    //todo macros
    impl<T> Introspect for (T,) where T: Introspect {
        fn introspect() -> Metadata {
            TupleMetadata(TupleMeta{ items: vec![Box::new(T::introspect())]})
        }
    }

    impl<T1, T2> Introspect for (T1, T2) where T1: Introspect, T2: Introspect {
        fn introspect() -> Metadata {
            TupleMetadata(TupleMeta{ items: vec![
                Box::new(T1::introspect()),
                Box::new(T2::introspect()),
            ]})
        }
    }

    impl<T1, T2, T3> Introspect for (T1, T2, T3) where T1: Introspect, T2: Introspect, T3: Introspect {
        fn introspect() -> Metadata {
            TupleMetadata(TupleMeta{ items: vec![
                Box::new(T1::introspect()),
                Box::new(T2::introspect()),
                Box::new(T3::introspect()),
            ]})
        }
    }

    impl<T1, T2, T3, T4> Introspect for (T1, T2, T3, T4) where T1: Introspect, T2: Introspect, T3: Introspect, T4: Introspect {
        fn introspect() -> Metadata {
            TupleMetadata(TupleMeta{ items: vec![
                Box::new(T1::introspect()),
                Box::new(T2::introspect()),
                Box::new(T3::introspect()),
                Box::new(T4::introspect()),
            ]})
        }
    }

    impl<T1, T2, T3, T4, T5> Introspect for (T1, T2, T3, T4, T5) where T1: Introspect, T2: Introspect, T3: Introspect, T4: Introspect, T5: Introspect {
        fn introspect() -> Metadata {
            TupleMetadata(TupleMeta{items: vec![
                Box::new(T1::introspect()),
                Box::new(T2::introspect()),
                Box::new(T3::introspect()),
                Box::new(T4::introspect()),
                Box::new(T5::introspect()),
            ]})
        }
    }
}

pub trait Introspect {
    fn introspect() -> Metadata;
}

#[derive(Debug)]
pub enum Metadata {
    IntMetadata(IntMeta),
    BoolMetadata,
    OptionMetadata(OptionMeta),
    ResultMetadata(ResultMeta),
    VecMetadata(VecMeta),
    TupleMetadata(TupleMeta),
    StructMetadata(StructMeta),
    EnumMetadata(EnumMeta),
}

#[derive(Debug)]
pub struct IntMeta {
    pub mode: Mode,
}

#[derive(Debug)]
pub struct OptionMeta {
    pub item: Box<Metadata>,
}

#[derive(Debug)]
pub struct ResultMeta {
    pub ok: Box<Metadata>,
    pub err: Box<Metadata>,
}

#[derive(Debug)]
pub struct VecMeta {
    item: Box<Metadata>,
}

#[derive(Debug)]
pub struct TupleMeta {
    items: Vec<Box<Metadata>>,
}

// //todo diff between declaration and definition
#[derive(Debug)]
pub enum Declaration {
    IntDeclaration {
        //name of the property
        name: String,
        decl: IntMeta,
    },
    BoolDeclaration {
        name: String,
    },
    OptionDeclaration {
        name: String,
        decl: OptionMeta,
    },
    ResultDeclaration {
        name: String,
        decl: ResultMeta,
    },
    VecDeclaration {
        name: String,
        decl: VecMeta,
    },
    TupleDeclaration {
        name: String,
        decl: TupleMeta,
    },
    StructDeclaration {
        name: String,
        //name of the pub struct meta
        meta_ident: String,
    },
    EnumDeclaration {
        name: String,
        //name of the pub enum meta
        meta_ident: String,
    },
}
#[derive(Debug)]
pub enum Mode {
    FixedWidth,
    CompactSingleByte,
    CompactTwoByte,
    CompactFourByte,
    CompactBigInteger,
}
#[derive(Debug)]
pub enum Codec {
    Scale,
    Json,
}
#[derive(Debug)]
pub struct StructMeta {
    ident: String,
    codec: Codec,
    declarations: Vec<Declaration>,
    metas: HashSet<Box<Metadata>>,
}
#[derive(Debug)]
pub struct EnumMeta {
    //todo type to deserialize
    //todo correct name?
    ident: String,
    codec: Codec,
    variants: Vec<EnumVariant>
}
#[derive(Debug)]
pub struct EnumVariant {
    name: String,
    discriminator: u32,
    declarations: Vec<Declaration>,
}
