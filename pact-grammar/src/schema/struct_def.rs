use super::{DefaultValue, type_def::TypeExpr};
use crate::schema::{Block, BlockSpan, type_def::Unsigned};

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
    pub block: BlockSpan,
}

impl Block for Field {
    fn block(&self) -> &BlockSpan {
        &self.block
    }
}

#[derive(Debug, Clone)]
pub struct StructDef {
    pub name: String,
    pub kind: StructKind,
    pub doc: Option<String>,
    pub block: BlockSpan,
}

impl Block for StructDef {
    fn block(&self) -> &BlockSpan {
        &self.block
    }
}

#[derive(Debug, Clone)]
pub struct FlagsDef {
    pub name: String,
    pub repr: Unsigned,
    pub variants: Vec<String>,
    pub doc: Option<String>,
    pub block: BlockSpan,
}

impl Block for FlagsDef {
    fn block(&self) -> &BlockSpan {
        &self.block
    }
}
