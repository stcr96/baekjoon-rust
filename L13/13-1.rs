// 2750번, 수 정렬하기

use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<i32>().unwrap();
    
    let mut sort_for_array: Vec<i32> = Vec::new();
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().parse::<i32>().unwrap();
        sort_for_array.push(input);
    }
    sort_for_array.sort();
    
    for i in sort_for_array {
        println!("{}", i);
    }
}