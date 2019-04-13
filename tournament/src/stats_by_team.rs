use crate::stats::Stats;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::{self, Display};

pub struct StatsByTeam<'a>(HashMap<&'a str, Stats>);

impl<'a> StatsByTeam<'a> {
    fn get_or_insert(&mut self, name: &'a str) -> &mut Stats {
        self.0.entry(name).or_insert_with(Stats::default)
    }
}

impl<'a> Display for StatsByTeam<'a> {
    fn fmt(&self, output: &mut fmt::Formatter) -> fmt::Result {
        let mut rows = self.0.iter().collect::<Vec<_>>();
        rows.sort_unstable_by(|a, b| a.1.points.cmp(&b.1.points).reverse().then(a.0.cmp(&b.0)));

        write!(
            output,
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            "Team", "MP", "W", "D", "L", "P"
        )?;

        for (name, stats) in rows {
            write!(
                output,
                "\n{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
                name, stats.played, stats.won, stats.drawn, stats.lost, stats.points
            )?;
        }

        Ok(())
    }
}

impl<'a> TryFrom<&'a str> for StatsByTeam<'a> {
    type Error = &'a str;

    fn try_from(log: &'a str) -> Result<Self, Self::Error> {
        let mut results = Self(HashMap::new());

        for line in log.lines() {
            let mut fields = line.splitn(3, ';');

            match (fields.next(), fields.next(), fields.next()) {
                (Some(a), Some(b), Some("draw")) => {
                    results.get_or_insert(a).draw();
                    results.get_or_insert(b).draw();
                }
                (Some(a), Some(b), Some("win")) | (Some(b), Some(a), Some("loss")) => {
                    results.get_or_insert(a).win();
                    results.get_or_insert(b).loss();
                }
                _ => return Err(line),
            }
        }

        Ok(results)
    }
}
