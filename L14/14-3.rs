// 7785번, 회사에 있는 사람

use std::io::{self, BufReader, BufRead, BufWriter, Write};
use std::collections::HashSet;

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    
    let mut n = String::new();
    reader.read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();
    
    let mut line = String::new();
    let mut employees = HashSet::new();
    for _ in 0..n {
        line.clear();
        reader.read_line(&mut line).unwrap();
        let mut line_iter = line
            .trim()
            .split_whitespace();
        let name = line_iter.next().unwrap();
        let attendance = line_iter.next().unwrap();
        
        if attendance == "enter" {
            employees.insert(name.to_string());
        } else {
            employees.remove(name);
        }
    }
    
    let mut sorted_employees: Vec<String> = employees.into_iter().collect();
    sorted_employees.sort_by(|a, b| b.cmp(a));
    for employee in sorted_employees  {
        writeln!(writer, "{}", employee).unwrap();
    }
}