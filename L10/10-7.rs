// 5073번, 삼각형과 세 변

use std::io;

fn main() {
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut xs: Vec<i32> = line
            .split_whitespace()
            .map(|i| i.trim().parse::<i32>().unwrap())
            .collect();
        xs.sort();
        
        if xs[0] == 0 && xs[1] == 0 && xs[2] == 0 { break; }
        
        if xs[0]+xs[1] <= xs[2] {
            println!("Invalid");
        } else {
            if xs[0] == xs[1] && xs[0] == xs[2] && xs[1] == xs[2] {
                println!("Equilateral");
            } else if xs[0] != xs[1] && xs[0] != xs[2] && xs[1] != xs[2] {
                println!("Scalene");
            } else {
                println!("Isosceles");
            }
        }
    }
}