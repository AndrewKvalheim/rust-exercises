pub struct Stats {
    pub drawn: u32,
    pub lost: u32,
    pub played: u32,
    pub points: u32,
    pub won: u32,
}

impl Stats {
    pub fn default() -> Self {
        Self {
            drawn: 0,
            lost: 0,
            played: 0,
            points: 0,
            won: 0,
        }
    }

    pub fn draw(&mut self) {
        self.drawn += 1;
        self.played += 1;
        self.points += 1;
    }

    pub fn loss(&mut self) {
        self.lost += 1;
        self.played += 1;
    }

    pub fn win(&mut self) {
        self.played += 1;
        self.points += 3;
        self.won += 1;
    }
}
