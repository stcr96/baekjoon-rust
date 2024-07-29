// 11005번, 진법 변환 2

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: Vec<&str> = input.split_whitespace().collect();
    
    let mut n: usize = input[0].trim().parse().unwrap();
    let b: usize = input[1].trim().parse().unwrap();
    
    let mut output = String::new();
    let digits = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    if n == 0 {
        output.push('0');
    } else {
        while n > 0 {
            output.push(digits.chars().nth(n % b).unwrap());
            n /= b;
        }
    }
    
    let result: String = output.chars().rev().collect();
    println!("{}", result);
}
