// 3003번, 킹, 퀸, 룩, 비숍, 나이트, 폰

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: Vec<i32> = input
        .split_whitespace()
        .map(|i| i.trim().parse::<i32>().unwrap())
        .collect();
    let chess: Vec<i32> = vec![1, 1, 2, 2, 2, 8];
    
    for i in 0..6 {
        let output = chess[i] - input[i];
        print!("{} ", output);
    }
}