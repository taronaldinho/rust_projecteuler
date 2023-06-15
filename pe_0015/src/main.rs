use std::time;

fn main() {
    // #15 Lattice Paths - Project Euler
    // https://projecteuler.net/problem=15
    // https://projecteuler.net/overview=0015
    // Starting in the top left corner of a
    // grid, and only being able to move to the right and down, there are exactly
    // routes to the bottom right corner.
    // How many such routes are there through a 20 x 20 grid?
    let now = time::Instant::now();
    println!("pe_0015 ans: {} - {:?}", pe_0015(20), now.elapsed())
}

fn pe_0015(num: u64) -> u64 {
    get_num_combinations(2 * num, num)
}

fn get_num_combinations(n: u64, r: u64) -> u64 {
    if r > n {
        return 0;
    }

    let r = r.min(n - r); // 計算の回数を減らし、オーバーフローを防ぐ。
    let mut result: u64 = 1;

    for i in 0..r {
        // 分子は上から、分母は1からとすることで毎ループの除算結果は必ず整数になる。
        result *= n - i;
        result /= i + 1;
    }

    result
}
