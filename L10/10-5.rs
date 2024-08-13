// 9063번, 대지

use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<i32>().unwrap();
    
    let mut x_vec: Vec<i32> = Vec::new();
    let mut y_vec: Vec<i32> = Vec::new();
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut line_iter = line
            .split_whitespace()
            .map(|i| i.trim().parse::<i32>().unwrap());
        x_vec.push(line_iter.next().unwrap());
        y_vec.push(line_iter.next().unwrap());
    }
    
    let width = x_vec.iter().max().unwrap() - x_vec.iter().min().unwrap();
    let height = y_vec.iter().max().unwrap() - y_vec.iter().min().unwrap();
    
    println!("{}", width * height);
}