pub fn hamming_distance(sequence_1: &str, sequence_2: &str) -> Option<usize> {
    Some((sequence_1, sequence_2))
        .filter(|(s1, s2)| s1.len() == s2.len())
        .map(|(s1, s2)| s1.chars().zip(s2.chars()).filter(|(a, b)| a != b).count())
}
