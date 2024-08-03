// 2869번, 달팽이는 올라가고 싶다

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: Vec<&str> = input.split_whitespace().collect();
    
    let a: u32 = input[0].trim().parse().unwrap();
    let b: u32 = input[1].trim().parse().unwrap();
    let v: u32 = input[2].trim().parse().unwrap();
    
    let count: u32;
    if (v-a)%(a-b) == 0 {
        count = (v-a)/(a-b) + 1;
    } else {
        count = (v-a)/(a-b)+1 + 1;
    }
    println!("{}", count);
}