// 28278번, 스택 2

use std::io::{self, BufReader, BufRead, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    
    let mut n = String::new();
    reader.read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();
    
    let mut stack: Vec<i32> = Vec::new();
    let mut input = String::new();
    for _ in 0..n {
        reader.read_line(&mut input).unwrap();
        let mut input_iter = input
            .trim()
            .split_whitespace()
            .map(|i| i.parse::<i32>().unwrap());
        let inst = input_iter.next().unwrap();
        
        match inst {
            1 => {
                let x: i32 = input_iter.next().unwrap();
                stack.push(x);
            },
            2 => {
                if stack.len() == 0 {
                    writeln!(writer, "-1").unwrap();
                } else {
                    let output = stack.pop().unwrap();
                    writeln!(writer, "{output}").unwrap();
                }
            },
            3 => {
                writeln!(writer, "{}", stack.len()).unwrap();
            },
            4 => {
                if stack.len() == 0 { writeln!(writer, "1").unwrap(); }
                else { writeln!(writer, "0").unwrap(); }
            },
            5 => {
                if stack.len() == 0 {
                    writeln!(writer, "-1").unwrap();
                } else {
                    writeln!(writer, "{}", stack[stack.len()-1]).unwrap();
                }
            },
            _ => (),
        }
        input.clear();
    }
}

