use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
pub struct School(BTreeMap<u32, BTreeSet<String>>);

impl School {
    pub fn new() -> Self {
        School(BTreeMap::new())
    }

    pub fn add(&mut self, grade: u32, name: &str) {
        self.0
            .entry(grade)
            .or_insert_with(BTreeSet::new)
            .insert(name.into());
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        Some(self.0.get(&grade)?.iter().cloned().collect())
    }

    pub fn grades(&self) -> Vec<u32> {
        self.0.keys().cloned().collect()
    }
}
