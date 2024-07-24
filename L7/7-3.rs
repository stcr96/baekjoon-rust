// 10798번, 세로읽기

use std::io;

fn main() {
    // 입력
    let mut input = String::new();
    for _ in 0..5 {
        io::stdin().read_line(&mut input).unwrap();
    }
    
    let input: Vec<&str> = input.split_whitespace().collect();
    // println!("{:?}", input); -> ["ABCDE", "abcde", "01234", "FGHIJ", "fghij"]
    
    let mut output = String::new();
    let mut longest_word_length: usize = 0;
    for word in input.iter() {
        if longest_word_length < word.len() {
            longest_word_length = word.len();
        }
    }
    
    for c in 0..longest_word_length {
        for i in 0..5 {
            if input[i].chars().nth(c) == None { continue }
            output.push(input[i].chars().nth(c).unwrap());
        }
    }
    println!("{}", output);
}