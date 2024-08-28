// 10816번, 숫자 카드 2

use std::io::{self, BufReader, BufRead, BufWriter, Write};
use std::collections::HashMap;

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    
    let mut n = String::new();
    reader.read_line(&mut n).unwrap();
    
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let input_vec: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|i| i.parse::<i32>().unwrap())
        .collect();
    
    let mut set = HashMap::new();
    for i in input_vec {
        set.entry(i).and_modify(|counter| *counter += 1).or_insert(1);
    }
    
    let mut m = String::new();
    reader.read_line(&mut m).unwrap();
    
    input.clear();
    reader.read_line(&mut input).unwrap();
    let input: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|i| i.parse::<i32>().unwrap())
        .collect();
    
    for i in input {
        if let Some(n) = set.get(&i) {
            write!(writer, "{} ", n).unwrap();
        } else {
            write!(writer, "0 ").unwrap();
        }
    }
}