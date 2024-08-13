// 24267번, 알고리즘 수업 - 알고리즘의 수행 시간 6

use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<u64>().unwrap();
    // 500_000^3 =    125_000_000_000_000_000
    // 2^64      = 18_446_744_073_709_551_616
    
    // O(n) = n(n-1)(n-2) / 3*2*1
    println!("{}", n*(n-1)*(n-2) / 6);
    println!("3");
}