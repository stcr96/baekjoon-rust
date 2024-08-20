// 10989번, 수 정렬하기 3

use std::io::{self, BufReader, BufRead, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    
    let mut n = String::new();
    reader.read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();
    
    let mut counting_array: [usize; 10001] = [0; 10001];
    let mut input = String::new();
    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let input: usize = input.trim().parse().unwrap();
        counting_array[input] += 1;
    }
    for index in 0..counting_array.len() {
        for _ in 0..counting_array[index] {
            writeln!(writer, "{}", index).unwrap();
        }
    }
}