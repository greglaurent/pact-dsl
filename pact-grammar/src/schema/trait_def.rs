use crate::schema::Literal;

#[derive(Debug, Clone)]
pub struct TraitDef {
    pub name: String,
    pub variants: Vec<TraitVariant>,
    pub doc: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TraitVariant {
    pub name: String,
    pub props: Vec<TraitProp>,
}

#[derive(Debug, Clone)]
pub struct TraitProp {
    pub name: String,
    pub value: Literal,
}

#[derive(Debug, Clone)]
pub struct ImplForDef {
    pub feature_name: String,
    pub trait_name: String,
    pub blocks: Vec<ImplBlock>,
}

#[derive(Debug, Clone)]
pub struct ImplBlock {
    pub variant_name: String,
    pub flags: Vec<String>,
}
