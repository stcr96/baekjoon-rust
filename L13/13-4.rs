// 2751번, 수 정렬하기 2

use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());

    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    let n: usize = buffer.trim().parse().unwrap();
    
    let mut numbers = Vec::with_capacity(n);
    for _ in 0..n {
        buffer.clear();
        reader.read_line(&mut buffer).unwrap();
        let number: i32 = buffer.trim().parse().unwrap();
        numbers.push(number);
    }
    numbers.sort();
    
    let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    for number in numbers {
        writeln!(writer, "{}", number).unwrap();
    }
}