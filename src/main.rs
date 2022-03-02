#![allow(non_snake_case, dead_code)]

mod jotch;
mod problems;

fn main() {
    //Find the sum of all the multiples of 3 or 5 below 1000.
    problems::p001::solve(1_000);
    //By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.
    problems::p002::solve(4_000_000);
    //What is the largest prime factor of the number 600851475143
    problems::p003::solve(600_851_475_143);
    //Find the largest palindrome made from the product of two 3-digit numbers.
    problems::p004::solve();
    //What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
    problems::p005::solve(1, 20);
    //Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
    problems::p006::solve(1, 100);
    //What is the 10 001st prime number?
    problems::p007::solve(10_001);
}
