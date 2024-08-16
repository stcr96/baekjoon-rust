// 1316번, 그룹 단어 체커

use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<i32>().unwrap();
    
    let mut output = 0;
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut input: Vec<char> = input.trim().chars().collect();
        input.dedup();
        let pre_count = input.len();
        
        input.sort();
        input.dedup();
        let pro_count = input.len();
        
        if pre_count == pro_count { output += 1; }
    }
    println!("{output}");
}