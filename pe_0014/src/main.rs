use std::time;

fn main() {
    // #14 Longest Collatz Sequence - Project Euler
    // https://projecteuler.net/problem=14
    // The following iterative sequence is defined for the set of positive integers:

    // n -> n/2 (n is even)
    // (n -> 3n+1 (n is odd)
    // Using the rule above and starting with 13, we generate the following sequence:
    // 13 -> 40 -> 20 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1.
    // It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms.
    // Although it has not been proved yet (Collatz Problem),
    // it is thought that all starting numbers finish at 1.

    // Which starting number, under one million, produces the longest chain?
    // NOTE: Once the chain starts the terms are allowed to go above one million.
    let now = time::Instant::now();
    println!("pe_0014 ans: {} - {:?}", pe_0014(num), now.elapsed());
}

fn pe_0014(num: u32) -> String {
    let mut collatz_seq:Vec<u32> = vec![1];
    let mut n = 1;
    let target_n: u32;
    let len: u32 = 1;

    while n <= num {
        let target_n = n;
        if collatz_seq.contains(&n) {
            continue;
            n += 1;
        } else {

        }
    }

}

fn get_collatz_seq
