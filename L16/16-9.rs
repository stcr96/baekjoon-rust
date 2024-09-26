// 28279번, 덱 2

use std::io::{self, BufReader, BufRead, BufWriter, Write};
use std::collections::VecDeque;

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    
    let mut n = String::new();
    reader.read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();
    
    let mut output: VecDeque<i32> = VecDeque::new();
    let mut input = String::new();
    for _ in 0..n {
        reader.read_line(&mut input).unwrap();
        let mut input_iter = input
            .trim()
            .split_whitespace();
        let inst = input_iter.next().unwrap();
        match inst {
            "1" => {
                let x = input_iter.next().unwrap().parse::<i32>().unwrap();
                output.push_front(x);
            },
            "2" => {
                let x = input_iter.next().unwrap().parse::<i32>().unwrap();
                output.push_back(x);
            },
            "3" => {
                if let Some(x) = output.pop_front() {
                    writeln!(writer, "{x}").unwrap();
                } else {
                    writeln!(writer, "-1").unwrap();
                }
            },
            "4" => {
                if let Some(x) = output.pop_back() {
                    writeln!(writer, "{x}").unwrap();
                } else {
                    writeln!(writer, "-1").unwrap();
                }
            },
            "5" => writeln!(writer, "{}", output.len()).unwrap(),
            "6" => {
                if output.len() == 0 {
                    writeln!(writer, "1").unwrap();
                } else {
                    writeln!(writer, "0").unwrap();
                }
            },
            "7" => {
                if output.len() != 0 {
                    writeln!(writer, "{}", output[0]).unwrap();
                } else {
                    writeln!(writer, "-1").unwrap();
                }
            },
            "8" => {
                if output.len() != 0 {
                    writeln!(writer, "{}", output[output.len()-1]).unwrap();
                } else {
                    writeln!(writer, "-1").unwrap();
                }
            },
            _ => (),
        }
        input.clear();
    }
}