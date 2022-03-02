//What is the 10 001st prime number?
use crate::jotch;
use std::time::Instant;

pub fn solve(n: i64) {
    let now = Instant::now();

    println!("Solution   7: {} in {:.3?}", jotch::get_n_primes(n)[(n - 1) as usize], now.elapsed());
}