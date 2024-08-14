// 2444번, 별 찍기 - 7

use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<usize>().unwrap();
    
    for i in 1..=n {
        println!("{}{}", " ".repeat(n-i), "*".repeat(2*i-1));
    }
    for j in (1..n).rev() {
        println!("{}{}", " ".repeat(n-j), "*".repeat(2*j-1));
    }
}