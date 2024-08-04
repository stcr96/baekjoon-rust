// 5086번, 배수와 약수

use std::io;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: Vec<&str> = input.split_whitespace().collect();
        
        let a: i32 = input[0].trim().parse().unwrap();
        let b: i32 = input[1].trim().parse().unwrap();
        if a == 0 && b == 0 { break }
        else if b % a == 0 { println!("factor") }
        else if a % b == 0 { println!("multiple") }
        else { println!("neither") }
    }
}