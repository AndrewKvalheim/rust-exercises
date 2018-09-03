#[cfg(feature = "grapheme")]
extern crate grapheme;

#[cfg(feature = "grapheme")]
use grapheme::UnicodeSegmentation;

pub fn reverse(text: &str) -> String {
    #[cfg(feature = "grapheme")]
    let parts = UnicodeSegmentation::graphemes(text, true);
    #[cfg(not(feature = "grapheme"))]
    let parts = text.chars();

    parts.rev().collect()
}
