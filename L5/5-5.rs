// 11720번, 숫자의 합

use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();
    
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let line: Vec<char> = line.trim().chars().collect();
    
    let mut sum: u32 = 0;
    for i in 0..n {
        sum += line[i].to_digit(10).unwrap();
    }
    
    println!("{}", sum);
}