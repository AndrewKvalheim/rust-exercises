use anagram;
use std::collections::HashSet;

#[test]
fn test_unicode_case_folding() {
    let word = "ΔΣ";
    let candidates = ["ΔΣ", "ΣΔ", "δσ", "σδ", "δς", "ςδ"];
    let expected = ["ΣΔ", "σδ", "ςδ"];

    assert_eq!(
        expected.iter().cloned().collect::<HashSet<_>>(),
        anagram::anagrams_for(&word, &candidates)
    );
}
