// 9086번, 문자열

use std::io;

fn main() {
    let mut num = String::new();
    io::stdin().read_line(&mut num).unwrap();
    let num: usize = num.trim().parse().unwrap();
    
    for _ in 0..num {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let length: usize = input.trim().len();
        
        println!("{}{}", input.chars().nth(0).unwrap(), input.chars().nth(length-1).unwrap());
    }
}