// 11021번, A + B - 7

use std::io;

fn main() {
    let mut num = String::new();
    io::stdin().read_line(&mut num).unwrap();
    
    let num: u128 = num.trim().parse().unwrap();
    
    for n in 1..=num {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let input: Vec<&str> = input.split_whitespace().collect();
        let a: u32 = input[0].trim().parse().unwrap();
        let b: u32 = input[1].trim().parse().unwrap();
        
        println!("Case #{n}: {}", a+b)
    }
}