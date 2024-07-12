use std::io;

fn main() {
    let mut input = String::new();
    
    for _ in 0..10 {
        io::stdin().read_line(&mut input).unwrap();
    }
    
    let input: Vec<&str> = input.split_whitespace().collect();
    let mut number: Vec<u32> = Vec::new();
    
    for i in 0..10 {
        let num: u32 = input[i].trim().parse().unwrap();
        number.push(num%42);
    }
    
    number.sort();
    number.dedup();
    
    println!("{}", number.len());
}