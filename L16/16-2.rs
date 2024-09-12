// 10773번, 제로

use std::io::{self, BufReader, BufRead, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    
    let mut k = String::new();
    reader.read_line(&mut k).unwrap();
    let k: usize = k.trim().parse().unwrap();
    
    let mut stack: Vec<u32> = Vec::new();
    let mut input = String::new();
    for _ in 0..k {
        reader.read_line(&mut input).unwrap();
        let n: u32 = input.trim().parse().unwrap();
        if n == 0 {
            stack.pop();
        } else {
            stack.push(n);
        }
        input.clear();
    }
    let output: u32 = stack.iter().sum();
    writeln!(writer, "{}", output).unwrap();
}

