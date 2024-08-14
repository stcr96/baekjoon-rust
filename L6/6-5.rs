// 1157번, 단어 공부

use std::io;
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_uppercase();
    
    let mut used_alphabet = HashMap::new();
    for char in input.chars() {
        used_alphabet
            .entry(char)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    
    let mut max_value = 0;
    let mut max_alphabet = '?';
    for (&key, &value) in &used_alphabet {
        if value > max_value {
            max_value = value;
            max_alphabet = key;
        } else if value == max_value {
            max_alphabet = '?';
        }
    }
    println!("{max_alphabet}")
}