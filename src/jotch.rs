
pub fn iter_is_prime(n: i64) -> bool {
    if (n & 1) == 0 {
        false
    }
    else{
        let mut i = 3;
        while i * i <= n {
            if n % i == 0 {
                return false;
            }
            i += 2;
        }
        true
    }
}

pub fn max(a: i64, b: i64) -> i64 {
    if a > b {
        a
    }
    else {
        b
    }
}