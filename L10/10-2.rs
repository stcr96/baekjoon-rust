// 1085번, 직사각형에서 탈출

use std::io;
use std::cmp;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: Vec<&str> = input.split_whitespace().collect();
    
    let x = input[0].trim().parse::<u32>().unwrap();
    let y = input[1].trim().parse::<u32>().unwrap();
    let w = input[2].trim().parse::<u32>().unwrap();
    let h = input[3].trim().parse::<u32>().unwrap();
    
    let x_min = cmp::min(x-0, w-x);
    let y_min = cmp::min(y-0, h-y);
    
    println!("{}", cmp::min(x_min, y_min));
}