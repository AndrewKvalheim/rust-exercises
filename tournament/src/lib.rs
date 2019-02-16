mod stats;
mod stats_by_team;

use stats_by_team::StatsByTeam;
use std::iter::once;

pub fn tally(log: &str) -> String {
    // Read
    let stats_by_team = StatsByTeam::from(log);

    // Sort
    let mut teams = stats_by_team.iter().collect::<Vec<_>>();
    teams.sort_unstable_by(|a, b| a.1.points.cmp(&b.1.points).reverse().then(a.0.cmp(&b.0)));

    // Render
    let header = "Team                           | MP |  W |  D |  L |  P".to_owned();
    let rows = teams.iter().map(|(name, stats)| {
        format!(
            "{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
            name, stats.played, stats.won, stats.drawn, stats.lost, stats.points
        )
    });
    once(header).chain(rows).collect::<Vec<_>>().join("\n")
}
