use std::time;

fn main() {
    // #3 Largest Prime Factor - Project Euler
    // https://projecteuler.net/problem=3
    // https://projecteuler.net/overview=0003
    // The prime factors of 13195 are 5, 7, 13 and 29.
    // What is the largest prime factor of the number 600851475143 ?
    let now = time::Instant::now();
    println!(
        "pe_0003 ans: {} - {:?}",
        pe_0003(600_851_475_143),
        now.elapsed()
    );
}

fn pe_0003(n: u64) -> u64 {
    let mut devidend = n;
    let mut divisor = 2;
    let mut lpf = 0;
    while devidend > 1 {
        if devidend % divisor == 0 {
            lpf = divisor;
            devidend /= divisor;
        } else {
            divisor += 1
        }
    }
    lpf
}
