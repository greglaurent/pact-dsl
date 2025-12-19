use pest::Span;

pub use crate::schema::{
    enum_def::{EnumDef, EnumVariant},
    struct_def::{Field, FlagsDef, StructDef, StructKind},
    trait_def::{ImplBlock, ImplForDef, TraitDef, TraitProp, TraitVariant},
    type_def::{Primitive, Signed, TypeAliasDef, TypeExpr, Unsigned},
};

mod enum_def;
mod struct_def;
mod trait_def;
mod type_def;

pub trait Block {
    fn block(&self) -> &BlockSpan;
}

#[derive(Debug, Clone)]
pub struct BlockSpan {
    pub name: Span<'static>,
    pub full: Span<'static>,
}

#[derive(Debug, Clone)]
pub struct Schema {
    pub items: Vec<Item>,
}

impl Schema {
    pub fn map_types() {}
}

#[derive(Debug, Clone)]
pub enum Item {
    Struct(StructDef),
    Enum(EnumDef),
    TypeAlias(TypeAliasDef),
    Flags(FlagsDef),
    Trait(TraitDef),
    ImplFor(ImplForDef),
}

#[derive(Debug, Clone)]
pub enum DefaultValue {
    Literal(Literal),
    Option(Option<Literal>),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Bool(bool),
    Number(f64),
    String(String),
}
