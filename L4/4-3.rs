// 10818번, 최소, 최대

use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();
    
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let a: Vec<&str> = a.split_whitespace().collect();
    
    let mut min: i128 = a[0].trim().parse().unwrap();
    let mut max: i128 = a[0].trim().parse().unwrap();
    let mut item: i128;
    
    for i in 1..n {
        item = a[i].trim().parse().unwrap();
        if item < min {
            min = item;
        }
        if item > max {
            max = item;
        }
    }
    
    println!("{min} {max}");
}