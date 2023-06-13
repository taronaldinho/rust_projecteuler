use std::time;

fn main() {
    // #14 Longest Collatz Sequence - Project Euler
    // https://projecteuler.net/problem=14
    // https://projecteuler.net/overview=0014
    // The following iterative sequence is defined for the set of positive integers:
    // n -> n/2 (n is even)
    // n -> 3n+1 (n is odd)
    // Using the rule above and starting with 13, we generate the following sequence:
    // 13 -> 40 -> 20 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1.
    // It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms.
    // Although it has not been proved yet (Collatz Problem),
    // it is thought that all starting numbers finish at 1.

    // Which starting number, under one million, produces the longest chain?
    // NOTE: Once the chain starts the terms are allowed to go above one million.
    let now = time::Instant::now();
    println!("pe_0014 ans: {} - {:?}", pe_0014(1_000_000), now.elapsed())
    // println!("pe_0014 ans: {} - {:?}", pe_0014(13), now.elapsed());
}

fn pe_0014(num: u64) -> u64 {
    let mut i: u64 = 2;
    let mut max_chain: usize = 1;
    let mut ret: u64 = i;
    while i <= num {
        let mut target = i;
        let mut chain: Vec<u64> = vec![1];
        while target != 1 {
            // println!("i:{}/{}", i, target);
            chain.push(target);
            if target % 2 == 1 {
                target = 3 * target + 1;
            } else {
                target /= 2;
            }
        }
        if max_chain <= chain.len() {
            max_chain = chain.len();
            ret = i;
        }
        i += 1;
    }
    ret
}
