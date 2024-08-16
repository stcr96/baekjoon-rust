// 25206번, 너의 평점은

use std::io;

#[derive(Debug)]
struct Subject {
    credit: f64,
    grade_num: f64,
}

fn main() {
    let mut credit_sum: f64 = 0.0;
    let mut multiply_sum: f64 = 0.0;
    for _ in 0..20 {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let line = line.split_whitespace().collect::<Vec<&str>>();
        
        let credit = line[1].trim().parse::<f64>().unwrap();
        let grade_str = line[2].trim();
        let grade_num: f64 = match grade_str {
            "A+" => 4.5,
            "A0" => 4.0,
            "B+" => 3.5,
            "B0" => 3.0,
            "C+" => 2.5,
            "C0" => 2.0,
            "D+" => 1.5,
            "D0" => 1.0,
            "F" => 0.0,
            _ => continue,
        };
        let subject = Subject {
            credit,
            grade_num,
        };
        let multiply: f64 = subject.multiply();
        credit_sum += credit;
        multiply_sum += multiply;
    }
    println!("{}", multiply_sum / credit_sum);
}

impl Subject {
    fn multiply(&self) -> f64 {
        self.credit * self.grade_num
    }
}