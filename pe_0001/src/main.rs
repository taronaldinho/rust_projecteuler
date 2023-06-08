use std::time;

fn main() {
    // #1 Multiples of $3$ or $5$ - Project Euler
    // https://projecteuler.net/problem=1
    // https://projecteuler.net/overview=0001
    // If we list all the natural numbers below 10 that are multiples of 3 or 5,
    // we get 3, 5, 6 and 9. The sum of these multiples is 23.
    // Find the sum of all the multiples of 3 or 5 below 1000.
    let now = time::Instant::now();
    println!("pe_0001 ans: {} - {:?}", pe_0001(1_000), now.elapsed());
}

fn pe_0001(n: u32) -> u32 {
    let mut sum = 0;
    for n in 0..n {
        if n % 3 == 0 || n % 5 == 0 {
            sum += n
        }
    }
    sum
}
