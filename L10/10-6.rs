// 10101번, 삼각형 외우기

use std::io;

fn main() {
    let mut input = String::new();
    for _ in 0..3 {
        io::stdin().read_line(&mut input).unwrap();
    }
    let input: Vec<i32> = input
        .split_whitespace()
        .map(|i| i.trim().parse::<i32>().unwrap())
        .collect();
    
    if input.iter().sum::<i32>() != 180 {
        println!("Error");
    } else {
        if input[0] == input[1] && input[1] == input[2] {
            println!("Equilateral");
        } else if input[0] != input[1] && input[1] != input[2] && input[0] != input[2] {
            println!("Scalene");
        } else {
            println!("Isosceles");
        }
    }
}