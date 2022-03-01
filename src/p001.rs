//Find the sum of all the multiples of 3 or 5 below 1000.
pub fn run(n: i32) {
    let mut sum = 0;
    for i in 0..n {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    println!("Solution 1: {}", sum);
}