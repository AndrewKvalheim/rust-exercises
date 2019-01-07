const OPENINGS: [char; 3] = ['(', '[', '{'];
const CLOSINGS: [char; 3] = [')', ']', '}'];

pub fn brackets_are_balanced(text: &str) -> bool {
    text.chars()
        .try_fold(Vec::new(), |mut due, character| {
            if let Some(&closing) = CLOSINGS.iter().find(|&&c| c == character) {
                if due.pop() == Some(closing) {
                    Some(due)
                } else {
                    None
                }
            } else {
                if let Some(index) = OPENINGS.iter().position(|&o| o == character) {
                    due.push(CLOSINGS[index]);
                }

                Some(due)
            }
        })
        .map_or(false, |due| due.is_empty())
}
