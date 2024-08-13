// 15894번, 수학은 체육과목 입니다

use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<u64>().unwrap();
    
    println!("{}", 4*n);
}