// 2485번, 가로수

use std::io::{self, BufReader, BufRead, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    
    let mut n = String::new();
    reader.read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();
    
    let mut input = String::new();
    for _ in 0..n {
        reader.read_line(&mut input).unwrap();
    }
    let input: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect();
    
    // 가로수 간격 구하기
    let mut intervals: Vec<i32> = Vec::new();
    for i in 1..input.len() {
        intervals.push(input[i] - input[i-1]);
    }
    
    // 간격의 최대공약수 구하기
    let mut final_gcd: i32 = gcd(intervals[0], intervals[1]);
    for i in 2..intervals.len() {
        final_gcd = gcd(final_gcd, intervals[i]);
    }
    
    // 새로 심어야하는 개수 세기
    let output = (input[n-1] - input[0]) / final_gcd - (n as i32) + 1;
    writeln!(writer, "{}", output).unwrap();
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let r: i32 = a % b;
        a = b;
        b = r;
    }
    a
}