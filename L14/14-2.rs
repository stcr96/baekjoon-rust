// 14425번, 문자열 집합

use std::io::{self, BufReader, BufRead, BufWriter, Write};
use std::collections::HashSet;

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let mut line_iter = line
        .trim()
        .split_whitespace()
        .map(|i| i.parse::<usize>().unwrap());
    let n = line_iter.next().unwrap();
    let m = line_iter.next().unwrap();
    
    let mut s = HashSet::new();
    let mut input = String::new();
    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        s.insert(input.clone());
    }
    
    let mut counter: u32 = 0;
    for _ in 0..m {
        input.clear();
        reader.read_line(&mut input).unwrap();
        if s.contains(&input) {
            counter += 1;
        }
    }
    writeln!(writer, "{}", counter).unwrap();
}