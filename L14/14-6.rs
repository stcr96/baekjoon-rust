// 1764번, 듣보잡

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
    
    let mut input = String::new();
    let mut no_heard = HashSet::new();
    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        no_heard.insert(input.trim().to_string());
    }
    
    let mut no_seen = HashSet::new();
    for _ in 0..m {
        input.clear();
        reader.read_line(&mut input).unwrap();
        no_seen.insert(input.trim().to_string());
    }
    
    let mut no_heard_no_seen: Vec<String> = Vec::new();
    let mut count = 0;
    for i in no_heard.iter() {
        if no_seen.contains(i) {
            count += 1;
            no_heard_no_seen.push(i.to_string());
        }
    }
    no_heard_no_seen.sort_unstable();
    
    writeln!(writer, "{count}").unwrap();
    for i in no_heard_no_seen.iter() {
        writeln!(writer, "{}", i).unwrap();
    }
}