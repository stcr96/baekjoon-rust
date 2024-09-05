// 4134번, 다음 소수

use std::io::{self, BufReader, BufRead, BufWriter, Write};
use std::f64;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    
    let mut num = String::new();
    reader.read_line(&mut num).unwrap();
    let num: usize = num.trim().parse().unwrap();
    
    let mut input = String::new();
    for _ in 0..num {
        reader.read_line(&mut input).unwrap();
        let n: u32 = input.trim().parse().unwrap();
        
        // n보다 크거나 같은 소수 중 가장 작은 소수
        let mut output = n;
        'a: loop {
            let limit = (output as f64).sqrt() as u32;
            if output < 2 {
                output = 2;
            }
            for i in 2..=limit {
                if output % i == 0 {
                    output += 1;
                    continue 'a;
                }
            }
            break;
        }
        
        writeln!(writer, "{}", output).unwrap();
        input.clear();
    }
    
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("{:?}", elapsed_time);
}
