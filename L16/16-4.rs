// 4949번, 균형잡힌 세상

use std::io::{self, BufReader, BufRead, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    
    let mut input = String::new();
    loop {
        reader.read_line(&mut input).unwrap();
        input.pop();
        
        let mut stack: Vec<char> = Vec::new();
        if input == "." { break; }
        for c in input.chars() {
            match c {
                '[' => stack.push(c),
                '(' => stack.push(c),
                ']' => {
                    if stack.len() > 0 && stack[stack.len()-1] == '[' { stack.pop(); }
                    else { 
                        writeln!(writer, "no").unwrap();
                        break;
                    }
                },
                ')' => {
                    if stack.len() > 0 && stack[stack.len()-1] == '(' { stack.pop(); }
                    else { 
                        writeln!(writer, "no").unwrap();
                        break;
                    }
                },
                '.' => {
                    if stack.is_empty() {
                        writeln!(writer, "yes").unwrap();
                    } else {
                        writeln!(writer, "no").unwrap();
                    }
                },
                _ => (),
                
            }
        }
        input.clear();
    }
}

