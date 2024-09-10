// 13909번, 창문 닫기

use std::io::{self, BufReader, BufRead, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    
    let mut n = String::new();
    reader.read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();
    
    let count = (1..).take_while(|&x| x * x <= n).count();
    
    writeln!(writer, "{}", count).unwrap();
}
