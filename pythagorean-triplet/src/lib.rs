use std::collections::HashSet;

pub fn find(perimeter: u32) -> HashSet<[u32; 3]> {
    (1..perimeter / 3)
        .map(|a| {
            let b = (perimeter.pow(2) / 2 - perimeter * a) / (perimeter - a);
            let c = perimeter - (a + b);

            [a, b, c]
        })
        .filter(|[a, b, c]| a <= b && a.pow(2) + b.pow(2) == c.pow(2))
        .collect()
}
