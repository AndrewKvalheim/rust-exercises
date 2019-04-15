pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        luhn::is_valid(&self.0)
    }
}

impl<T: ToString> From<T> for Luhn {
    fn from(number: T) -> Self {
        Self(number.to_string())
    }
}
