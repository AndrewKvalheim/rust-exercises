pub fn square(n: u32) -> u64 {
    // Pending RFC 1303 or RFC 1434
    if !(1 <= n && n <= 64) {
        panic!("Square must be between 1 and 64");
    }

    2u64.pow(n - 1)
}

pub fn total() -> u64 {
    u64::max_value() // (1..=64).map(square).sum()
}
