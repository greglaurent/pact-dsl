use crate::schema::type_def::Unsigned;

#[derive(Debug, Clone)]
pub struct FlagsDef {
    pub name: String,
    pub repr: Unsigned,
    pub variants: Vec<String>,
    pub doc: Option<String>,
}
