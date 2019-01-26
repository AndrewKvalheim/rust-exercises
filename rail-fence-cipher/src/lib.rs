pub struct RailFence(usize);

impl RailFence {
    pub fn new(width: u32) -> Self {
        RailFence(width as usize - 1)
    }

    pub fn encode(&self, plaintext: &str) -> String {
        plaintext
            .chars()
            .zip(self.rail_indexes())
            .fold(
                self.new_rails(plaintext.len()),
                |mut rails, (character, index)| {
                    rails[index].push(character);

                    rails
                },
            )
            .iter()
            .flatten()
            .collect()
    }

    pub fn decode(&self, ciphertext: &str) -> String {
        ciphertext
            .chars()
            .zip(self.plaintext_indexes(ciphertext.len()))
            .fold(
                vec!['🥑'; ciphertext.len()],
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
        let get_position = move |rail_index, position_index| {
            rail_index
                + 2 * match (rail_index, self.0 - rail_index) {
                    (a, 0) => a * position_index,
                    (0, b) => b * position_index,
                    (a, b) => (position_index / 2) * a + ((position_index + 1) / 2) * b,
                }
        };

        (0..length).scan((0, 0), move |(rail_index, position_index), _| {
            let mut position = get_position(*rail_index, *position_index);

            if position >= length {
                *rail_index += 1;
                *position_index = 0;

                position = get_position(*rail_index, *position_index);
            };

            *position_index += 1;

            Some(position)
        })
    }

    fn rail_indexes(&self) -> impl Iterator<Item = usize> + '_ {
        (0..).map(move |index| {
            (((index + self.0) % (self.0 * 2)) as isize - self.0 as isize).abs() as usize
        })
    }
}
