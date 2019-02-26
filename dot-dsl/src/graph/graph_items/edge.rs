use crate::utilities::CollectionUtilities;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
    pub attrs: HashMap<String, String>,
}

impl Edge {
    pub fn new(_: &str, _: &str) -> Self {
        Self {
            attrs: HashMap::new(),
        }
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        self.attrs.extend_from_strs(attrs);

        self
    }
}
