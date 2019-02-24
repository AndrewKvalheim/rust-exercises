// Pending rayon-rs/rayon#572
#![allow(clippy::range_plus_one)]

extern crate rayon;

use rayon::prelude::*;

pub type Palindrome = u64;

pub fn get_palindrome_products(start: u64, end: u64) -> Vec<Palindrome> {
    (start..(1 + end))
        .into_par_iter()
        .flat_map(|x| (x..(1 + end)).into_par_iter().map(move |y| x * y))
        .filter(|&product| is_palindrome(product))
        .collect()
}

pub fn max(palindromes: &[Palindrome]) -> Option<Palindrome> {
    palindromes.iter().max().cloned()
}

pub fn min(palindromes: &[Palindrome]) -> Option<Palindrome> {
    palindromes.iter().min().cloned()
}

fn is_palindrome(n: u64) -> bool {
    fn recurse(n: u64, length: u32) -> bool {
        length <= 1 || {
            let scale = 10u64.pow(length - 1);

            n / scale == n % 10 && recurse(n % scale / 10, length - 2)
        }
    }

    recurse(n, 1 + (n as f64).log10().floor() as u32)
}
