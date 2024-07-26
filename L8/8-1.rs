// 2745번, 진법 변환

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: Vec<&str> = input.split_whitespace().collect();
    
    let n = input[0];
    let b: u32 = input[1].trim().parse().unwrap();
    
    let output: u32 = u32::from_str_radix(n, b).unwrap();
    println!("{}", output);
}