// 10815번, 숫자 카드

use std::io::{self, BufReader, BufRead, BufWriter, Write};
use std::collections::HashMap;

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    
    let mut n = String::new();
    reader.read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();
    
    let mut map: HashMap<i32, i32> = HashMap::new();
    
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut input_iter = input
        .trim()
        .split_whitespace()
        .map(|i| i.parse::<i32>().unwrap());
    for _ in 0..n {
        let key = input_iter.next().unwrap();
        map.insert(key, 1);
    }
    
    // println!("{:?}", map);
    
    let mut m = String::new();
    reader.read_line(&mut m).unwrap();
    let m: usize = m.trim().parse().unwrap();
    
    let mut todo = String::new();
    reader.read_line(&mut todo).unwrap();
    let todo = todo
        .trim()
        .split_whitespace()
        .map(|i| i.parse::<i32>().unwrap());
    for i in todo {
        if map.get(&i).is_some() {
            write!(writer, "{} ", map.get(&i).unwrap()).unwrap();
        } else {
            write!(writer, "0 ").unwrap();
        }
    }
        
    
}