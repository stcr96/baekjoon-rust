// 24264번, 알고리즘 수업 - 알고리즘의 수행 시간 3

use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<u64>().unwrap();
    
    println!("{}", n*n);
    println!("2");
}