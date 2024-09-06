// 1929번, 소수 구하기

use std::io::{self, BufReader, BufRead, BufWriter, Write};
use std::f64;

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut input_iter = input
        .trim()
        .split_whitespace()
        .map(|i| i.parse::<u32>().unwrap());
    let m = input_iter.next().unwrap();
    let n = input_iter.next().unwrap();
    
    for i in m..=n {
        if is_prime(i) {
            writeln!(writer, "{}", i).unwrap();
        }
    }
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        false
    } else {
        for x in 2..=((n as f64).sqrt() as u32) {
            if n % x == 0 { return false; }
        }
        true
    }
}
