// 10813번, 공 바꾸기

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: Vec<&str> = input.split_whitespace().collect();
    let n: u32 = input[0].trim().parse().unwrap();
    let m: u32 = input[1].trim().parse().unwrap();
    let mut bucket = Vec::new();
    
    for k in 1..=n {
        bucket.push(k);
    }
    
    for _ in 0..m {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let line: Vec<&str> = line.split_whitespace().collect();
        let i: usize = line[0].trim().parse::<usize>().unwrap() - 1;
        let j: usize = line[1].trim().parse::<usize>().unwrap() - 1;
        
        let temp: u32 = bucket[i].clone();
        bucket[i] = bucket[j];
        bucket[j] = temp;
    }
    
    bucket.into_iter().for_each(|item| print!("{} ", item));
}