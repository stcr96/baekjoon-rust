// 25305번, 커트라인

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut input_iter = input
        .split_whitespace()
        .map(|i| i.trim().parse::<usize>().unwrap());
    let n = input_iter.next().unwrap();
    let k = input_iter.next().unwrap();
    input.clear();
    
    io::stdin().read_line(&mut input).unwrap();
    let mut scores: Vec<i32> = input
        .split_whitespace()
        .map(|score| score.trim().parse::<i32>().unwrap())
        .collect();
    scores.sort();
    println!("{}", scores[n-k]);
}