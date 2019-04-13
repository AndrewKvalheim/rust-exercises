// https://en.wikipedia.org/wiki/Modular_exponentiation#Right-to-left_binary_method
pub fn modular_pow(base: u64, exponent: u64, modulus: u64) -> u64 {
    let mut cycle_base = base % modulus;
    let mut cycle_exponent = exponent;
    let mut result = 1;

    while cycle_exponent > 0 {
        if cycle_exponent % 2 == 1 {
            result = (result * cycle_base) % modulus;
        }
        cycle_base = cycle_base.pow(2) % modulus;
        cycle_exponent /= 2;
    }

    result
}
