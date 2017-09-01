use pest::inputs::StringInput;
use pest::iterators::Pair;
use super::{Rule, AsmPair};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Label {
    pub name: String,
}

impl From<AsmPair> for Label {
    fn from(value: AsmPair) -> Self {
        let value = value.into_inner().next().unwrap();
        Label {
            name: value.into_span().as_str().into()
        }
    }
}
