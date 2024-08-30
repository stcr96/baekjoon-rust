// 1269번, 대칭 차집합

use std::io::{self, BufReader, BufRead, BufWriter, Write};
use std::collections::HashSet;

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    
    // 집합 A
    let mut input = String::new();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let input_iter = input
        .trim()
        .split_whitespace()
        .map(|i| i.parse::<u32>().unwrap());
    let set_a: HashSet<u32> = HashSet::from_iter(input_iter);
    
    
    // 집합 B
    input.clear();
    reader.read_line(&mut input).unwrap();
    let input_iter = input
        .trim()
        .split_whitespace()
        .map(|i| i.parse::<u32>().unwrap());
    let set_b: HashSet<u32> = HashSet::from_iter(input_iter);
    
    let set_sym_diff: HashSet<_> = set_a.symmetric_difference(&set_b).collect();
    let output = set_sym_diff.len();
    
    writeln!(writer, "{}", output).unwrap();
    
}