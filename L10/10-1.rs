// 27323번, 직사각형

use std::io;

fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let a = a.trim().parse::<i32>().unwrap();
    
    let mut b = String::new();
    io::stdin().read_line(&mut b).unwrap();
    let b = b.trim().parse::<i32>().unwrap();
    
    println!("{}", a*b);
}