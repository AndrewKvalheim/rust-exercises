const OPENINGS: [char; 3] = ['(', '[', '{'];
const CLOSINGS: [char; 3] = [')', ']', '}'];

pub fn brackets_are_balanced(text: &str) -> bool {
    text.chars()
        .try_fold(Vec::new(), |mut due, character| {
            if CLOSINGS.contains(&character) {
                due.pop().filter(|&d| d == character).map(|_| due)
            } else {
                if let Some(index) = OPENINGS.iter().position(|&o| o == character) {
                    due.push(CLOSINGS[index]);
                }

                Some(due)
            }
        })
        .map_or(false, |due| due.is_empty())
}
