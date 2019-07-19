use std::ops::Div;

#[derive(Clone, Copy)]
pub struct Duration(u64);

impl Duration {
    pub const fn from_seconds(seconds: u64) -> Self {
        Self(seconds)
    }
}

impl Div for Duration {
    type Output = f64;

    fn div(self, denominator: Self) -> Self::Output {
        self.0 as f64 / denominator.0 as f64
    }
}

impl From<u64> for Duration {
    fn from(seconds: u64) -> Self {
        Self::from_seconds(seconds)
    }
}
