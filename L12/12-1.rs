// 2798번, 블랙잭

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut input_iter = input
        .split_whitespace()
        .map(|i| i.trim().parse::<usize>().unwrap());
    let n = input_iter.next().unwrap();
    let m = input_iter.next().unwrap();
    input.clear();
    
    io::stdin().read_line(&mut input).unwrap();
    let input: Vec<usize> = input
        .split_whitespace()
        .map(|i| i.trim().parse::<usize>().unwrap())
        .collect();
    
    let mut sum_vec: Vec<usize> = Vec::new();
    let mut sum;
    for x in 0..n {
        for y in x+1..n {
            for z in y+1..n {
                sum = input[x] + input[y] + input[z];
                if sum > m { continue }
                else { sum_vec.push(sum); }
            }
        }
    }
    println!("{}", sum_vec.iter().max().unwrap());
}