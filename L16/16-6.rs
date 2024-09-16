// 18258번, 큐 2

use std::io::{self, BufReader, BufRead, BufWriter, Write};
use std::collections::VecDeque;

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    
    let mut n = String::new();
    reader.read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();
    
    let mut queue: VecDeque<i32> = VecDeque::new();
    let mut input = String::new();
    for _ in 0..n {
        reader.read_line(&mut input).unwrap();
        let mut line_iter = input.trim().split_whitespace();
        
        let inst = line_iter.next().unwrap();
        let mut output = 0;
        match inst {
            "push" => {
                let x: i32 = line_iter.next().unwrap().parse().unwrap();
                queue.push_back(x);
                input.clear();
                continue;
            },
            "pop" => {
                if queue.is_empty() { output = -1; }
                else {
                    output = queue.pop_front().unwrap();
                }
            },
            "size" => output = queue.len() as i32,
            "empty" => {
                if queue.is_empty() { output = 1; }
                else { output = 0; }
            },
            "front" => {
                if queue.is_empty() { output = -1; }
                else { output = queue[0] }
            },
            "back" => {
                if queue.is_empty() { output = -1; }
                else { output = queue[queue.len()-1] as i32; }
            },
            _ => ()
        }
        writeln!(writer, "{}", output).unwrap();
        input.clear();
    }
}