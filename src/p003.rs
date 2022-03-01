//What is the largest prime factor of the number 600851475143
use crate::jotch;

pub fn run(n: i64) {
    let mut factors: Vec<i64> = Vec::<i64>::new();

    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            factors.push(i);
            factors.push(n / i);
        }
        i += 2;
    }
    let mut i = 0;
    while i < factors.len(){
        if !jotch::iter_is_prime(factors[i]) {
            factors.remove(i);
            continue;
        }

        i += 1;
    }
    factors.sort();
    println!("Solution 3: {}", factors.pop().unwrap());
}

