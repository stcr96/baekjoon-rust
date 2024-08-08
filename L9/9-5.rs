// 2581번, 소수

use std::io;

fn main() {
    let mut m = String::new();
    io::stdin().read_line(&mut m).unwrap();
    let m: i32 = m.trim().parse().unwrap();
    
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: i32 = n.trim().parse().unwrap();
    
    let mut prime: Vec<i32> = Vec::new();
    for x in m..=n {
        if is_prime(x) {
            prime.push(x);
        } else {
            continue
        }
    }
    
    if prime.len() == 0 {
        println!("{}", -1);
    } else {
        let sum: i32 = prime.iter().sum();
        println!("{}", sum);
        println!("{}", prime[0]);
    }
}

fn is_prime(num: i32) -> bool {
    if num <= 1 {
        false
    } else {
        for n in 2..=num/2 {
            if num % n == 0 { return false; }
        }
        true
    }
}