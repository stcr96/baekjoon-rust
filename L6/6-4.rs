// 10988번, 팰린드롬인지 확인하기

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    
    if input == input.chars().rev().collect::<String>() {
        println!("1");
    } else {
        println!("0");
    }
}