use crate::{
    parser::{PestParser, Rule},
    schema::{DefaultValue, Field, FlagsDef, StructDef, StructKind, TypeExpr, Unsigned},
};

impl PestParser for Field {
    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Result<Self, String> {
        let mut inner = pair.into_inner();
        let name = inner
            .next()
            .ok_or("field missing name")?
            .as_str()
            .to_string();
        let ty = TypeExpr::parse_pair(inner.next().ok_or("field missing type")?)?;

        let default = if let Some(default_pair) = inner.next() {
            Some(DefaultValue::parse_pair(default_pair)?)
        } else {
            None
        };

        Ok(Field { name, ty, default })
    }
}

impl PestParser for Vec<Field> {
    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Result<Self, String> {
        let mut fields = Vec::new();
        for p in pair.into_inner() {
            if p.as_rule() == Rule::field {
                fields.push(Field::parse_pair(p)?);
            }
        }
        Ok(fields)
    }
}

impl PestParser for StructDef {
    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Result<Self, String> {
        let mut inner = pair.into_inner();
        let name = inner
            .next()
            .ok_or("Struct definition missing a name.")?
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

        Ok(StructDef {
            name,
            kind,
            doc: None,
        })
    }
}

//TODO: Hinge this on bitwidth
impl PestParser for FlagsDef {
    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Result<FlagsDef, String> {
        let mut inner = pair.into_inner();
        let name = inner
            .next()
            .ok_or("flags missing name")?
            .as_str()
            .to_string();

        let mut repr = Unsigned::U32; // default
        let mut variants = Vec::new();

        for p in inner {
            match p.as_rule() {
                Rule::flags_repr => {
                    repr = match p.as_str() {
                        "u8" => Unsigned::U8,
                        "u16" => Unsigned::U16,
                        "u32" => Unsigned::U32,
                        "u64" => Unsigned::U64,
                        "usize" => Unsigned::Usize,
                        _ => Unsigned::U32,
                    };
                }
                Rule::flags_variant_list => {
                    for v in p.into_inner() {
                        if v.as_rule() == Rule::identifier {
                            variants.push(v.as_str().to_string());
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(FlagsDef {
            name,
            repr,
            variants,
            doc: None,
        })
    }
}
