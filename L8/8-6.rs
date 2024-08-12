// 1193번, 분수찾기

use std::io;

fn main() {
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let x = x.trim().parse::<i32>().unwrap();
    
    let mut input: i32 = x;
    let mut layer: i32 = 0;
    
    let mut child = 0;
    let mut parent = 1;
    
    while input > 0 {
        layer += 1;
        if layer % 2 == 0 {
            parent += 1;
        } else {
            child += 1;
        }
        
        if input > layer {
            for _ in 1..layer {
                if layer % 2 == 0 {
                    child += 1;
                    parent -= 1;
                } else {
                    child -= 1;
                    parent += 1;
                }
            }
        } else {
            for _ in 1..input {
                if layer % 2 == 0 {
                    child += 1;
                    parent -= 1;
                } else {
                    child -= 1;
                    parent += 1;
                }
            }
        }
        input -= layer;
    }
    println!("{child}/{parent}");
}