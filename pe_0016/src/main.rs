use std::time;

fn main() {
    // #16 Power Digit Sum - Project Euler
    // https://projecteuler.net/problem=16
    // 2^15 = 32768 and; the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
    // What is the sum of the digits of the number 2^1000?
    let now = time::Instant::now();
    println!("pe_0016 ans: {} - {:?}", pe_0016(15), now.elapsed())
}

// std::num::po
fn pe_0016(num: u32) -> u32 {
    let num: String = (2_u64.pow(num)).to_string();
    println!("{}", num);

    let mut ret: u32 = 0;
    for n in num.chars() {
        ret += n.to_digit(10).unwrap();
    }
    ret
}
