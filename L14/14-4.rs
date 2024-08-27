// 1620번, 나는야 포켓몬 마스터 이다솜

use std::io::{self, BufReader, BufRead, BufWriter, Write};
use std::collections::HashMap;

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let mut line_iter = line
        .trim()
        .split_whitespace()
        .map(|i| i.parse::<usize>().unwrap());
    let n = line_iter.next().unwrap();
    let m = line_iter.next().unwrap();
    
    let mut pocketmons_by_name = HashMap::new();
    let mut pocketmons_by_num = Vec::new();
    let mut input = String::new();
    for i in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let pocketmon = input.trim().to_string();
        pocketmons_by_name.insert(pocketmon.clone(), i);
        pocketmons_by_num.push(pocketmon);
    }
    
    for _ in 0..m {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let query = input.trim().to_string();
        let mut result = String::new();
        if let Ok(i) = query.parse::<usize>() {
            result = pocketmons_by_num[i-1].clone();
        } else {
            let i = pocketmons_by_name[&query];
            result = (i+1).to_string();
        }
        writeln!(writer, "{}", result).unwrap();
    }
}