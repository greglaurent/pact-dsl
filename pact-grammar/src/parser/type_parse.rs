use crate::{
    parser::{PestParser, Rule},
    schema::{Primitive, TypeAliasDef, TypeExpr},
};

impl PestParser for TypeAliasDef {
    type S = Self;
    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Result<Self::S, String> {
        let mut inner = pair.into_inner();
        let name = inner
            .next()
            .ok_or("Type alias missing name.")?
            .as_str()
            .to_string();
        let ty = TypeExpr::parse_pair(inner.next().ok_or("Type alias missing type")?)?;

        Ok(TypeAliasDef {
            name,
            ty,
            doc: None,
        })
    }
}

impl PestParser for TypeExpr {
    type S = Self;

    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Result<Self::S, String> {
        let inner = pair.into_inner().next().ok_or("empty type expr")?;

        match inner.as_rule() {
            Rule::primitive => Ok(TypeExpr::Primitive(parse_primitive(inner.as_str())?)),
            Rule::identifier => Ok(TypeExpr::Named(inner.as_str().to_string())),
            Rule::array_type => {
                let elem = inner
                    .into_inner()
                    .next()
                    .ok_or("array missing element type")?;
                Ok(TypeExpr::Array(Box::new(Self::parse_pair(elem)?)))
            }
            Rule::array_fixed_type => {
                let mut parts = inner.into_inner();
                let elem = parts.next().ok_or("fixed array missing element type")?;
                let size_str = parts.next().ok_or("fixed array missing size")?.as_str();
                let size: usize = size_str.parse().map_err(|_| "invalid array size")?;
                Ok(TypeExpr::ArrayFixed(
                    Box::new(Self::parse_pair(elem)?),
                    size,
                ))
            }
            Rule::map_type => {
                let mut parts = inner.into_inner();
                let key = parts.next().ok_or("map missing key type")?;
                let value = parts.next().ok_or("map missing value type")?;
                Ok(TypeExpr::Map(
                    Box::new(Self::parse_pair(key)?),
                    Box::new(Self::parse_pair(value)?),
                ))
            }
            Rule::set_type => {
                let elem = inner
                    .into_inner()
                    .next()
                    .ok_or("set missing element type")?;
                Ok(TypeExpr::Set(Box::new(parse_type_expr(elem)?)))
            }
            _ => Err(format!("unexpected type expr rule: {:?}", inner.as_rule())),
        }
    }
}

impl PestParser for Primitive {
    fn parse_primitive(s: &str) -> Result<Primitive, String> {
        match s {
            "bool" => Ok(Primitive::Bool),
            "u8" => Ok(Primitive::U8),
            "u16" => Ok(Primitive::U16),
            "u32" => Ok(Primitive::U32),
            "u64" => Ok(Primitive::U64),
            "i8" => Ok(Primitive::I8),
            "i16" => Ok(Primitive::I16),
            "i32" => Ok(Primitive::I32),
            "i64" => Ok(Primitive::I64),
            "f32" => Ok(Primitive::F32),
            "f64" => Ok(Primitive::F64),
            "String" => Ok(Primitive::String),
            _ => Err(format!("unknown primitive: {}", s)),
        }
    }
}
