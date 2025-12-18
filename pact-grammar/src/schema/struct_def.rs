use super::{DefaultValue, type_def::TypeExpr};

#[derive(Debug, Clone)]
pub struct StructDef {
    pub name: String,
    pub kind: StructKind,
    pub doc: Option<String>,
}

#[derive(Debug, Clone)]
pub enum StructKind {
    Unit,
    Tuple(Vec<TypeExpr>),
    Named(Vec<Field>),
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub ty: TypeExpr,
    pub default: Option<DefaultValue>,
}
