// 10811번, 바구니 뒤집기

use std::io;

fn main() {
    // n, m 입력 받기
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut input_iter = input
        .split_whitespace()
        .map(|i| i.trim().parse::<i32>().unwrap());
    let n: i32 = input_iter.next().unwrap();
    let m: i32 = input_iter.next().unwrap();
    
    let mut bucket: Vec<i32> = (1..=n).collect();
    
    for _ in 0..m {
        // i, j 입력 받기
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut line_iter = line
            .split_whitespace()
            .map(|i| i.trim().parse::<usize>().unwrap());
        let i: usize = line_iter.next().unwrap();
        let j: usize = line_iter.next().unwrap();
        
        // i ~ j 번째 바구니 뒤집기
        bucket = reverse_bucket(&mut bucket, i-1, j-1);
    }
    
    for item in bucket {
        print!("{} ", item);
    }
}

fn reverse_bucket(bucket: &mut Vec<i32>,
    start_index: usize,
    end_index: usize
) -> Vec<i32> {
    let head_bucket: Vec<i32> = bucket[0..start_index].to_vec();
    let mid_bucket: Vec<i32> = bucket[start_index..=end_index]
        .to_vec()
        .clone()
        .into_iter()
        .rev()
        .collect();
    let tail_bucket: Vec<i32> = bucket[end_index+1..].to_vec();
    let result = [head_bucket, mid_bucket, tail_bucket].concat();
    result
    // println!("{:?}", result);
}