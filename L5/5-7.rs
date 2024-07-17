// 2675번, 문자열 반복

use std::io;

fn main() {
    let mut iter = String::new();
    io::stdin().read_line(&mut iter).unwrap();
    let iter: u32 = iter.trim().parse().unwrap();
    
    for _ in 0..iter {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: Vec<&str> = input.split_whitespace().collect();
        
        let n: usize = input[0].trim().parse().unwrap();
        for c in input[1].chars() {
            print!("{}", c.to_string().repeat(n));
        }
        print!("\n");
    }
}