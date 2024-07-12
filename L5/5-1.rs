// 27866번, 문자와 문자열

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let mut index = String::new();
    io::stdin().read_line(&mut index).unwrap();
    let index: usize = index.trim().parse().unwrap();
    
    let chars: Vec<char> = input.trim().chars().collect();
    
    println!("{}", chars[index-1]);
}