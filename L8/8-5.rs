// 2292번, 벌집

use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: i32 = n.trim().parse().unwrap();
    
    let mut k: i32 = 1;
    loop {
        if n == 1 {
            break;
        } else if 3*(k-1)*(k-1) - 3*(k-1) + 1 < n && 3*k*k - 3*k + 1 >= n {
            break;
        } else {
            k += 1;
        }
    }
    println!("{k}");
}