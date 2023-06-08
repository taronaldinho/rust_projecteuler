use std::time;

fn main() {
    // #12 Highly Divisible Triangular Number - Project Euler
    // https://projecteuler.net/problem=12
    // https://projecteuler.net/overview=0012
    // The sequence of triangle numbers is generated by adding the natural numbers.
    // So the 7th triangle number would be
    // 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28.
    // The first ten terms would be:
    // 1, 3, 6, 10, 15, 21, 28, 36, 45, 55,…
    // Let us list the factors of the first seven triangle numbers:

    //  1: 1
    //  3: 1,3
    //  6: 1,2,3,6
    // 10: 1,2,5,10
    // 15: 1,3,5,15
    // 21: 1,3,7,21
    // 28: 1,2,4,7,14,28
    // We can see that
    //  is the first triangle number to have over five divisors.

    // What is the value of the first triangle number to have over five hundred divisors?
    let now = time::Instant::now();
    println!("pe_0012 ans: {} - {:?}", pe_0012(500), now.elapsed());
}

fn pe_0012(threshold: u32) -> u32 {
    let mut triangular_num: u32;
    let mut i: u32 = 2;
    loop {
        triangular_num = generate_nth_triangular_term(i);

        if get_num_divisors(triangular_num) > threshold {
            break;
        } else {
            i += 1;
        }
    }
    triangular_num
}

fn generate_nth_triangular_term(n: u32) -> u32 {
    (1..=n).sum()
}

fn get_num_divisors(n: u32) -> u32 {
    let mut num: u32 = 1;

    if n != 1 {
        num += 1;
    }

    for i in 2..=(n / 2) {
        // 整数の除算は整数を返す。
        if n % i == 0 {
            num += 1;
        }
    }
    num
}