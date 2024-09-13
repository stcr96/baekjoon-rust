// 9012번, 괄호

use std::io::{self, BufReader, BufRead, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    
    let mut t = String::new();
    reader.read_line(&mut t).unwrap();
    let t: usize = t.trim().parse().unwrap();
    
    let mut input = String::new();
    for _ in 0..t {
        let mut count = 0;
        reader.read_line(&mut input).unwrap();
        for c in input.chars() {
            match c {
                '(' => count += 1,
                ')' => count -= 1,
                _ => (),
            }
            if count < 0 { break; }
        }
        if count != 0 { writeln!(writer, "NO").unwrap(); }
        else { writeln!(writer, "YES").unwrap(); }
        input.clear();
    }
}

