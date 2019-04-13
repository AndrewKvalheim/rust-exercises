pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(height: u32) -> Self {
        Self(height)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (0..self.0).map(Self::row).collect()
    }

    fn row(level: u32) -> Vec<u32> {
        (0..=level)
            .scan(None, |value, index| {
                value.replace(value.map_or(1, |v| v * (1 + level - index) / index));

                *value
            })
            .collect()
    }
}
