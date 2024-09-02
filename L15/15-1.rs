// 1934번, 최소공배수

use std::io::{self, BufReader, BufRead, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    
    let mut t = String::new();
    reader.read_line(&mut t).unwrap();
    let t: usize = t.trim().parse().unwrap();
    
    for _ in 0..t {
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        let mut input_iter = input
            .trim()
            .split_whitespace()
            .map(|i| i.parse::<u32>().unwrap());
        let a = input_iter.next().unwrap();
        let b = input_iter.next().unwrap();
        
        let result = lcm(a, b);
        writeln!(writer, "{}", result).unwrap();
    }
}

fn lcm(a: u32, b: u32) -> u32 {
    (a * b) / gcd(a, b)
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    let mut r: u32;
    while b != 0 {
        r = a % b;
        a = b;
        b = r;
    }
    a
}