use std::time;

fn main() {
    // #5 Smallest Multiple - Project Euler
    // https://projecteuler.net/problem=5
    // https://projecteuler.net/overview=0005
    // 2520 is the smallest number that can be divided by each of the numbers from 1 to 10
    // without any remainder.
    // What is the smallest positive number that is evenly divisible by
    // all of the numbers from 1 to 20?
    let now = time::Instant::now();
    println!("pe_0005 ans: {} - {:?}", pe_0005(), now.elapsed());
}

use std::collections::HashMap;

fn pe_0005() -> u64 {
    // 1から20以下の整数で割り切れる
    // ⇒ 1から20以下のそれぞれの整数の素因数の集合を調べて、その積が答えとなる。
    let mut map = HashMap::new();
    let mut n = 2;
    while n <= 20 {
        let mut devidend = n; // 割られる数 nからスタート
        let mut divisor: u64 = 2; // 割る数 2からスタート
        let mut c = 0;

        //
        while devidend > 1 {
            if devidend % divisor == 0 {
                // 割り切れたら、key: 割る数、val: その因数で割れた回数 をハッシュマップに書き込む。
                // ただし割れた回数は、それぞれのnについて最大のものだけ保持する。
                c += 1;
                let count = map.entry(divisor).or_insert(0);
                if c > *count {
                    *count = c;
                }
                devidend /= divisor;
            } else {
                // 割り切れなかったら割る数をカウントアップする。割れた回数も初期化。
                c = 0;
                divisor += 1
            }
        }
        n += 1;
    }

    // ハッシュマップのすべてのkey^valの値の積が答えとなる。
    let mut ret = 1;
    for (k, v) in map {
        // println!("{}: {}", k, v);
        ret *= k.pow(v);
    }
    ret
}
