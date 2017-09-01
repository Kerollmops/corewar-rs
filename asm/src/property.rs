use super::{Rule, AsmSpan, AsmPair};

#[derive(Debug)]
pub struct Property {
    pub name: AsmSpan,
    pub value: Option<AsmSpan>,
}
impl From<AsmPair> for Property {
    fn from(value: AsmPair) -> Self {
        let mut value = value.into_inner();

        let name = value.by_ref().find(|p| p.as_rule() == Rule::prop_name).unwrap();

        let value = value.by_ref().find(|p| p.as_rule() == Rule::prop_value);
        let quotted = value.map(|v| v.into_inner().find(|p| p.as_rule() == Rule::quotted_string).unwrap());
        let value_string = quotted.map(|q| q.into_inner().find(|p| p.as_rule() == Rule::inner_string).unwrap());

        Property {
            name: name.into_span(),
            value: value_string.map(|v| v.into_span()),
        }
    }
}
