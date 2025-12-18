use core::{fmt::Debug, hash::Hash};

#[derive(Debug, Clone)]
pub struct TypeAlias {
    pub name: String,
    pub ty: TypeExpr,
    pub doc: Option<String>,
}

#[derive(Debug, Clone)]
pub enum TypeExpr {
    Primitive(Primitive),
    Named(String),
    Array(Box<TypeExpr>),
    ArrayFixed(Box<TypeExpr>, usize),
    Map(Box<TypeExpr>, Box<TypeExpr>),
    Set(Box<TypeExpr>),
}

pub trait Prim {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Unsigned {
    U8,
    U16,
    U32,
    U64,
    Usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Signed {
    I8,
    I16,
    I32,
    I64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Primitive {
    Bool,
    F32,
    F64,
    Signed(Signed),
    Unsigned(Unsigned),
    String,
}
