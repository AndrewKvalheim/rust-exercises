pub fn build_proverb(items: &[&str]) -> String {
    items
        .windows(2)
        .map(|pair| match pair {
            [wanted, lost] => format!("For want of a {} the {} was lost.", wanted, lost),
            _ => unreachable!(),
        })
        .chain(
            items
                .iter()
                .take(1)
                .map(|wanted| format!("And all for the want of a {}.", wanted)),
        )
        .collect::<Vec<_>>()
        .join("\n")
}
