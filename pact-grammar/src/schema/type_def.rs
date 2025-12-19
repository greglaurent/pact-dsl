use core::{fmt::Debug, hash::Hash};

use crate::schema::{Block, BlockSpan};

#[derive(Debug, Clone)]
pub struct TypeAliasDef {
    pub name: String,
    pub ty: TypeExpr,
    pub doc: Option<String>,
    pub block: BlockSpan,
}

impl Block for TypeAliasDef {
    fn block(&self) -> &BlockSpan {
        &self.block
    }
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
