// 18870번, 좌표 압축

use std::io::{self, BufReader, BufRead, BufWriter, Write};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::new(stdout.lock());

    let mut n = String::new();
    reader.read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let input: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect();

    // input의 정렬된 중복 제거된 복사본 생성
    let mut sorted_input = input.clone();
    sorted_input.sort();
    sorted_input.dedup();

    // 해시맵을 사용하여 각 값에 대해 압축된 인덱스를 저장
    let mut index_map = HashMap::new();
    for (i, &value) in sorted_input.iter().enumerate() {
        index_map.insert(value, i);
    }

    // 압축된 결과 출력
    for &value in &input {
        let result = index_map[&value];
        write!(writer, "{} ", result).unwrap();
    }
}
