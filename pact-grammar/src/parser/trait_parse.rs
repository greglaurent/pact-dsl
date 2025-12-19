use crate::{
    parser::{PestParser, Rule},
    schema::{ImplBlock, ImplForDef, Literal, TraitDef, TraitProp, TraitVariant},
};

impl PestParser for TraitDef {
    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Result<Self, String> {
        let mut inner = pair.into_inner();
        let name = inner
            .next()
            .ok_or("trait missing name")?
            .as_str()
            .to_string();

        let variants = inner
            .filter(|p| p.as_rule() == Rule::trait_variant_list)
            .flat_map(|p| p.into_inner())
            .filter(|v| v.as_rule() == Rule::trait_variant)
            .map(TraitVariant::parse_pair)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(TraitDef {
            name,
            variants,
            doc: None,
        })
    }
}

impl PestParser for TraitVariant {
    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Result<Self, String> {
        let mut inner = pair.into_inner();
        let name = inner
            .next()
            .ok_or("trait variant missing name")?
            .as_str()
            .to_string();

        let props = inner
            .filter(|p| p.as_rule() == Rule::trait_prop_list)
            .flat_map(|p| p.into_inner())
            .filter(|p| p.as_rule() == Rule::trait_prop)
            .map(TraitProp::parse_pair)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(TraitVariant { name, props })
    }
}

impl PestParser for TraitProp {
    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Result<Self, String> {
        let mut inner = pair.into_inner();
        let name = inner
            .next()
            .ok_or("trait prop missing name")?
            .as_str()
            .to_string();

        let value = Literal::parse_pair(inner.next().ok_or("trait prop missing value")?)?;

        Ok(TraitProp { name, value })
    }
}

impl PestParser for ImplForDef {
    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Result<Self, String> {
        let mut inner = pair.into_inner();
        let feature_name = inner
            .next()
            .ok_or("impl missing feature name")?
            .as_str()
            .to_string();
        let trait_name = inner
            .next()
            .ok_or("impl missing trait name")?
            .as_str()
            .to_string();

        let blocks = inner
            .filter(|p| p.as_rule() == Rule::impl_block_list)
            .flat_map(|p| p.into_inner())
            .filter(|b| b.as_rule() == Rule::impl_block)
            .map(ImplBlock::parse_pair)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(ImplForDef {
            feature_name,
            trait_name,
            blocks,
        })
    }
}

impl PestParser for ImplBlock {
    fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Result<Self, String> {
        let mut inner = pair.into_inner();
        let variant_name = inner
            .next()
            .ok_or("impl block missing variant name")?
            .as_str()
            .to_string();

        let flags = inner
            .filter(|p| p.as_rule() == Rule::identifier_list)
            .flat_map(|p| p.into_inner())
            .filter(|id| id.as_rule() == Rule::identifier)
            .map(|id| id.as_str().to_string())
            .collect();

        Ok(ImplBlock {
            variant_name,
            flags,
        })
    }
}
