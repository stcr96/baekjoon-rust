// 24511번, queuestack

use std::io::{self, BufReader, BufRead, BufWriter, Write};
use std::collections::VecDeque;

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    
    input.clear();
    reader.read_line(&mut input).unwrap();
    let a: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect();
    
    input.clear();
    reader.read_line(&mut input).unwrap();
    let b: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect();
    
    input.clear();
    reader.read_line(&mut input).unwrap();
    let m: usize = input.trim().parse().unwrap();
    
    input.clear();
    reader.read_line(&mut input).unwrap();
    let c: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect();
    
    let mut queuestack: VecDeque<i32> = VecDeque::new();
    for (i, v) in b.iter().enumerate() {
        if a[i] == 0 { queuestack.push_back(b[i]); }
    }
    // println!("{:?}", queuestack);
    
    for i in c {
        queuestack.push_front(i);
        let output = queuestack.pop_back().unwrap();
        write!(writer, "{output} ").unwrap();
    }
}