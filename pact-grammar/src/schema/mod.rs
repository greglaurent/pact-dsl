use crate::schema::{
    flag_def::FlagsDef,
    trait_def::{ImplForDef, TraitDef},
};

pub use crate::schema::{
    enum_def::{EnumDef, EnumVariant},
    struct_def::{StructDef, StructKind},
    type_def::{Primitive, TypeAliasDef, TypeExpr},
};

mod enum_def;
mod flag_def;
mod struct_def;
mod trait_def;
mod type_def;

#[derive(Debug, Clone)]
pub struct Schema {
    pub items: Vec<Item>,
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
