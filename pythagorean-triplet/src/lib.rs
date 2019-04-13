const PERIMETER: u32 = 1000;

pub fn find() -> Option<u32> {
    (1..=PERIMETER / 2)
        .map(|a| {
            let b = (PERIMETER.pow(2) / 2 - PERIMETER * a) / (PERIMETER - a);
            let c = PERIMETER - (a + b);

            (a, b, c)
        })
        .find(|(a, b, c)| a.pow(2) + b.pow(2) == c.pow(2))
        .map(|(a, b, c)| a * b * c)
}
