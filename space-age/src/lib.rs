mod duration;
mod planet;

pub use duration::Duration;
pub use planet::Planet;

planet!(Earth, 31_557_600);
planet!(Jupiter, 374_355_659);
planet!(Mars, 59_354_032);
planet!(Mercury, 7_600_543);
planet!(Neptune, 5_200_418_560);
planet!(Saturn, 929_292_362);
planet!(Uranus, 2_651_370_019);
planet!(Venus, 19_414_149);
