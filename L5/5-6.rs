use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let chars: Vec<char> = input.trim().chars().collect();
    
    let alphabet: Vec<char> = ('a'..='z').into_iter().collect();
    let mut result = Vec::new();
    for i in alphabet.iter() {
        // println!("{i}");
        match chars.iter().position(|&c| c == *i) {
            Some(n) => result.push(n as i32),
            None => result.push(-1),
        }
    }
    
    for n in result.iter() {
        print!("{n} ", );
    }
}