// 10814번, 나이순 정렬

use std::io;

#[derive(Debug)]
struct Member {
    age: i32,
    name: String,
}

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<i32>().unwrap();
    
    let mut members: Vec<Member> = Vec::new();
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: Vec<&str> = input.split_whitespace().collect();
        let age = input[0].trim().parse::<i32>().unwrap();
        let name = input[1].trim().to_string();
        let member = Member {
            age,
            name,
        };
        members.push(member);
    }
    members.sort_by_key(|k| k.age);
    
    for member in &members {
        println!("{} {}", member.age, member.name);
    } 
}