use crate::duration::Duration;

pub trait Planet {
    const YEAR: Duration;

    fn years_during(duration: &Duration) -> f64 {
        *duration / Self::YEAR
    }
}

#[macro_export]
macro_rules! planet {
    ($name: ident, $year: expr) => {
        pub struct $name;

        impl Planet for $name {
            const YEAR: Duration = Duration::from_seconds($year);
        }
    };
}
