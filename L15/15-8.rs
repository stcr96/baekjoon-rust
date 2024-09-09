// 17103번, 골드바흐 파티션

use std::io::{self, BufReader, BufRead, BufWriter, Write};
use std::f64;

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    
    let mut t = String::new();
    reader.read_line(&mut t).unwrap();
    let t: usize = t.trim().parse().unwrap();
    
    // 에라토스테네스의 체로 미리 소수 구하기
    let max_n = 1_000_000;
    let is_prime = eratos(max_n);
    
    let mut input = String::new();
    for _ in 0..t {
        reader.read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();
        let mut count = 0;
        
        // 모든 가능한 소수쌍 찾기
        for i in 2..=n / 2 {
            if is_prime[i] && is_prime[n - i] {
                count += 1;
            }
        }
        
        writeln!(writer, "{}", count).unwrap();
        input.clear();
    }
}

// 에라토스테네스의 체를 사용해 소수 구하기
fn eratos(n: usize) -> Vec<bool> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    
    for i in 2..=(n as f64).sqrt() as usize {
        if is_prime[i] {
            for j in (i * i..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    
    is_prime
}