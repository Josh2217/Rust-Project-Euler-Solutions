//Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
use std::time::Instant;

pub fn solve(a: i64, b: i64) {
    let now = Instant::now();
    let mut sum: i64 = 0;
    let mut sum_of_squares: i64 = 0;
    for i in a..(b + 1){
        sum += i;
        sum_of_squares += i * i;
    }
    println!("Solution   6: {} in {:.3?}", sum * sum - sum_of_squares, now.elapsed());
}