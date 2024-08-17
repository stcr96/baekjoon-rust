// 2587번, 대표값2

use std::io;

fn main() {
    let mut array: Vec<i32> = Vec::new();
    for _ in 0..5 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().parse::<i32>().unwrap();
        array.push(input);
    }
    let average = average(&mut array);
    println!("{}", average);
    let median = median(&mut array);
    println!("{}", median);
}

fn average(array: &mut Vec<i32>) -> i32 {
    array.iter().sum::<i32>() / array.len() as i32
}

fn median(array: &mut Vec<i32>) -> i32 {
    array.sort();
    array[array.len()/2]
}