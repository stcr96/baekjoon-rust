// 15552번, 빠른 A + B

use std::io::{self, Write, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::BufWriter::new(io::stdout().lock());

    // 버퍼된 입력을 받습니다.
    let mut lines = stdin.lock().lines();

    // 첫 번째 줄에서 테스트 케이스의 수를 받습니다.
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // 각 테스트 케이스에 대해 A + B를 계산하여 출력합니다.
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let mut numbers = line.split_whitespace();
        let a: i32 = numbers.next().unwrap().parse().unwrap();
        let b: i32 = numbers.next().unwrap().parse().unwrap();
        writeln!(stdout, "{}", a + b).unwrap();
    }
}