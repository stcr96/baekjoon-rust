// 5597번, 과제 안 내신 분..?

use std::io;

fn main() {
    let mut students = [0; 30];
    // println!("{:?}", students);
    let mut input = String::new();
    
    for _ in 0..28 {
        io::stdin().read_line(&mut input).unwrap();
    }
    
    let submitted: Vec<&str> = input.split_whitespace().collect();
    
    for i in submitted.iter() {
        let index: usize = i.trim().parse().unwrap();
        students[index-1] = 1;
    }
    
    for i in 0..30 {
        if students[i]==0 {
            println!("{}", i+1);
        }
    }
    // println!("{:?}", students);
}