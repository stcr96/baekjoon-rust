// 2720번, 세탁소 사장 동혁

use std::io;

fn main() {
    let mut num = String::new();
    io::stdin().read_line(&mut num).unwrap();
    let num: usize = num.trim().parse().unwrap();
    
    for _ in 0..num {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: u32 = input.trim().parse().unwrap();
        
        let mut rest: u32;
        let quarter: u32 = input/25;
        rest = input-quarter*25;
        let dime: u32 = rest/10;
        rest -= rest/10 * 10;
        let nickel: u32 = rest/5;
        rest -= rest/5 * 5;
        let penny: u32 = rest/1;
        println!("{quarter} {dime} {nickel} {penny}");
    }
}