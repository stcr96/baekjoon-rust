// 2941번, 크로아티아 알파벳

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    
    let croatia_alphabet = vec!["c=", "c-", "dz=", "d-", "lj", "nj", "s=", "z="];
    let mut index = 0;
    let mut count = 0;
    
    while index < input.len() {
        let mut matched = false;
    
        for &alphabet in &croatia_alphabet {
            if index+alphabet.len() <= input.len() && &input[index..index+alphabet.len()] == alphabet {
                count += 1;
                index += alphabet.len();
                matched = true;
                break;
            }
        }
        
        if !matched {
            count += 1;
            index += 1;
        }
    }
    
    println!("{}", count);
}