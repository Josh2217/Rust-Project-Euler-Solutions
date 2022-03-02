//Find the largest palindrome made from the product of two 3-digit numbers.
use crate::jotch;
use std::time::Instant;

pub fn run() {
    let now = Instant::now();

    let mut max = 0;
    for first in 100..1000 {
        for second in first..1000 {
            let num: i64 = first * second;
            if num.to_string() == num.to_string().chars().rev().collect::<String>() {
                max = jotch::max(max, num);
            }
        }
    }
    println!("Solution   4: {max} in {:.3?}", now.elapsed());
}