pub struct RailFence(usize);

impl RailFence {
    pub fn new(width: u32) -> Self {
        RailFence(width as usize - 1)
    }

    pub fn encode(&self, plaintext: &str) -> String {
        let length = plaintext.chars().count();

        plaintext
            .chars()
            .zip(self.rail_indexes())
            .fold(self.new_rails(length), |mut rails, (character, index)| {
                rails[index].push(character);

                rails
            })
            .iter()
            .flatten()
            .collect()
    }

    pub fn decode(&self, ciphertext: &str) -> String {
        let length = ciphertext.chars().count();

        ciphertext
            .chars()
            .zip(self.plaintext_indexes(length))
            .fold(
                vec![char::default(); length],
                |mut plaintext, (character, index)| {
                    plaintext[index] = character;

                    plaintext
                },
            )
            .iter()
            .collect()
    }

    fn new_rails(&self, text_length: usize) -> Vec<Vec<char>> {
        (0..=self.0)
            .map(|index| index == 0 || index == self.0)
            .map(|is_edge| if is_edge { 1 } else { 2 } * text_length / (1 + self.0))
            .map(Vec::with_capacity)
            .collect()
    }

    fn plaintext_indexes(&self, length: usize) -> impl Iterator<Item = usize> + '_ {
        let get_index = move |rail, longitude| match (rail, self.0 - rail) {
            (a, 0) | (0, a) => Some(rail + a * longitude * 2),
            (a, b) => Some(rail + longitude / 2 * 2 * a + (longitude + 1) / 2 * 2 * b),
        };

        (0..).scan((0, 0), move |(rail, longitude), _| {
            let index = get_index(*rail, *longitude)
                .filter(|&i| i < length)
                .or_else(|| {
                    *rail += 1;
                    *longitude = 0;

                    get_index(*rail, *longitude)
                });

            *longitude += 1;

            index
        })
    }

    fn rail_indexes(&self) -> impl Iterator<Item = usize> + '_ {
        let width = self.0 as isize;

        (0..).map(move |i| ((i + width) % (width * 2) - width).abs() as usize)
    }
}
