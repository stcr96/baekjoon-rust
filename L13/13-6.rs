// 1427번, 소트인사이드

use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let mut n: Vec<u32> = n
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    n.sort();
    let sorted_n = n
        .iter()
        .rev()
        .map(|i| i.to_string())
        .collect::<String>();
    
    println!("{}", sorted_n);
}