// 14215번, 세 막대

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut input: Vec<i32> = input
        .split_whitespace()
        .map(|i| i.trim().parse::<i32>().unwrap())
        .collect();
    input.sort();    
    
    let result: i32;
    if input[2] >= input[0] + input[1] {
        input.pop();
        result = input.iter().sum::<i32>() * 2 - 1;
    } else {
        result = input.iter().sum::<i32>();
    }
    println!("{}", result);
}