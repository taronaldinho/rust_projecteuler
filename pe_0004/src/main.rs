use std::time;

fn main() {
    // #4 Largest Palindrome Product - Project Euler
    // https://projecteuler.net/problem=4
    // https://projecteuler.net/overview=0004
    // A palindromic number reads the same both ways.
    // The largest palindrome made from the product
    // of two 2-digit numbers is 9009 = 91 × 99.
    // Find the largest palindrome made from the product of two 3-digit numbers.
    let now = time::Instant::now();
    println!("pe_0004 ans: {} - {:?}", pe_0004(), now.elapsed());
}

fn pe_0004() -> u64 {
    let mut n = 999;
    let mut palindromic_number: u64 = 999999;
    while n >= 100 {
        // 6桁の双子数を作るために 3桁の数字を文字列化→逆転→元の文字列と結合→数値に戻す を行う
        let s = n.to_string();
        let r = s.chars().rev().collect::<String>();
        let s = format!("{}{}", s, r);
        palindromic_number = s.parse().unwrap();

        //
        let mut divisor = 999;
        while divisor >= 100 {
            if palindromic_number % divisor == 0 {
                let quatient = palindromic_number / divisor;
                if (quatient >= 100) && (quatient < 1000) {
                    n = 0;
                    break;
                }
            }
            divisor -= 1;
        }
        n -= 1;
    }
    palindromic_number
}
