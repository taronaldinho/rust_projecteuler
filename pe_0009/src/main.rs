use std::time;

fn main() {
    // #9 Special Pythagorean Triplet - Project Euler
    // https://projecteuler.net/problem=9
    // https://projecteuler.net/overview=0009
    // A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
    // a^2 + b^2 = c^2
    // For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
    // There exists exactly one Pythagorean triplet for which a + b + c = 1000.
    // Find the product abc.
    let now = time::Instant::now();
    println!("pe_0009 ans: {} - {:?}", pe_0009(1_000), now.elapsed());
}

fn pe_0009(num: u64) -> u64 {
    let mut ans = 0;
    for a in 1..=num {
        for b in (a + 1)..=(num - a) {
            let c = num - a - b;
            if a.pow(2) + b.pow(2) == c.pow(2) {
                ans = a * b * c;
                break;
            };
        }
    }
    ans
}
