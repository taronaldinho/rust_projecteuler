use std::time;

fn main() {
    // #10 Summation of Primes - Project Euler
    // https://projecteuler.net/problem=10
    // https://projecteuler.net/overview=0010
    // The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
    // Find the sum of all the primes below two million.
    let now = time::Instant::now();
    println!("pe_0010 ans: {} - {:?}", pe_0010(2_000_000), now.elapsed());
}

fn pe_0010(num: usize) -> usize {
    get_pe_list(num).iter().sum()
}

fn get_pe_list(upper_limit: usize) -> Vec<usize> {
    // エラトステネスの篩 - Wikipedia
    // https://ja.wikipedia.org/wiki/%E3%82%A8%E3%83%A9%E3%83%88%E3%82%B9%E3%83%86%E3%83%8D%E3%82%B9%E3%81%AE%E7%AF%A9

    let mut primes = vec![true; upper_limit + 1];
    primes[0] = false; // 0は素数ではない。
    primes[1] = false; // 1は素数ではない。

    let mut p = 2;
    while p * p <= upper_limit {
        if primes[p] {
            let mut i = p * p;
            while i <= upper_limit {
                primes[i] = false;
                i += p;
            }
        }
        p += 1;
    }

    let mut result = Vec::new();
    for (num, is_prime) in primes.iter().enumerate() {
        if *is_prime {
            result.push(num);
        }
    }

    result
}
