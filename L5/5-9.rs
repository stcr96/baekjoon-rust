// 2908번, 상수

use std::io;
use std::cmp;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: Vec<&str> = input.split_whitespace().collect();
    
    let rev_a = input[0].chars().rev().collect::<String>().trim().parse::<i32>().unwrap();
    let rev_b = input[1].chars().rev().collect::<String>().trim().parse::<i32>().unwrap();
    
    println!("{}", cmp::max(rev_a, rev_b));
}