// 2738번, 행렬 덧셈

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: Vec<&str> = input.split_whitespace().collect();
    
    let n: usize = input[0].trim().parse().unwrap();
    let m: usize = input[1].trim().parse().unwrap();
    
    // 행렬 A
    let mut a: Vec<Vec<u32>> = Vec::with_capacity(n*m);
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        
        let line: Vec<u32> = line
            .split_whitespace()
            .map(|v| v.trim().parse::<u32>().unwrap())
            .collect();
        a.push(line);
    }
    
    // 행렬 B
    let mut b: Vec<Vec<u32>> = Vec::with_capacity(n*m);
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        
        let line: Vec<u32> = line
            .split_whitespace()
            .map(|v| v.trim().parse::<u32>().unwrap())
            .collect();
        b.push(line);
    }
    
    let mut result: Vec<Vec<u32>> = vec![vec![0;m]; n];
    for i in 0..n {
        for j in 0..m {
            // println!("{}", a[i][j] + b[i][j]);
            result[i][j] = a[i][j] + b[i][j];
            print!("{} ", result[i][j]);
        }
        print!("\n");
    }
    
}