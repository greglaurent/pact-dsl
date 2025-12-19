use crate::schema::{Block, BlockSpan, StructKind};

#[derive(Debug, Clone)]
pub struct EnumDef {
    pub name: String,
    pub variants: Vec<EnumVariant>,
    pub doc: Option<String>,
    pub block: BlockSpan,
}

impl Block for EnumDef {
    fn block(&self) -> &BlockSpan {
        &self.block
    }
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub kind: StructKind,
    pub block: BlockSpan,
}

impl Block for EnumVariant {
    fn block(&self) -> &BlockSpan {
        &self.block
    }
}
