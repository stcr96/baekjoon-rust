// 2501번, 약수 구하기

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: Vec<&str> = input.split_whitespace().collect();
    let n: u32 = input[0].trim().parse().unwrap();
    let mut k: u32 = input[1].trim().parse().unwrap();
    
    let mut output: u32 = 0;
    for i in 1..=n {
        if n % i == 0 {
            k -= 1;
        } else {
            continue;
        }
        if k == 0 {
            output = i;
            break;
        } else {
            continue;
        }
    }
    println!("{output}");
}