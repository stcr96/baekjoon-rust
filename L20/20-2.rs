// 25192번, 인사성 밝은 곰곰이

use std::io::{self, BufReader, BufRead, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    
    let mut n = String::new();
    reader.read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();
    
    let mut input = String::new();
    let mut count = 0;
    for _ in 0..n {
        reader.read_line(&mut input).unwrap();
    }
    let input: Vec<String> = input
        .trim()
        .split_whitespace()
        .map(|i| i.to_string())
        .collect();

    let input_iter = input.split(|i| *i == "ENTER");
    for i in input_iter {
      let mut hello = i.to_vec();
      hello.sort_unstable();
      hello.dedup();
      count += hello.len();
    }

    writeln!(writer, "{}", count).unwrap();
}