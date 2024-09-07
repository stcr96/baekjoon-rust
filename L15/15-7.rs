// 4948번, 베르트랑 공준

use std::io::{self, BufReader, BufRead, BufWriter, Write};
use std::f64;

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    
    let mut input = String::new();
    loop {
        reader.read_line(&mut input).unwrap();
        let n: u32 = input.trim().parse().unwrap();
        if n == 0 { break; }
        
        let mut count = 0;
        for i in n+1..=2*n {
            if is_prime(i) { count += 1; }
        }
        
        writeln!(writer, "{}", count).unwrap();
        input.clear();
    }
}

fn is_prime(i: u32) -> bool {
    if i < 2 { return false; }
    for div in 2..=(i as f64).sqrt() as u32 {
        if i % div == 0 { return false; }
    }
    true
}