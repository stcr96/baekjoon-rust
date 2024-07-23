// 2566번, 최댓값

use std::io;

fn main() {
    // 입력
    let mut input = String::new();
    for _ in 0..9 {
        io::stdin().read_line(&mut input).unwrap();
    }
    let input: Vec<u32> = input
        .split_whitespace()
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect();
    
    let max = input.iter().max().unwrap();
    println!("{}", max);
    // println!("{:?}", input);
    let position = input.iter().position(|v| v == max).unwrap();
    println!("{} {}", position/9+1, position-(position/9)*9+1);
    // println!("{:?}", matrix.iter().flatten().position(|m| m == max));
}