// 11478번, 서로 다른 부분 문자열의 개수

use std::io::{self, BufReader, BufRead, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    
    // 입력 받기
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    input = input.trim().to_string();
    
    // 부분 문자열 벡터 만들기
    let mut words: Vec<&str> = Vec::new();
    for i in 0..input.len() {
        for j in i..input.len() {
            words.push(&input[i..=j]);
        }
    }
    
    // 중복 제거하기
    words.sort_unstable();
    words.dedup();
    
    // 개수 출력하기
    writeln!(writer, "{}", words.len()).unwrap();
}