use std::hash::{Hash, Hasher};
use std::fmt;
use ::{AsmPair, AsmSpan};

#[derive(Clone, Eq)]
pub struct Label {
    pub name: AsmSpan,
}

impl Label {
    pub fn as_span(&self) -> &AsmSpan {
        &self.name
    }
}

impl PartialEq for Label {
    fn eq(&self, other: &Label) -> bool {
        self.name.as_str() == other.name.as_str()
    }
}

impl Hash for Label {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.name.as_str().hash(state)
    }
}

impl fmt::Debug for Label {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Label")
            .field("name", &self.name.as_str())
            .finish()
    }
}

impl From<AsmPair> for Label {
    fn from(value: AsmPair) -> Self {
        let value = value.into_inner().next().unwrap();
        Label {
            name: value.into_span()
        }
    }
}
