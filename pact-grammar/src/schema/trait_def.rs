use crate::schema::{Block, BlockSpan, Literal};

#[derive(Debug, Clone)]
pub struct TraitDef {
    pub name: String,
    pub variants: Vec<TraitVariant>,
    pub doc: Option<String>,
    pub block: BlockSpan,
}

impl Block for TraitDef {
    fn block(&self) -> &BlockSpan {
        &self.block
    }
}

#[derive(Debug, Clone)]
pub struct TraitVariant {
    pub name: String,
    pub props: Vec<TraitProp>,
    pub block: BlockSpan,
}

impl Block for TraitVariant {
    fn block(&self) -> &BlockSpan {
        &self.block
    }
}

#[derive(Debug, Clone)]
pub struct TraitProp {
    pub name: String,
    pub value: Literal,
    pub block: BlockSpan,
}

impl Block for TraitProp {
    fn block(&self) -> &BlockSpan {
        &self.block
    }
}

#[derive(Debug, Clone)]
pub struct ImplForDef {
    pub feature_name: String,
    pub trait_name: String,
    pub impl_blocks: Vec<ImplBlock>,
    pub block: BlockSpan,
}

impl Block for ImplForDef {
    fn block(&self) -> &BlockSpan {
        &self.block
    }
}

#[derive(Debug, Clone)]
pub struct ImplBlock {
    pub variant_name: String,
    pub flags: Vec<String>,
    pub block: BlockSpan,
}

impl Block for ImplBlock {
    fn block(&self) -> &BlockSpan {
        &self.block
    }
}
