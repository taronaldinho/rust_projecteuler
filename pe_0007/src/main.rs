use std::time;

fn main() {
    // #7 $10001$st Prime - Project Euler
    // https://projecteuler.net/problem=7
    // https://projecteuler.net/overview=0007
    // By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13,
    // we can see that the 6th prime is 13.
    // What is the 10001st prime number?
    let now = time::Instant::now();
    println!("pe_0001 ans: {} - {:?}", pe_0007(10_001), now.elapsed());
}

fn pe_0007(n: u64) -> u64 {
    let mut z = 1;
    let mut c = 0;
    while c <= n {
        if is_pn(z) {
            c += 1;
            if c == n {
                break;
            };
        };
        z += 1;
    }
    z
}

fn is_pn(n: u64) -> bool {
    // 与えられた引数の整数が、素数かどうか判定する関数。
    let mut ret = true;
    if n <= 1 {
        // n = 1 ⇒ 素数ではない。
        ret = false;
    } else if n == 2 {
        // n = 2 ⇒ 素数。
        ret = true;
    } else if n % 2 == 0 {
        // nが偶数 ⇒ 素数ではない。
        ret = false;
    } else {
        let n_ = n as f64; // 浮動小数点型にキャストしておく。
        let lim = n_.sqrt() as u64; // 浮動小数点を整数にキャストすると小数部分は切り捨てられる。

        // iについて3からlimまでの奇数の範囲で繰り返し処理を行います。
        for i in (3..=lim).filter(|&x| x % 2 != 0) {
            if n % i == 0 {
                ret = false;
                break;
            };
        }
    };
    // println!("{}: {}", n, ret);
    ret
}
