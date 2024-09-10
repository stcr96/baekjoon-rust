// 3009번, 네 번째 점

use std::io::{self, BufReader, BufRead, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    
    let mut xs: Vec<i32> = Vec::new();
    let mut ys: Vec<i32> = Vec::new();
    
    let mut input = String::new();
    for _ in 0..3 {
        reader.read_line(&mut input).unwrap();
        let mut input_iter = input
            .trim()
            .split_whitespace()
            .map(|i| i.parse().unwrap());
        let x = input_iter.next().unwrap();
        let y = input_iter.next().unwrap();
        xs.push(x);
        ys.push(y);
        input.clear();
    }
    
    let fourth_x = find_unique(&xs);
    let fourth_y = find_unique(&ys);
    writeln!(writer, "{} {}", fourth_x, fourth_y).unwrap();
}

fn find_unique(input: &Vec<i32>) -> i32 {
    if input[0] == input[1] {
        input[2]
    } else if input[0] == input[2] {
        input[1]
    } else {
        input[0]
    }
}