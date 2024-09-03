// 1735번, 분수 합

use std::io::{self, BufReader, BufRead, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    
    let mut input = String::new();
    for _ in 0..2 {
        reader.read_line(&mut input).unwrap();
    }
    let mut input_iter = input
        .trim()
        .split_whitespace()
        .map(|i| i.parse::<i32>().unwrap());
    let a_child = input_iter.next().unwrap();
    let a_parent = input_iter.next().unwrap();
    let b_child = input_iter.next().unwrap();
    let b_parent = input_iter.next().unwrap();
    
    // 분모의 최소공배수로 분수 더하기
    let parent = lcm(a_parent, b_parent);
    let child = a_child * (parent/a_parent) + b_child * (parent/b_parent);
    
    // 기약분수로 만들기
    let output_parent = parent / gcd(parent, child);
    let output_child = child / gcd(parent, child);
    
    writeln!(writer, "{} {}", output_child, output_parent).unwrap();
}

fn lcm(a: i32, b: i32) -> i32 {
    (a * b) / gcd(a, b)
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    let mut r: i32;
    while b != 0 {
        r = a % b;
        a = b;
        b = r;
    }
    a
}