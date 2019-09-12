mod iter_moves;
mod specification;

use iter_moves::IterMoves;
pub use specification::*;
use Bucket::*;

pub fn solve(capacity_1: u8, capacity_2: u8, goal: u8, start: &Bucket) -> BucketStats {
    let (names, capacities) = match start {
        One => ((One, Two), (capacity_1, capacity_2)),
        Two => ((Two, One), (capacity_2, capacity_1)),
    };

    IterMoves::new(goal, capacities)
        .enumerate()
        .find_map(|(index, amounts)| {
            // Pending RFC 2757
            if amounts.0 == goal || amounts.1 == goal {
                Some(BucketStats::new(names, index, amounts, amounts.0 == goal))
            } else {
                None
            }
        })
        .unwrap()
}
