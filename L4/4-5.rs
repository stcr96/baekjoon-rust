// 10810번, 공 넣기

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    
    let input_array: Vec<&str> = input.split_whitespace().collect();
    let n: usize = input_array[0].trim().parse().unwrap();
    let m: usize = input_array[1].trim().parse().unwrap();
    
    let mut bucket = vec![0; n];
    
    for _ in 0..m {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .unwrap();
        let line_array: Vec<&str> = line.split_whitespace().collect();
        let i: usize = line_array[0].trim().parse::<usize>().unwrap()-1;
        let j: usize = line_array[1].trim().parse().unwrap();
        let k: usize = line_array[2].trim().parse().unwrap();
        for index in i..j {
            bucket[index] = k;
        }
    }
    
    for i in 0..n {
        print!("{} ", bucket[i]);
    }
}