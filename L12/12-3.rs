// 19532번, 수학은 비대면강의입니다

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut input_iter = input
        .split_whitespace()
        .map(|i| i.trim().parse::<i32>().unwrap());
    let a = input_iter.next().unwrap();
    let b = input_iter.next().unwrap();
    let c = input_iter.next().unwrap();
    let d = input_iter.next().unwrap();
    let e = input_iter.next().unwrap();
    let f = input_iter.next().unwrap();
    
    for x in -999..=999 {
        for y in -999..=999 {
            if a*x + b*y == c && d*x + e*y == f {
                println!("{} {}", x, y);
                break;
            }
        }
    }
}