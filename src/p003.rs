//What is the largest prime factor of the number 600851475143
use crate::jotch;

pub fn run(n: i64) {
    
    let mut max: i64 = 1;
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            if jotch::iter_is_prime(i) {
                max = jotch::max(max, i);
            }
            if jotch::iter_is_prime(n / i) {
                max = jotch::max(max, n / i);
            }
        }
        i += 2;
    }
    println!("Solution 3: {}", max);
}

