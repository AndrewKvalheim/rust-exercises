const FRAMES: u16 = 10;
const PINS: u16 = 10;

#[derive(Debug, PartialEq)]
pub enum Error {
    GameComplete,
    NotEnoughPinsLeft,
}

pub struct BowlingGame {
    bonus: u16,
    next_bonus: u16,
    previous_pinfall: Option<u16>,
    regular_balls_due: u16,
    score: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            bonus: 0,
            next_bonus: 0,
            previous_pinfall: None,
            regular_balls_due: FRAMES * 2,
            score: 0,
        }
    }

    pub fn roll(&mut self, pinfall: u16) -> Result<(), Error> {
        if self.is_complete() {
            Err(Error::GameComplete)
        } else if self.previous_pinfall.unwrap_or(0) + pinfall > PINS {
            Err(Error::NotEnoughPinsLeft)
        } else {
            self.spend_bonus(pinfall);

            if self.regular_balls_due > 0 {
                self.score += pinfall;
                self.regular_balls_due -= 1;

                if pinfall == PINS {
                    self.strike()
                /* Pending eRFC 2497 */
                } else if let Some(previous_pinfall) = self.previous_pinfall {
                    if previous_pinfall + pinfall == PINS {
                        self.spare()
                    }
                }
            }

            self.step_half_frame(pinfall);

            Ok(())
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.is_complete() {
            Some(self.score)
        } else {
            None
        }
    }

    fn abort_frame(&mut self) {
        self.previous_pinfall = None;
        self.regular_balls_due -= 1;
    }

    fn is_complete(&self) -> bool {
        self.regular_balls_due == 0 && self.bonus == 0 && self.next_bonus == 0
    }

    fn spare(&mut self) {
        self.bonus += 1;
    }

    fn spend_bonus(&mut self, pinfall: u16) {
        self.score += pinfall * self.bonus;

        self.bonus = self.next_bonus;
        self.next_bonus = 0;
    }

    fn step_half_frame(&mut self, pinfall: u16) {
        self.previous_pinfall = if pinfall == PINS || self.previous_pinfall.is_some() {
            None
        } else {
            Some(pinfall)
        };
    }

    fn strike(&mut self) {
        self.bonus += 1;
        self.next_bonus += 1;

        self.abort_frame();
    }
}
