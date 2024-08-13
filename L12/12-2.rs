// 2231번, 분해합

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u32>().unwrap();
    
    let mut sum: u32;
    let mut g_vec: Vec<u32> = Vec::new();
    for i in 1..=n {
        sum = i;
        sum += i.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .sum::<u32>();
        // println!("{sum}");
        if sum == n { g_vec.push(i); }
    }
    
    if g_vec.len() == 0 {
        println!("0");
    } else {
        println!("{}", g_vec.iter().min().unwrap());
    }
}