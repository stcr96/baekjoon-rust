// 2588번, 곱셈

use std::io;

fn main() {
    // 입력 ... 1
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let a: u32 = a.trim().parse().unwrap();
    
    // 입력 ... 2
    let mut b = String::new();
    io::stdin().read_line(&mut b).unwrap();
    let b_num: u32 = b.trim().parse().unwrap();
    let b_1: u32 = b.chars().nth(0).unwrap().to_string().parse().unwrap();
    let b_2: u32 = b.chars().nth(1).unwrap().to_string().parse().unwrap();
    let b_3: u32 = b.chars().nth(2).unwrap().to_string().parse().unwrap();
    
    println!("{}", a*b_3);
    println!("{}", a*b_2);
    println!("{}", a*b_1);
    println!("{}", a*b_num);
}