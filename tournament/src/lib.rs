mod stats;
mod stats_by_team;

use stats_by_team::StatsByTeam;
use std::convert::TryFrom;

pub fn tally(log: &str) -> String {
    let stats_by_team = StatsByTeam::try_from(log).expect("failed to parse log");

    format!("{}", stats_by_team)
}
