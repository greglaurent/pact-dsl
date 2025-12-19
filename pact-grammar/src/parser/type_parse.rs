use crate::{
    parser::{PestParser, Rule, parse_primitive},
    schema::{Span, TypeAliasDef, TypeExpr},
};

impl PestParser for TypeAliasDef {
    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Result<Self, String> {
        let span = pair.as_span();
        let span_range = span.start()..span.end();

        let mut inner = pair.into_inner();

        let name_pair = inner.next().ok_or("Type alias missing name.")?;
        let name_span = name_pair.as_span();
        let name = name_pair.as_str().to_string();

        let ty = TypeExpr::parse_pair(inner.next().ok_or("Type alias missing type")?)?;

        Ok(TypeAliasDef {
            name,
            span,
            ty,
            doc: None,
        })
    }
}

impl PestParser for TypeExpr {
    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Result<Self, String> {
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
                Ok(TypeExpr::Set(Box::new(TypeExpr::parse_pair(elem)?)))
            }
            _ => Err(format!("unexpected type expr rule: {:?}", inner.as_rule())),
        }
    }
}

impl PestParser for Vec<TypeExpr> {
    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Result<Self, String> {
        let mut types = Vec::new();
        for p in pair.into_inner() {
            if p.as_rule() == Rule::tuple_elem {
                let inner = p.into_inner().next().ok_or("empty tuple elem")?;
                types.push(TypeExpr::parse_pair(inner)?);
            }
        }
        Ok(types)
    }
}
