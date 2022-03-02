pub fn get_prime_factors(mut n: i64) -> Vec<i64> {
    let mut i: i64 = 2;
    let mut ans: Vec<i64> = vec![];
    while i <= n {
        if !iter_is_prime(i){
            i += 1;
            continue;
        }
        if n % i == 0 {
            n /= i;
            ans.push(i);
            continue; 
        }
        i += 1;
    }

    return ans;
}

pub fn pow(a: i64, b: i64) -> i64{
    let mut prod = 1;
    if b < 0 {
        1 / pow(a, -b)
    }
    else {
        for _ in 0..b {
            prod *= a;
        }
        prod
    }
}

pub fn iter_is_prime(n: i64) -> bool {
    if n == 2 {
        true
    }
    else if n & 1 == 0 {
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

pub fn max<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    }
    else {
        b
    }
}