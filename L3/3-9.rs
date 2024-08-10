// 2438번, 별 찍기 - 1

use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Cannot read line.");
    
    let input: usize = input.trim().parse().unwrap();
    
    for i in 1..=input {
        println!("{}", "*".repeat(i));
    }
}