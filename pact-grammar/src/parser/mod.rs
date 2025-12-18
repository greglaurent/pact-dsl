use pest::Parser;
use pest_derive::Parser;

mod type_parse;

use crate::schema::{EnumDef, EnumVariant, Item, Schema, StructDef, StructKind, TypeAliasDef};

pub trait PestParser<T> {
    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Result<T, String>;
}

#[derive(Parser)]
#[grammar = "./src/spec.pest"]
pub struct SchemaParser;

pub fn parse(input: &str) -> Result<Schema, String> {
    let pairs = SchemaParser::parse(Rule::schema, input).map_err(|e| e.to_string())?;

    let items = pairs
        .flatten()
        .filter(|p| p.as_rule() == Rule::item)
        .filter_map(|p| Item::parse_pair(p).transpose())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Schema { items })
}

impl PestParser<Item> {
    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Result<Self::S, String> {
        let inner = pair.into_inner().next().ok_or("empty item")?;

        match inner.as_rule() {
            Rule::struct_def => Ok(Some(Item::Struct(StructDef::parse_pair(inner)?))),
            Rule::enum_def => Ok(Some(Item::Enum(parse_enum_def(inner)?))),
            Rule::type_alias => Ok(Some(Item::TypeAlias(parse_type_alias(inner)?))),
            Rule::flags_def => Ok(Some(Item::Flags(parse_flags_def(inner)?))),
            Rule::trait_def => Ok(Some(Item::Trait(parse_trait_def(inner)?))),
            Rule::impl_for_def => Ok(Some(Item::ImplFor(parse_impl_for_def(inner)?))),
            _ => Ok(None),
        }
    }
}

impl PestParser for StructDef {
    type S = Self;

    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Result<Self::S, String> {
        let mut inner = pair.into_inner();
        let name = inner
            .next()
            .ok_or("Struct definition missing a name.")?
            .as_str()
            .to_string();

        let kind = if let Some(body) = inner.next() {
            match body.as_rule() {
                Rule::field_list => StructKind::Named(parse_field_list(body)?),
                Rule::tuple_list => StructKind::Tuple(parse_tuple_list(body)?),
                _ => StructKind::Unit,
            }
        } else {
            StructKind::Unit
        };

        Ok(StructDef {
            name,
            kind,
            doc: None,
        })
    }
}

impl PestParser for EnumDef {
    type S = Self;

    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Result<Self::S, String> {
        let mut inner = pair.into_inner();
        let name = inner
            .next()
            .ok_or("enum missing name")?
            .as_str()
            .to_string();

        let mut variants = Vec::new();
        for p in inner {
            if p.as_rule() == Rule::enum_variant_list {
                for v in p.into_inner() {
                    if v.as_rule() == Rule::enum_variant {
                        variants.push(parse_enum_variant(v)?);
                    }
                }
            }
        }

        Ok(EnumDef {
            name,
            variants,
            doc: None,
        })
    }
}

impl PestParser for EnumVariant {
    type S = Self;

    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Result<Self::S, String> {
        let mut inner = pair.into_inner();
        let name = inner
            .next()
            .ok_or("Enum variant missing name.")?
            .as_str()
            .to_string();

        let kind = if let Some(body) = inner.next() {
            match body.as_rule() {
                Rule::field_list => StructKind::Named(parse_field_list(body)?),
                Rule::tuple_list => StructKind::Tuple(parse_tuple_list(body)?),
                _ => StructKind::Unit,
            }
        } else {
            StructKind::Unit
        };

        Ok(EnumVariant { name, kind })
    }
}
