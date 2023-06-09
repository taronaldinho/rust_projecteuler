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
    // We can see that is the first triangle number to have over five divisors.

    // What is the value of the first triangle number to have over five hundred divisors?
    let now = time::Instant::now();
    println!("pe_0012 ans: {} - {:?}", pe_0012(500), now.elapsed());
}

// この実装では35秒程度

fn pe_0012(threshold: u64) -> u64 {
    let mut triangular_num: u64;
    let mut i: u64 = 2;
    let ul: u64 = 10000000;
    let pn_list = get_pn_list(ul);
    // let max_pn = pn_list[pn_list.len() - 1];

    loop {
        triangular_num = (1..=i).sum();
        // if 2 * max_pn < triangular_num {
        //     ul *= 10;
        //     pn_list = get_pn_list(ul);
        //     println!("pe_list updated. {}", ul);
        // };
        let num_divisors: u64 = prime_factorization(triangular_num, &pn_list)
            .values()
            .map(|x| x + 1)
            .product();

        if num_divisors > threshold {
            return triangular_num;
        }
        i += 1
    }
}

use std::collections::HashMap;

fn prime_factorization(target_number: u64, pn_list: &Vec<u64>) -> HashMap<u64, u64> {
    let mut pf_result = HashMap::new();

    for p in pn_list {
        let mut devidend = target_number;
        let mut c = 0;
        while devidend % p == 0 {
            devidend /= p;
            c += 1;
        }
        if c > 0 {
            pf_result.insert(*p, c);
        }
        if 2 * p >= target_number {
            break;
        }
    }
    pf_result
}

fn get_pn_list(upper_limit: u64) -> Vec<u64> {
    // エラトステネスの篩 - Wikipedia
    // https://ja.wikipedia.org/wiki/%E3%82%A8%E3%83%A9%E3%83%88%E3%82%B9%E3%83%86%E3%83%8D%E3%82%B9%E3%81%AE%E7%AF%A9
    let upper_limit = upper_limit as usize;
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
            result.push(num as u64);
        }
    }

    result
}

// 以下の実装では10分以上掛かってしまう。
// fn pe_0012(threshold: u32) -> u32 {
//     let mut triangular_num: u32;
//     let mut i: u32 = 2;
//     loop {
//         triangular_num = generate_nth_triangular_term(i);

//         if get_num_divisors(triangular_num) > threshold {
//             break;
//         } else {
//             i += 1;
//         }
//     }
//     triangular_num
// }

// fn generate_nth_triangular_term(n: u32) -> u32 {
//     (1..=n).sum()
// }

// fn get_num_divisors(n: u32) -> u32 {
//     let mut num: u32 = 1;

//     if n != 1 {
//         num += 1;
//     }

//     for i in 2..=(n / 2) {
//         // 整数の除算は整数を返す。
//         if n % i == 0 {
//             num += 1;
//         }
//     }
//     num
// }
