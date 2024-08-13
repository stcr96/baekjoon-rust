// 24263번, 알고리즘 수업 - 알고리즘의 수행 시간 2

use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<u32>().unwrap();
    
    println!("{}", n);
    println!("1");
}