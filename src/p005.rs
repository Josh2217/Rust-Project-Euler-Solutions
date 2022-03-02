//What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
use crate::jotch;
extern crate intmap;
use intmap::IntMap;
use std::time::Instant;


//this is very verbose, but also EXTREMELY fast for some reason
pub fn run(a: i32, b: i32) {
    let now = Instant::now();
    let mut vec2d: Vec<Vec<i32>> = vec![];
    for i in a..(b + 1) {
        vec2d.push(jotch::get_prime_factors(i));
    }
    let mut maxes = IntMap::<i32>::new();
    for v in vec2d {
        let mut map = IntMap::<i32>::new();
        for i in v {
            match map.get_mut(i as u64){
                Some(num) => *num += 1,
                None => {map.insert(i as u64, 1);},
            }
        }
        for i in map.keys() {       
            let val = map.get(*i).unwrap_or(&0_i32);
            match maxes.get_mut(*i) {
                Some(num) => {
                    if *num < *val {
                        *num = *val;
                    }
                },
                None => {maxes.insert(*i, *val);},
            }
        }
    }
    let mut product: i32 = 1;
    for i in maxes.keys() {
        product *= jotch::pow(*i as i32 , *maxes.get(*i).unwrap_or(&0_i32));
    }
    println!("Solution   5: {product} in {:.3?}", now.elapsed());
}

