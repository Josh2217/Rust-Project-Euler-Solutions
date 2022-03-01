//By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.

pub fn run(n: i32) {
    let mut a: i32;
    let mut b: i32 = 1;
    let mut c: i32 = 1;
    let mut sum: i32 = 0;

    while c < n {
        a = b;
        b = c;
        c = a + b;
        //my favorite way to test parity
        if c & 1 == 0 {
            sum += c;
        }
    }
    println!("Solution 2: {}", sum);
}