// 2562번, 최댓값

use std::io;

fn main() {
    let mut input = String::new();
    let mut max: u8 = 0;
    let mut line: u8 = 0;
    
    for i in 1..10 {
        io::stdin()
            .read_line(&mut input)
            .unwrap();
        
        let num: u8 = input.trim().parse().unwrap();
        
        if num > max {
            max = num;
            line = i;
        }
        input.clear();
    }
    println!("{max}");
    println!("{line}");
}