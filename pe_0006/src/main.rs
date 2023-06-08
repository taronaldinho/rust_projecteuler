use std::time;

fn main() {
    // #6 Sum Square Difference - Project Euler
    // https://projecteuler.net/problem=6
    // https://projecteuler.net/overview=0006
    // The sum of the squares of the first ten natural numbers is,
    // 1^2+2^2+...+10^2=385
    // The square of the sum of the first ten natural numbers is,
    // (1+2+...+10)^2=55^2=3025
    // Hence the difference between the sum of the squares of the first ten natural numbers and
    // the square of the sum is 3025−385=2640.
    // Find the difference between the sum of the squares of the first one hundred natural numbers
    // and the square of the sum.
    let now = time::Instant::now();
    println!("pe_0006 ans: {} - {:?}", pe_0006(100), now.elapsed());
}

fn pe_0006(n: u64) -> u64 {
    let mut sum_of_nums = 0;
    let mut sum_of_squares = 0;
    for i in 1..=n {
        sum_of_nums += i;
        sum_of_squares += i.pow(2);
    }
    sum_of_nums.pow(2) - sum_of_squares
}
