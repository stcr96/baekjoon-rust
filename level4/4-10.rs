// 1546번, 평균

use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: Vec<&str> = input.split_whitespace().collect();
    
    let mut max: u32 = 0;
    let mut scores: Vec<u32> = Vec::new();
    
    for i in 0..n {
        let m: u32 = input[i].trim().parse().unwrap();
        scores.push(m);
        if max < m {
            max = m;
        }
    }
    
    let scores: Vec<f64> = scores.iter().map(|score| {
        (*score as f64)/(max as f64)*100.0
    })
    .collect();
    
    let sum: f64 = scores.iter().sum();
    let average: f64 = sum/scores.len() as f64;
    
    println!("{}", average);
    
}