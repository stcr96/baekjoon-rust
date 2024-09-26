// 12789번, 도키도키 간식드리미

use std::collections::VecDeque;
use std::io::{self, BufReader, BufRead, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    
    let mut n = String::new();
    reader.read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();
    
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let balloons: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect();
    
    // 풍선 번호와 이동 값을 저장하는 VecDeque
    let mut queue: VecDeque<(usize, i32)> = VecDeque::new();
    for i in 0..n {
        queue.push_back((i + 1, balloons[i])); // (풍선 번호, 종이에 적힌 값)
    }

    // 첫 번째 풍선을 터뜨리고 시작
    let mut result = Vec::new();
    let mut current = queue.pop_front().unwrap();
    result.push(current.0);

    while !queue.is_empty() {
        let steps = current.1;
        if steps > 0 {
            // 오른쪽으로 이동 (steps-1만큼 이동)
            for _ in 0..(steps - 1) {
                let front = queue.pop_front().unwrap();
                queue.push_back(front);
            }
        } else {
            // 왼쪽으로 이동 (steps만큼 이동)
            for _ in 0..(-steps) {
                let back = queue.pop_back().unwrap();
                queue.push_front(back);
            }
        }

        // 다음 터뜨릴 풍선 선택
        current = queue.pop_front().unwrap();
        result.push(current.0);
    }

    // 결과 출력
    writeln!(writer, "{}", result.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")).unwrap();
}