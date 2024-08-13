// 24265번, 알고리즘 수업 - 알고리즘의 수행 시간 4

use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<f64>().unwrap();
    
    // O(n) = (n^2 - n) / 2
    println!("{}", (n*n - n) / 2 as f64);
    println!("2");
}