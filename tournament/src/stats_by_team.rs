use crate::stats::Stats;
use std::collections::HashMap;

pub struct StatsByTeam<'a>(HashMap<&'a str, Stats>);

impl<'a> StatsByTeam<'a> {
    pub fn iter(&self) -> impl Iterator<Item = (&&str, &Stats)> {
        self.0.iter()
    }

    fn get_or_insert(&mut self, name: &'a str) -> &mut Stats {
        self.0.entry(name).or_insert_with(Stats::default)
    }
}

impl<'a> From<&'a str> for StatsByTeam<'a> {
    fn from(log: &'a str) -> Self {
        let mut results = Self(HashMap::new());

        for line in log.lines() {
            let mut fields = line.splitn(3, ';');

            match (fields.next(), fields.next(), fields.next()) {
                (Some(a), Some(b), Some("draw")) => {
                    results.get_or_insert(a).draw();
                    results.get_or_insert(b).draw();
                }
                (Some(a), Some(b), Some("loss")) | (Some(b), Some(a), Some("win")) => {
                    results.get_or_insert(a).loss();
                    results.get_or_insert(b).win();
                }
                _ => panic!("unrecognized data format"),
            }
        }

        results
    }
}
