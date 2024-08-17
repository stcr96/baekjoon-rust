// 10807번, 개수 세기

use std::io;

fn main() {
    let mut num = String::new();
    io::stdin().read_line(&mut num).unwrap();
    let num: usize = num.trim().parse().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: Vec<&str> = input.split_whitespace().collect();
    
    let mut result = 0;
    let mut v = String::new();
    io::stdin().read_line(&mut v).unwrap();
    let v: i32 = v.trim().parse().unwrap();
    
    for i in 0..num {
        let n: i32 = input[i].trim().parse().unwrap();
        if n == v { result += 1 }
    }
    println!("{result}");
}