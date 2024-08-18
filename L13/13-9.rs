// 1181번, 단어 정렬

use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<i32>().unwrap();
    
    let mut input = String::new();
    for _ in 0..n {
        io::stdin().read_line(&mut input).unwrap();
    }
    let mut words: Vec<&str> = input
        .split_whitespace()
        .collect();
    words.sort_by(|a, b| {
        if a.len() == b.len() {
            a.cmp(&b)
        } else {
            a.len().cmp(&b.len())
        }
    });
    words.dedup();
    
    for item in &words {
        println!("{}", item);
    }
}