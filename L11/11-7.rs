// 24313번, 알고리즘 수업 - 점근적 표기 1

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut input_iter = input
        .split_whitespace()
        .map(|i| i.trim().parse::<i32>().unwrap());
    let a_1 = input_iter.next().unwrap();
    let a_0 = input_iter.next().unwrap();
    
    let mut c = String::new();
    io::stdin().read_line(&mut c).unwrap();
    let c = c.trim().parse::<i32>().unwrap();
    
    let mut n_0 = String::new();
    io::stdin().read_line(&mut n_0).unwrap();
    let n_0 = n_0.trim().parse::<i32>().unwrap();
    
    // f(n) = a_1*n + a_0 <= c*n
    if a_1 * n_0 + a_0 <= c * n_0 && a_1 <= c {
        println!("1");
    } else {
        println!("0");
    }
}