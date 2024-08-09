// 11653번, 소인수분해

use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: i32 = n.trim().parse().unwrap();
    
    let mut copy_n: i32 = n;
    
    for div in 2..=n {
        while copy_n % div == 0 {
            copy_n /= div;
            println!("{}", div);
        }
    }
}