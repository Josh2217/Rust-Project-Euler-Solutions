#![allow(non_snake_case, dead_code)]

mod jotch;
mod p001;
mod p002;
mod p003;
mod p004;
mod p005;

fn main() {
    //Find the sum of all the multiples of 3 or 5 below 1000.
    p001::run(1000_i32);
    //By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.
    p002::run(4_000_000_i32);
    //What is the largest prime factor of the number 600851475143
    p003::run(600_851_475_143_i64);
    //Find the largest palindrome made from the product of two 3-digit numbers.
    p004::run();
    //What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
    p005::run(1, 20);
}
