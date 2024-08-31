// 2563번, 색종이

use std::io;

fn main() {
    let mut paper = vec![vec![0; 100]; 100];

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let coords: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let x = coords[0];
        let y = coords[1];

        // 색종이 붙이기
        for i in x..x + 10 {
            for j in y..y + 10 {
                paper[i][j] = 1;
            }
        }
    }

    // 검은색 영역의 넓이 계산
    let mut black_area = 0;
    for i in 0..100 {
        for j in 0..100 {
            if paper[i][j] == 1 {
                black_area += 1;
            }
        }
    }

    println!("{}", black_area);
}
