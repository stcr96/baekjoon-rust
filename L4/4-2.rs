// 10871번, X보다 작은 수

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    let input: Vec<&str> = input.split_whitespace().collect();
    
    let n: usize = input[0].trim().parse().unwrap();
    let x: u32 = input[1].trim().parse().unwrap();
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    let input: Vec<&str> = input.split_whitespace().collect();
    
    // let mut result: Vec<u32> = Vec::new();
    for i in 0..n {
        let item: u32 = input[i].trim().parse().unwrap();
        if item < x {
            print!("{item} ");
        }
    }
}