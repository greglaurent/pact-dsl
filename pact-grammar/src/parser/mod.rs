use pest::Parser;
use pest_derive::Parser;

mod enum_parse;
mod struct_parse;
mod trait_parse;
mod type_parse;

use crate::schema::{
    DefaultValue, EnumDef, FlagsDef, ImplForDef, Item, Literal, Primitive, Schema, Signed,
    StructDef, TraitDef, TypeAliasDef, Unsigned,
};

#[derive(Parser)]
#[grammar = "./src/spec.pest"]
pub struct SchemaParser;

pub fn parse(input: &str) -> Result<Schema, String> {
    let pairs = SchemaParser::parse(Rule::schema, input).map_err(|e| e.to_string())?;
    let items = pairs
        .flatten()
        .filter(|p| p.as_rule() == Rule::item)
        .map(Item::parse_pair)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Schema { items })
}

fn parse_primitive(s: &str) -> Result<Primitive, String> {
    match s {
        "bool" => Ok(Primitive::Bool),
        "u8" => Ok(Primitive::Unsigned(Unsigned::U8)),
        "u16" => Ok(Primitive::Unsigned(Unsigned::U16)),
        "u32" => Ok(Primitive::Unsigned(Unsigned::U32)),
        "u64" => Ok(Primitive::Unsigned(Unsigned::U64)),
        "i8" => Ok(Primitive::Signed(Signed::I8)),
        "i16" => Ok(Primitive::Signed(Signed::I16)),
        "i32" => Ok(Primitive::Signed(Signed::I32)),
        "i64" => Ok(Primitive::Signed(Signed::I64)),
        "f32" => Ok(Primitive::F32),
        "f64" => Ok(Primitive::F64),
        "String" => Ok(Primitive::String),
        _ => Err(format!("Unknown primitive: {}", s)),
    }
}

trait PestParser: Sized {
    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Result<Self, String>;
}

impl PestParser for Item {
    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Result<Self, String> {
        let inner = pair.into_inner().next().ok_or("empty item")?;

        match inner.as_rule() {
            Rule::struct_def => Ok(Item::Struct(StructDef::parse_pair(inner)?)),
            Rule::enum_def => Ok(Item::Enum(EnumDef::parse_pair(inner)?)),
            Rule::type_alias => Ok(Item::TypeAlias(TypeAliasDef::parse_pair(inner)?)),
            Rule::flags_def => Ok(Item::Flags(FlagsDef::parse_pair(inner)?)),
            Rule::trait_def => Ok(Item::Trait(TraitDef::parse_pair(inner)?)),
            Rule::impl_for_def => Ok(Item::ImplFor(ImplForDef::parse_pair(inner)?)),
            _ => panic!("Fix this"),
        }
    }
}

impl PestParser for DefaultValue {
    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Result<DefaultValue, String> {
        let inner = pair.into_inner().next().ok_or("empty default value")?;

        match inner.as_rule() {
            Rule::option_default => {
                let lit = inner
                    .into_inner()
                    .next()
                    .map(|p| Literal::parse_pair(p))
                    .transpose()?;
                Ok(DefaultValue::Option(lit))
            }
            Rule::literal => Ok(DefaultValue::Literal(Literal::parse_pair(inner)?)),
            _ => Err(format!(
                "unexpected default value rule: {:?}",
                inner.as_rule()
            )),
        }
    }
}

impl PestParser for Literal {
    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Result<Self, String> {
        let inner = pair.into_inner().next().ok_or("empty literal")?;

        match inner.as_rule() {
            Rule::bool_lit => Ok(Literal::Bool(inner.as_str() == "true")),
            Rule::number_lit => {
                let n: f64 = inner.as_str().parse().map_err(|_| "invalid number")?;
                Ok(Literal::Number(n))
            }
            Rule::string_lit => {
                let content = inner.into_inner().next().ok_or("empty string")?;
                Ok(Literal::String(content.as_str().to_string()))
            }
            _ => Err(format!("unexpected literal rule: {:?}", inner.as_rule())),
        }
    }
}
