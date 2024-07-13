// 2743번, 단어 길이 재기

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let result: usize = input.trim().len();
    
    println!("{result}");
}