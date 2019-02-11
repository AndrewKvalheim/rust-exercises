mod allergen;

pub use allergen::Allergen;

pub struct Allergies(u32);

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self(score)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::ALL
            .iter()
            .filter(|&a| self.is_allergic_to(a))
            .cloned()
            .collect()
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0 & *allergen as u32 != 0
    }
}
