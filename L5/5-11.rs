// 11718번, 그대로 출력하기

use std::io;

fn main() {
    let mut input = String::new();
    while io::stdin().read_line(&mut input).unwrap() > 0 {
        println!("{}", input.trim());
        input.clear();
    }
}