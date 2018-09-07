extern crate rand;

mod utilities;

use utilities::modular_pow;
use rand::prelude::*;

pub fn private_key(p: u64) -> u64 {
    thread_rng().gen_range(2, p)
}

pub fn public_key(p: u64, g: u64, private_key: u64) -> u64 {
    modular_pow(g, private_key, p)
}

pub fn secret(p: u64, public_key: u64, private_key: u64) -> u64 {
    modular_pow(public_key, private_key, p)
}
