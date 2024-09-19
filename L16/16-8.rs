// 11866번, 요세푸스 문제 0

use std::io::{self, BufReader, BufRead, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut input_iter = input
        .trim()
        .split_whitespace()
        .map(|i| i.parse::<usize>().unwrap());
    let n = input_iter.next().unwrap();
    let k = input_iter.next().unwrap();
    
    let mut people: Vec<bool> = vec![true; n];
    let mut index: usize = 0;
    write!(writer, "<").unwrap();
    loop {
        let mut count = k;
        while count != 0 {
            if people[index] == true { count -= 1; }
            if count != 0 { index += 1; }
            if index == n { index = 0; }
        }
        people[index] = false;
        write!(writer, "{}", index+1).unwrap();
        if people.iter().all(|&i| i == false) { break; }
        else { write!(writer, ", ").unwrap(); }
    }
    write!(writer, ">").unwrap();
}