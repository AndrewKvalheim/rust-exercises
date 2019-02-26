use crate::utilities::CollectionUtilities;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub attrs: HashMap<String, String>,
    pub id: String,
}

impl Node {
    pub fn new(id: &str) -> Self {
        Self {
            attrs: HashMap::new(),
            id: id.to_string(),
        }
    }

    pub fn get_attr(&self, key: &str) -> Option<&str> {
        self.attrs.get(key).map(String::as_str)
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        self.attrs.extend_from_strs(attrs);

        self
    }
}
