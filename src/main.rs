#![allow(non_snake_case, dead_code)]

mod jotch;
mod p001;
mod p002;
mod p003;

fn main() {
    //Find the sum of all the multiples of 3 or 5 below 1000.
    p001::run(1000_i32);
    //By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.
    p002::run(4_000_000_i32);
    //What is the largest prime factor of the number 600851475143
    p003::run(600_851_475_143_i64);
}
