// 13241번, 최소공배수

use std::io::{self, BufReader, BufRead, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut input_iter = input
        .trim()
        .split_whitespace()
        .map(|i| i.parse::<u64>().unwrap());
    let a = input_iter.next().unwrap();
    let b = input_iter.next().unwrap();
    
    let result = lcm(a, b);
    writeln!(writer, "{}", result).unwrap();
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    let mut r: u64;
    while b != 0 {
        r = a % b;
        a = b;
        b = r;
    }
    a
}