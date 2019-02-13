use std::iter::once;

pub fn number(input: &str) -> Option<String> {
    input
        .chars()
        .filter(|c| c.is_numeric())
        .map(Some)
        .chain(once(None))
        .scan(0, |position, next| {
            *position += if *position == 0 && next != Some('1') { 2 } else { 1 };

            match (next, *position) {
                (Some('1'), 1) => Some(None), /* Omit */
                (Some('0'...'1'), _) if [2, 5].contains(position) => Some(Some(None)), /* Fail */
                (_, 12) => next.map(|_| Some(None)), /* Stop or fail */
                _ => Some(Some(next)), /* Emit */
            }
        })
        .flatten()
        .collect()
}
