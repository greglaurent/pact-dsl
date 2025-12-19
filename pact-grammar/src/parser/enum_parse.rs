use crate::{
    parser::{PestParser, Rule},
    schema::{EnumDef, EnumVariant, Field, StructKind, TypeExpr},
};

impl PestParser for EnumDef {
    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Result<Self, String> {
        let mut inner = pair.into_inner();
        let name = inner
            .next()
            .ok_or("enum missing name")?
            .as_str()
            .to_string();

        let variants = inner
            .filter(|p| p.as_rule() == Rule::enum_variant_list)
            .flat_map(|p| p.into_inner())
            .filter(|v| v.as_rule() == Rule::enum_variant)
            .map(EnumVariant::parse_pair)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(EnumDef {
            name,
            variants,
            doc: None,
        })
    }
}

impl PestParser for EnumVariant {
    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Result<Self, String> {
        let mut inner = pair.into_inner();
        let name = inner
            .next()
            .ok_or("Enum variant missing name.")?
            .as_str()
            .to_string();

        let kind = if let Some(body) = inner.next() {
            match body.as_rule() {
                Rule::field_list => StructKind::Named(Vec::<Field>::parse_pair(body)?),
                Rule::tuple_list => StructKind::Tuple(Vec::<TypeExpr>::parse_pair(body)?),
                _ => StructKind::Unit,
            }
        } else {
            StructKind::Unit
        };

        Ok(EnumVariant { name, kind })
    }
}
