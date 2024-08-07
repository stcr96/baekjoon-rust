// 1978번, 소수 찾기

use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: Vec<&str> = input.split_whitespace().collect();
    
    let mut count = 0;
    for i in input {
        let x: i32 = i.trim().parse().unwrap();
        if is_prime(x) { count += 1 }
    }
    println!("{}", count);
}

fn is_prime(num: i32) -> bool {
    if num <= 1 {
        false
    } else {
        for n in 2..num {
            if num % n == 0 { return false; }
        }
        true
    }
}