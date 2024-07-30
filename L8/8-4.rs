// 2903번, 중앙 이동 알고리즘

use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: u32 = n.trim().parse().unwrap();
    
    let mut dot_line: u32 = 2;
    for _ in 0..n {
        dot_line = dot_line*2 - 1;
    }
    let output: u32 = dot_line*dot_line;
    println!("{output}");
}