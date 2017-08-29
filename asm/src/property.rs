use pest::inputs::StringInput;
use pest::iterators::Pair;
use super::Rule;

#[derive(Debug)]
pub struct Property {
    pub name: String,
    pub value: Option<String>,
}

type PropPair = Pair<Rule, StringInput>;

impl From<PropPair> for Property {
    fn from(value: PropPair) -> Property {
        let mut value = value.into_inner();

        let name = value.by_ref().find(|p| p.as_rule() == Rule::prop_name).unwrap();

        let value = value.by_ref().find(|p| p.as_rule() == Rule::prop_value);
        let quotted = value.map(|v| v.into_inner().find(|p| p.as_rule() == Rule::quotted_string).unwrap());
        let value_string = quotted.map(|q| q.into_inner().find(|p| p.as_rule() == Rule::inner_string).unwrap());

        Property {
            name: name.into_span().as_str().into(),
            value: value_string.map(|v| v.into_span().as_str().into()),
        }
    }
}
